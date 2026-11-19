use std::{collections::BTreeMap, fs::{self, File}, io::Read, path::{Path, PathBuf}, sync::Arc};

use sha2::{Digest, Sha256};
use thiserror::Error;
use zip::ZipArchive;

use crate::manifest::{ManifestError, ModManifest, is_safe_relative_path};

#[derive(Clone, Copy, Debug)]
pub struct PackageLimits {
    pub max_archive_bytes: u64,
    pub max_files: usize,
    pub max_total_uncompressed_bytes: u64,
    pub max_file_bytes: u64,
    pub max_script_bytes: u64,
    pub max_compression_ratio: u32,
}

impl Default for PackageLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 32 * 1024 * 1024,
            max_files: 256,
            max_total_uncompressed_bytes: 64 * 1024 * 1024,
            max_file_bytes: 8 * 1024 * 1024,
            max_script_bytes: 2 * 1024 * 1024,
            max_compression_ratio: 100,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DmodPackage {
    pub manifest: ModManifest,
    pub entry_source: String,
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub sha256: String,
}

#[derive(Clone, Copy, Debug)]
pub struct PackageLoader {
    limits: PackageLimits,
}

impl Default for PackageLoader {
    fn default() -> Self {
        Self::new(PackageLimits::default())
    }
}

impl PackageLoader {
    pub fn new(limits: PackageLimits) -> Self {
        Self { limits }
    }

    pub fn load(&self, path: impl AsRef<Path>) -> Result<DmodPackage, PackageError> {
        let path = path.as_ref();
        let metadata = path.metadata()?;
        if !metadata.is_file() {
            return Err(PackageError::NotAFile);
        }
        if metadata.len() > self.limits.max_archive_bytes {
            return Err(PackageError::ArchiveTooLarge {
                actual: metadata.len(),
                limit: self.limits.max_archive_bytes,
            });
        }

        let archive_bytes = std::fs::read(path)?;
        let sha256 = format!("{:x}", Sha256::digest(&archive_bytes));
        let mut archive = ZipArchive::new(File::open(path)?)?;
        if archive.len() > self.limits.max_files {
            return Err(PackageError::TooManyFiles {
                actual: archive.len(),
                limit: self.limits.max_files,
            });
        }

        let mut files = BTreeMap::new();
        let mut total_uncompressed = 0_u64;

        for index in 0..archive.len() {
            let mut file = archive.by_index(index)?;
            let name = file.name().to_owned();

            if !is_safe_relative_path(&name) || file.enclosed_name().is_none() {
                return Err(PackageError::UnsafePath(name));
            }
            if let Some(mode) = file.unix_mode()
                && mode & 0o170000 == 0o120000
            {
                return Err(PackageError::Symlink(name));
            }
            if file.is_dir() {
                continue;
            }
            if is_forbidden_executable(&name) {
                return Err(PackageError::ExecutableContent(name));
            }

            let size = file.size();
            let compressed = file.compressed_size();
            if size > self.limits.max_file_bytes {
                return Err(PackageError::FileTooLarge {
                    path: name,
                    actual: size,
                    limit: self.limits.max_file_bytes,
                });
            }
            if size > 0
                && (compressed == 0
                    || size > compressed.saturating_mul(self.limits.max_compression_ratio as u64))
            {
                return Err(PackageError::SuspiciousCompression(name));
            }

            total_uncompressed = total_uncompressed
                .checked_add(size)
                .ok_or(PackageError::TotalSizeOverflow)?;
            if total_uncompressed > self.limits.max_total_uncompressed_bytes {
                return Err(PackageError::TotalTooLarge {
                    actual: total_uncompressed,
                    limit: self.limits.max_total_uncompressed_bytes,
                });
            }

            if files.contains_key(&name) {
                return Err(PackageError::DuplicatePath(name));
            }

            let mut bytes = Vec::with_capacity(size.min(usize::MAX as u64) as usize);
            file.by_ref()
                .take(self.limits.max_file_bytes.saturating_add(1))
                .read_to_end(&mut bytes)?;
            if bytes.len() as u64 > self.limits.max_file_bytes {
                return Err(PackageError::FileTooLarge {
                    path: name,
                    actual: bytes.len() as u64,
                    limit: self.limits.max_file_bytes,
                });
            }

            files.insert(name, Arc::<[u8]>::from(bytes));
        }

        let manifest_bytes = files
            .get("manifest.json")
            .ok_or(PackageError::MissingManifest)?;
        let manifest: ModManifest = serde_json::from_slice(manifest_bytes)
            .map_err(|error| PackageError::InvalidManifest(error.to_string()))?;
        manifest.validate()?;

        let entry = files
            .get(&manifest.entry)
            .ok_or_else(|| PackageError::MissingEntry(manifest.entry.clone()))?;
        if entry.len() as u64 > self.limits.max_script_bytes {
            return Err(PackageError::ScriptTooLarge {
                actual: entry.len() as u64,
                limit: self.limits.max_script_bytes,
            });
        }
        let entry_source = String::from_utf8(entry.to_vec())
            .map_err(|_| PackageError::ScriptNotUtf8(manifest.entry.clone()))?;

        Ok(DmodPackage {
            manifest,
            entry_source,
            files,
            sha256,
        })
    }
}


