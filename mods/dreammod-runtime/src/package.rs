use std::{collections::BTreeMap, fs::File, io::Read, path::Path, sync::Arc};

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
            if let Some(mode) = file.unix_mode() {
                if mode & 0o170000 == 0o120000 {
                    return Err(PackageError::Symlink(name));
                }
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

    use tempfile::NamedTempFile;
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
            ("scripts/main.dream", &vec![b'a'; 128]),
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