/// Limits for discovery within Horizon's shared `horizon-mods/` folder.
#[derive(Clone, Copy, Debug)]
pub struct ModsDirectoryLimits {
    /// Maximum candidate .dmod files to inspect in one directory.
    pub max_packages: usize,
    /// Aggregate archive bytes allowed before loading packages.
    pub max_total_archive_bytes: u64,
    /// Aggregate retained uncompressed package contents.
    pub max_total_retained_bytes: u64,
}

impl Default for ModsDirectoryLimits {
    fn default() -> Self {
        Self {
            max_packages: 256,
            max_total_archive_bytes: 256 * 1024 * 1024,
            max_total_retained_bytes: 512 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiscoveredDmod {
    pub path: PathBuf,
    pub package: DmodPackage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanIssue {
    pub path: PathBuf,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct ModsDirectoryScan {
    /// Packages are sorted by filename, with duplicate package IDs excluded.
    pub packages: Vec<DiscoveredDmod>,
    pub issues: Vec<ScanIssue>,
    /// Count of .hmod files deliberately ignored for Horizon's own loader.
    pub ignored_hmod_count: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct ModsDirectoryScanner {
    package_loader: PackageLoader,
    limits: ModsDirectoryLimits,
}

impl Default for ModsDirectoryScanner {
    fn default() -> Self {
        Self::new(PackageLoader::default(), ModsDirectoryLimits::default())
    }
}

impl ModsDirectoryScanner {
    pub fn new(package_loader: PackageLoader, limits: ModsDirectoryLimits) -> Self {
        Self {
            package_loader,
            limits,
        }
    }

    /// Discover player DreamMod packages directly in Horizon's mods folder.
    ///
    /// Only direct child files with a .dmod extension are considered. Horizon
    /// .hmod files are counted and ignored; other files and directories are
    /// ignored. Symlinked .dmod entries are reported, never followed.
    pub fn scan(
        &self,
        directory: impl AsRef<Path>,
    ) -> Result<ModsDirectoryScan, ScanDirectoryError> {
        let directory = directory.as_ref();
        let metadata = fs::metadata(directory)?;
        if !metadata.is_dir() {
            return Err(ScanDirectoryError::NotDirectory);
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_text = name.to_string_lossy();
            let extension = Path::new(name_text.as_ref())
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if extension != "dmod" && extension != "hmod" {
                continue;
            }
            entries.push((name_text.into_owned(), entry.path(), entry.file_type()?));
        }
        entries.sort_by(|left, right| left.0.cmp(&right.0));

        let candidates = entries
            .iter()
            .filter(|(name, _, _)| {
                Path::new(name)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("dmod"))
            })
            .count();
        if candidates > self.limits.max_packages {
            return Err(ScanDirectoryError::TooManyPackages {
                actual: candidates,
                limit: self.limits.max_packages,
            });
        }

        let mut result = ModsDirectoryScan::default();
        let mut total_archive_bytes = 0_u64;
        let mut total_retained_bytes = 0_u64;
        for (name, path, file_type) in entries {
            let extension = Path::new(&name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if extension == "hmod" {
                result.ignored_hmod_count += 1;
                continue;
            }
            if file_type.is_symlink() {
                result.issues.push(ScanIssue {
                    path,
                    reason: "symbolic link is not followed".into(),
                });
                continue;
            }
            if !file_type.is_file() {
                result.issues.push(ScanIssue {
                    path,
                    reason: "candidate is not a regular file".into(),
                });
                continue;
            }

            let metadata = fs::metadata(&path)?;
            total_archive_bytes = total_archive_bytes
                .checked_add(metadata.len())
                .ok_or(ScanDirectoryError::TotalSizeOverflow)?;
            if total_archive_bytes > self.limits.max_total_archive_bytes {
                return Err(ScanDirectoryError::TotalArchiveBytesExceeded {
                    actual: total_archive_bytes,
                    limit: self.limits.max_total_archive_bytes,
                });
            }

            match self.package_loader.load(&path) {
                Ok(package) => {
                    let package_bytes = package.files.values().try_fold(0_u64, |sum, file| {
                        sum.checked_add(file.len() as u64)
                    }).ok_or(ScanDirectoryError::TotalSizeOverflow)?;
                    total_retained_bytes = total_retained_bytes
                        .checked_add(package_bytes)
                        .ok_or(ScanDirectoryError::TotalSizeOverflow)?;
                    if total_retained_bytes > self.limits.max_total_retained_bytes {
                        return Err(ScanDirectoryError::TotalRetainedBytesExceeded {
                            actual: total_retained_bytes,
                            limit: self.limits.max_total_retained_bytes,
                        });
                    }
                    result.packages.push(DiscoveredDmod { path, package });
                }
                Err(error) => result.issues.push(ScanIssue {
                    path,
                    reason: error.to_string(),
                }),
            }
        }

        // Reject every package sharing a mod ID; choosing one based on order is unsafe.
        let mut id_paths: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
        for package in &result.packages {
            id_paths
                .entry(package.package.manifest.id.to_string())
                .or_default()
                .push(package.path.clone());
        }
        let duplicates: BTreeMap<String, Vec<PathBuf>> = id_paths
            .into_iter()
            .filter(|(_, paths)| paths.len() > 1)
            .collect();
        if !duplicates.is_empty() {
            let packages = std::mem::take(&mut result.packages);
            for package in packages {
                if let Some(paths) = duplicates.get(package.package.manifest.id.as_str()) {
                    let other_paths = paths
                        .iter()
                        .filter(|other| other.as_path() != package.path.as_path())
                        .map(|other| other.display().to_string())
                        .collect::<Vec<_>>();
                    result.issues.push(ScanIssue {
                        path: package.path.clone(),
                        reason: format!(
                            "duplicate mod ID '{}' also found in: {}",
                            package.package.manifest.id,
                            other_paths.join(", ")
                        ),
                    });
                } else {
                    result.packages.push(package);
                }
            }
        }

        result.packages.sort_by(|a, b| a.path.cmp(&b.path));
        result.issues.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanDirectoryError {
    #[error("mods directory is not a directory")]
    NotDirectory,
    #[error("too many .dmod packages: {actual} (limit {limit})")]
    TooManyPackages { actual: usize, limit: usize },
    #[error("aggregate .dmod archive size exceeds limit: {actual} bytes (limit {limit})")]
    TotalArchiveBytesExceeded { actual: u64, limit: u64 },
    #[error("aggregate retained package contents exceed limit: {actual} bytes (limit {limit})")]
    TotalRetainedBytesExceeded { actual: u64, limit: u64 },
    #[error("aggregate archive byte count overflow")]
    TotalSizeOverflow,
    #[error("directory I/O error: {0}")]
    Io(#[from] std::io::Error),
}

fn is_forbidden_executable(path: &str) -> bool {
    let extension = Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    matches!(
        extension.as_str(),
        "so" | "dll"
            | "dylib"
            | "exe"
            | "wasm"
            | "o"
            | "rlib"
            | "a"
            | "class"
            | "jar"
            | "pyc"
            | "luac"
            | "bc"
    )
}

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("package path is not a regular file")]
    NotAFile,
    #[error("package archive is too large: {actual} bytes (limit {limit})")]
    ArchiveTooLarge { actual: u64, limit: u64 },
    #[error("package contains too many entries: {actual} (limit {limit})")]
    TooManyFiles { actual: usize, limit: usize },
    #[error("unsafe archive entry path: {0}")]
    UnsafePath(String),
    #[error("symbolic links are not allowed in .dmod packages: {0}")]
    Symlink(String),
    #[error("native/compiled executable content is forbidden in .dmod: {0}")]
    ExecutableContent(String),
    #[error("package contains duplicate entry path: {0}")]
    DuplicatePath(String),
    #[error("package file is too large: {path} ({actual} bytes, limit {limit})")]
    FileTooLarge {
        path: String,
        actual: u64,
        limit: u64,
    },
    #[error("suspicious compression ratio in entry: {0}")]
    SuspiciousCompression(String),
    #[error("package total uncompressed size overflow")]
    TotalSizeOverflow,
    #[error("package expands beyond total size limit: {actual} bytes (limit {limit})")]
    TotalTooLarge { actual: u64, limit: u64 },
    #[error("package has no root manifest.json")]
    MissingManifest,
    #[error("manifest is invalid: {0}")]
    InvalidManifest(String),
    #[error("manifest validation failed: {0}")]
    Manifest(#[from] ManifestError),
    #[error("manifest entry script is missing: {0}")]
    MissingEntry(String),
    #[error("entry script exceeds limit: {actual} bytes (limit {limit})")]
    ScriptTooLarge { actual: u64, limit: u64 },
    #[error("entry script is not valid UTF-8: {0}")]
    ScriptNotUtf8(String),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::{NamedTempFile, TempDir};
    use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

    use super::*;

    fn write_package(entries: &[(&str, &[u8])]) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        let output = File::create(file.path()).unwrap();
        let mut writer = ZipWriter::new(output);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

        for (name, contents) in entries {
            writer.start_file(*name, options).unwrap();
            writer.write_all(contents).unwrap();
        }

        writer.finish().unwrap();
        file
    }

    fn valid_manifest() -> &'static [u8] {
        br#"{
            "schema_version": 1,
            "id": "dreammod.example",
            "version": "1.0.0",
            "display_name": "Example Mod",
            "dreammod_api": "^0.1.0",
            "entry": "scripts/main.dream",
            "side": "both",
            "events": ["server_tick"]
        }"#
    }

    fn create_dmod(dir: &Path, filename: &str, mod_id: &str) -> PathBuf {
        let path = dir.join(filename);
        let output = File::create(&path).unwrap();
        let mut writer = ZipWriter::new(output);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        let manifest = format!(r#"{{
            "schema_version":1,
            "id":"{mod_id}",
            "version":"1.0.0",
            "display_name":"Example",
            "dreammod_api":"^0.1.0",
            "entry":"scripts/main.dream",
            "side":"both",
            "events":["server_tick"]
        }}"#);
        writer.start_file("manifest.json", options).unwrap();
        writer.write_all(manifest.as_bytes()).unwrap();
        writer.start_file("scripts/main.dream", options).unwrap();
        writer.write_all(b"fn on_server_tick(event) { #{ commands: [] } }").unwrap();
        writer.finish().unwrap();
        path
    }

    #[test]
    fn scanner_accepts_dmod_and_ignores_horizon_hmod_and_other_files() {
        let dir = TempDir::new().unwrap();
        create_dmod(dir.path(), "player.dmod", "dreammod.player");
        std::fs::write(dir.path().join("horizon-native.hmod"), b"not inspected").unwrap();
        std::fs::write(dir.path().join("readme.txt"), b"ignored").unwrap();
        std::fs::create_dir(dir.path().join("nested.dmod")).unwrap();

        let scan = ModsDirectoryScanner::default().scan(dir.path()).unwrap();
        assert_eq!(scan.packages.len(), 1);
        assert_eq!(scan.packages[0].package.manifest.id.as_str(), "dreammod.player");
        assert_eq!(scan.ignored_hmod_count, 1);
        assert_eq!(scan.issues.len(), 1);
        assert!(scan.issues[0].reason.contains("regular file"));
    }

    #[test]
    fn scanner_rejects_all_packages_with_duplicate_ids() {
        let dir = TempDir::new().unwrap();
        create_dmod(dir.path(), "a.dmod", "dreammod.same");
        create_dmod(dir.path(), "b.dmod", "dreammod.same");

        let scan = ModsDirectoryScanner::default().scan(dir.path()).unwrap();
        assert!(scan.packages.is_empty());
        assert_eq!(scan.issues.len(), 2);
        assert!(scan.issues.iter().all(|issue| issue.reason.contains("duplicate mod ID")));
    }

    #[test]
    fn scanner_enforces_candidate_count_and_total_size() {
        let dir = TempDir::new().unwrap();
        create_dmod(dir.path(), "one.dmod", "dreammod.one");
        create_dmod(dir.path(), "two.dmod", "dreammod.two");
        let count_limited = ModsDirectoryScanner::new(PackageLoader::default(), ModsDirectoryLimits {
            max_packages: 1, max_total_archive_bytes: 1024,
        });
        assert!(matches!(count_limited.scan(dir.path()), Err(ScanDirectoryError::TooManyPackages { .. })));

        let size_limited = ModsDirectoryScanner::new(PackageLoader::default(), ModsDirectoryLimits {
            max_packages: 10, max_total_archive_bytes: 1024, max_total_retained_bytes: 1,
        });
        assert!(matches!(size_limited.scan(dir.path()), Err(ScanDirectoryError::TotalRetainedBytesExceeded { .. })));
    }

    #[cfg(unix)]
    #[test]
    fn scanner_reports_symlinked_dmod_without_following_it() {
        use std::os::unix::fs::symlink;
        let dir = TempDir::new().unwrap();
        let outside = TempDir::new().unwrap();
        create_dmod(outside.path(), "target.dmod", "dreammod.external");
        symlink(outside.path().join("target.dmod"), dir.path().join("linked.dmod")).unwrap();

        let scan = ModsDirectoryScanner::default().scan(dir.path()).unwrap();
        assert!(scan.packages.is_empty());
        assert_eq!(scan.issues.len(), 1);
        assert!(scan.issues[0].reason.contains("symbolic link"));
    }

    #[test]
    fn loads_source_package_and_hashes_archive() {
        let file = write_package(&[
            ("manifest.json", valid_manifest()),
            (
                "scripts/main.dream",
                b"fn on_server_tick(event) { #{ commands: [] } }",
            ),
            ("assets/readme.txt", b"example"),
        ]);

        let package = PackageLoader::default().load(file.path()).unwrap();
        assert_eq!(package.manifest.id.as_str(), "dreammod.example");
        assert!(package.entry_source.contains("on_server_tick"));
        assert_eq!(package.files.len(), 3);
        assert_eq!(package.sha256.len(), 64);
    }

    #[test]
    fn rejects_path_traversal() {
        let file = write_package(&[
            ("manifest.json", valid_manifest()),
            ("scripts/main.dream", b""),
            ("../outside.txt", b"escape"),
        ]);

        assert!(matches!(
            PackageLoader::default().load(file.path()),
            Err(PackageError::UnsafePath(_))
        ));
    }

    #[test]
    fn rejects_compiled_code_inside_source_mods() {
        let file = write_package(&[
            ("manifest.json", valid_manifest()),
            ("scripts/main.dream", b""),
            ("native/libcheat.so", b"not a library"),
        ]);

        assert!(matches!(
            PackageLoader::default().load(file.path()),
            Err(PackageError::ExecutableContent(_))
        ));
    }

    #[test]
    fn rejects_oversized_script() {
        let file = write_package(&[
            ("manifest.json", valid_manifest()),
            ("scripts/main.dream", &[b'a'; 128]),
        ]);
        let loader = PackageLoader::new(PackageLimits {
            max_script_bytes: 32,
            ..PackageLimits::default()
        });

        assert!(matches!(
            loader.load(file.path()),
            Err(PackageError::ScriptTooLarge { .. })
        ));
    }
}
