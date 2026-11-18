//! Strict, side-effect-free validation for the first DreamMod manifest contract.
//! This crate does not open archives, extract files, grant capabilities or execute scripts.

use semver::Version;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

pub const MANIFEST_SCHEMA_VERSION: u32 = 1;
pub const MAX_MANIFEST_BYTES: usize = 64 * 1024;
pub const MAX_DEPENDENCIES: usize = 128;
pub const MAX_CAPABILITIES: usize = 64;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub version: String,
    pub api: ApiCompatibility,
    pub entrypoint: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ApiCompatibility {
    pub min: String,
    pub max: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestError {
    TooLarge,
    InvalidJson(String),
    UnsupportedSchema(u32),
    InvalidField(&'static str),
    InvalidVersion(&'static str),
    InvalidApiRange,
    UnknownCapability(String),
    DuplicateCapability(String),
    TooManyCapabilities,
    TooManyDependencies,
    InvalidDependencyId(String),
    InvalidDependencyRange(String),
    InvalidEntrypoint,
}

const ALLOWED_CAPABILITIES: &[&str] = &[
    "assets.read",
    "entities.read",
    "entities.write",
    "events.emit",
    "events.listen",
    "ui.register",
    "world.read",
    "world.write",
];

pub fn parse_manifest(input: &[u8]) -> Result<Manifest, ManifestError> {
    if input.len() > MAX_MANIFEST_BYTES {
        return Err(ManifestError::TooLarge);
    }
    let manifest: Manifest = serde_json::from_slice(input)
        .map_err(|error| ManifestError::InvalidJson(error.to_string()))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

pub fn validate_manifest(manifest: &Manifest) -> Result<(), ManifestError> {
    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(ManifestError::UnsupportedSchema(manifest.schema_version));
    }
    if !valid_mod_id(&manifest.id) {
        return Err(ManifestError::InvalidField("id"));
    }
    if manifest.name.trim().is_empty()
        || manifest.name.len() > 128
        || manifest.name.chars().any(char::is_control)
    {
        return Err(ManifestError::InvalidField("name"));
    }
    parse_version(&manifest.version, "version")?;
    let min = parse_version(&manifest.api.min, "api.min")?;
    let max = parse_version(&manifest.api.max, "api.max")?;
    if min > max {
        return Err(ManifestError::InvalidApiRange);
    }
    if !valid_relative_script_path(&manifest.entrypoint) {
        return Err(ManifestError::InvalidEntrypoint);
    }
    if manifest.capabilities.len() > MAX_CAPABILITIES {
        return Err(ManifestError::TooManyCapabilities);
    }
    let mut seen = BTreeSet::new();
    for capability in &manifest.capabilities {
        if !ALLOWED_CAPABILITIES.contains(&capability.as_str()) {
            return Err(ManifestError::UnknownCapability(capability.clone()));
        }
        if !seen.insert(capability) {
            return Err(ManifestError::DuplicateCapability(capability.clone()));
        }
    }
    if manifest.dependencies.len() > MAX_DEPENDENCIES {
        return Err(ManifestError::TooManyDependencies);
    }
    for (id, range) in &manifest.dependencies {
        if !valid_mod_id(id) || id == &manifest.id {
            return Err(ManifestError::InvalidDependencyId(id.clone()));
        }
        if semver::VersionReq::parse(range).is_err() {
            return Err(ManifestError::InvalidDependencyRange(id.clone()));
        }
    }
    Ok(())
}

fn parse_version(value: &str, field: &'static str) -> Result<Version, ManifestError> {
    Version::parse(value).map_err(|_| ManifestError::InvalidVersion(field))
}

fn valid_mod_id(value: &str) -> bool {
    let parts: Vec<&str> = value.split('.').collect();
    parts.len() >= 2
        && parts.iter().all(|part| {
            !part.is_empty()
                && !part.starts_with('-')
                && !part.ends_with('-')
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
}

fn valid_relative_script_path(path: &str) -> bool {
    if path.is_empty()
        || path.len() > 240
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains(':')
        || path.chars().any(char::is_control)
    {
        return false;
    }
    let mut parts = path.split('/');
    let Some(first) = parts.next() else {
        return false;
    };
    first == "scripts"
        && parts.all(|part| !part.is_empty() && part != "." && part != "..")
        && path.ends_with(".dream")
}

/// Limits applied to ZIP central-directory metadata before any entry is opened.
pub const MAX_ARCHIVE_ENTRIES: usize = 1_024;
pub const MAX_ARCHIVE_UNCOMPRESSED_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_ARCHIVE_FILE_BYTES: u64 = 16 * 1024 * 1024;
pub const MAX_COMPRESSION_RATIO: u64 = 100;
pub const MAX_ARCHIVE_PATH_BYTES: usize = 240;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveEntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

/// Metadata supplied by a ZIP central-directory reader. This validator never reads
/// entry contents or extracts files; callers must validate before opening any entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveEntry {
    pub path: String,
    pub kind: ArchiveEntryKind,
    pub compressed_bytes: u64,
    pub uncompressed_bytes: u64,
    pub encrypted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    TooManyEntries,
    UnsafePath(String),
    DuplicatePath(String),
    UnsupportedEntryType(String),
    EncryptedEntry(String),
    UnexpectedPath(String),
    FileTooLarge(String),
    ArchiveTooLarge,
    SuspiciousCompressionRatio(String),
    MissingManifest,
    MultipleManifests,
    MissingEntrypoint(String),
    InvalidManifestEntrySize,
}

/// Validate untrusted ZIP metadata before opening or extracting any archive member.
/// The ZIP reader must separately reject malformed/overlapping local-file records.
pub fn validate_archive_inventory(
    entries: &[ArchiveEntry],
    manifest: &Manifest,
) -> Result<(), ArchiveError> {
    if entries.len() > MAX_ARCHIVE_ENTRIES {
        return Err(ArchiveError::TooManyEntries);
    }

    let mut seen = BTreeSet::new();
    let mut total_uncompressed = 0_u64;
    let mut manifest_count = 0_usize;
    let mut entrypoint_found = false;

    for entry in entries {
        if !valid_archive_path(&entry.path, entry.kind) {
            return Err(ArchiveError::UnsafePath(entry.path.clone()));
        }
        let key = entry.path.trim_end_matches('/').to_ascii_lowercase();
        if !seen.insert(key) {
            return Err(ArchiveError::DuplicatePath(entry.path.clone()));
        }
        if matches!(
            entry.kind,
            ArchiveEntryKind::Symlink | ArchiveEntryKind::Other
        ) {
            return Err(ArchiveError::UnsupportedEntryType(entry.path.clone()));
        }
        if entry.encrypted {
            return Err(ArchiveError::EncryptedEntry(entry.path.clone()));
        }
        if entry.kind == ArchiveEntryKind::Directory {
            if entry.uncompressed_bytes != 0
                || entry.compressed_bytes != 0
                || !(entry.path.starts_with("scripts/") || entry.path.starts_with("assets/"))
            {
                return Err(ArchiveError::UnsafePath(entry.path.clone()));
            }
            continue;
        }

        if entry.uncompressed_bytes > MAX_ARCHIVE_FILE_BYTES {
            return Err(ArchiveError::FileTooLarge(entry.path.clone()));
        }
        total_uncompressed = total_uncompressed
            .checked_add(entry.uncompressed_bytes)
            .ok_or(ArchiveError::ArchiveTooLarge)?;
        if total_uncompressed > MAX_ARCHIVE_UNCOMPRESSED_BYTES {
            return Err(ArchiveError::ArchiveTooLarge);
        }
        if entry.uncompressed_bytes > 0
            && (entry.compressed_bytes == 0
                || entry.uncompressed_bytes
                    > entry.compressed_bytes.saturating_mul(MAX_COMPRESSION_RATIO))
        {
            return Err(ArchiveError::SuspiciousCompressionRatio(entry.path.clone()));
        }

        if entry.path == "manifest.json" {
            manifest_count += 1;
            if entry.uncompressed_bytes > MAX_MANIFEST_BYTES as u64 {
                return Err(ArchiveError::InvalidManifestEntrySize);
            }
        } else if entry.path.starts_with("scripts/") {
            if !valid_relative_script_path(&entry.path) {
                return Err(ArchiveError::UnsafePath(entry.path.clone()));
            }
            if entry.path == manifest.entrypoint {
                entrypoint_found = true;
            }
        } else if !entry.path.starts_with("assets/") {
            return Err(ArchiveError::UnexpectedPath(entry.path.clone()));
        }
    }

    match manifest_count {
        0 => return Err(ArchiveError::MissingManifest),
        1 => {}
        _ => return Err(ArchiveError::MultipleManifests),
    }
    if !entrypoint_found {
        return Err(ArchiveError::MissingEntrypoint(manifest.entrypoint.clone()));
    }
    Ok(())
}

fn valid_archive_path(path: &str, kind: ArchiveEntryKind) -> bool {
    let normalized = if kind == ArchiveEntryKind::Directory {
        path.strip_suffix('/').unwrap_or(path)
    } else {
        path
    };
    if normalized.is_empty()
        || path.len() > MAX_ARCHIVE_PATH_BYTES
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains(':')
        || path.chars().any(char::is_control)
        || normalized.split('/').any(|part| {
            part.is_empty()
                || part == "."
                || part == ".."
                || part.starts_with('.')
                || !part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        })
    {
        return false;
    }
    true
}


/// Maximum number of installed manifests considered in one dependency plan.
pub const MAX_INSTALLED_MODS: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyError {
    TooManyMods,
    InvalidManifest(String),
    DuplicateModId(String),
    MissingDependency { mod_id: String, dependency: String },
    VersionMismatch { mod_id: String, dependency: String },
    DependencyCycle(Vec<String>),
}

/// Build a deterministic load order. Dependencies always precede dependents;
/// among currently-ready mods, IDs are ordered lexicographically.
pub fn resolve_load_order(manifests: &[Manifest]) -> Result<Vec<String>, DependencyError> {
    if manifests.len() > MAX_INSTALLED_MODS {
        return Err(DependencyError::TooManyMods);
    }

    let mut by_id = BTreeMap::<String, &Manifest>::new();
    for manifest in manifests {
        validate_manifest(manifest)
            .map_err(|_| DependencyError::InvalidManifest(manifest.id.clone()))?;
        if by_id.insert(manifest.id.clone(), manifest).is_some() {
            return Err(DependencyError::DuplicateModId(manifest.id.clone()));
        }
    }

    let mut indegree = BTreeMap::<String, usize>::new();
    let mut dependents = BTreeMap::<String, BTreeSet<String>>::new();

    for (id, manifest) in &by_id {
        indegree.insert(id.clone(), manifest.dependencies.len());
        for (dependency_id, requirement) in &manifest.dependencies {
            let dependency = by_id.get(dependency_id).ok_or_else(|| {
                DependencyError::MissingDependency {
                    mod_id: id.clone(),
                    dependency: dependency_id.clone(),
                }
            })?;
            let required = semver::VersionReq::parse(requirement)
                .map_err(|_| DependencyError::InvalidManifest(id.clone()))?;
            let actual = Version::parse(&dependency.version)
                .map_err(|_| DependencyError::InvalidManifest(dependency_id.clone()))?;
            if !required.matches(&actual) {
                return Err(DependencyError::VersionMismatch {
                    mod_id: id.clone(),
                    dependency: dependency_id.clone(),
                });
            }
            dependents
                .entry(dependency_id.clone())
                .or_default()
                .insert(id.clone());
        }
    }

    let mut ready = BTreeSet::<String>::new();
    for (id, count) in &indegree {
        if *count == 0 {
            ready.insert(id.clone());
        }
    }

    let mut order = Vec::with_capacity(manifests.len());
    while let Some(id) = ready.iter().next().cloned() {
        ready.remove(&id);
        order.push(id.clone());

        if let Some(children) = dependents.get(&id) {
            for child in children {
                let count = indegree
                    .get_mut(child)
                    .expect("dependent must have an indegree entry");
                *count -= 1;
                if *count == 0 {
                    ready.insert(child.clone());
                }
            }
        }
    }

    if order.len() != manifests.len() {
        let cycle = indegree
            .iter()
            .filter_map(|(id, count)| (*count > 0).then_some(id.clone()))
            .collect();
        return Err(DependencyError::DependencyCycle(cycle));
    }

    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"{
        "schema_version": 1,
        "id": "example.weather",
        "name": "Weather Example",
        "version": "1.2.3",
        "api": { "min": "0.2.0", "max": "0.3.0" },
        "entrypoint": "scripts/main.dream",
        "capabilities": ["events.listen", "world.read"],
        "dependencies": { "example.base": "^1.0.0" }
    }"#;

    #[test]
    fn accepts_a_valid_manifest() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        assert_eq!(manifest.id, "example.weather");
        assert_eq!(manifest.entrypoint, "scripts/main.dream");
    }

    #[test]
    fn rejects_unknown_schema_and_unknown_fields() {
        let input = VALID.replace("\"schema_version\": 1", "\"schema_version\": 2");
        assert_eq!(
            parse_manifest(input.as_bytes()).unwrap_err(),
            ManifestError::UnsupportedSchema(2)
        );
        let input = VALID.replace("\"name\":", "\"unexpected\": true, \"name\":");
        assert!(matches!(
            parse_manifest(input.as_bytes()),
            Err(ManifestError::InvalidJson(_))
        ));
    }

    #[test]
    fn rejects_invalid_ids_versions_and_api_ranges() {
        let cases = [
            (VALID.replace("example.weather", "../weather"), "id"),
            (VALID.replace("1.2.3", "not-a-version"), "version"),
            (VALID.replace("0.2.0", "0.4.0"), "api"),
        ];
        for (input, expected) in cases {
            let error = parse_manifest(input.as_bytes()).unwrap_err();
            match expected {
                "id" => assert_eq!(error, ManifestError::InvalidField("id")),
                "version" => assert_eq!(error, ManifestError::InvalidVersion("version")),
                _ => assert_eq!(error, ManifestError::InvalidApiRange),
            }
        }
    }

    #[test]
    fn rejects_path_traversal_and_non_script_entrypoints() {
        for path in [
            "../main.dream",
            "scripts/../../evil.dream",
            "/scripts/main.dream",
            "scripts\\main.dream",
            "assets/main.dream",
            "scripts/main.lua",
        ] {
            let input = VALID.replace("scripts/main.dream", path);
            assert_eq!(parse_manifest(input.as_bytes()).unwrap_err(), ManifestError::InvalidEntrypoint);
        }
    }

    #[test]
    fn rejects_unknown_and_duplicate_capabilities() {
        let unknown = VALID.replace("world.read", "process.spawn");
        assert_eq!(
            parse_manifest(unknown.as_bytes()).unwrap_err(),
            ManifestError::UnknownCapability("process.spawn".into())
        );
        let duplicate = VALID.replace("\"world.read\"", "\"events.listen\"");
        assert_eq!(
            parse_manifest(duplicate.as_bytes()).unwrap_err(),
            ManifestError::DuplicateCapability("events.listen".into())
        );
    }

    #[test]
    fn rejects_self_dependencies_and_invalid_dependency_ranges() {
        let self_dep = VALID.replace("example.base", "example.weather");
        assert_eq!(
            parse_manifest(self_dep.as_bytes()).unwrap_err(),
            ManifestError::InvalidDependencyId("example.weather".into())
        );
        let bad_range = VALID.replace("^1.0.0", "latest");
        assert_eq!(
            parse_manifest(bad_range.as_bytes()).unwrap_err(),
            ManifestError::InvalidDependencyRange("example.base".into())
        );
    }

    #[test]
    fn rejects_oversized_manifest_before_parsing() {
        let input = vec![b' '; MAX_MANIFEST_BYTES + 1];
        assert_eq!(parse_manifest(&input).unwrap_err(), ManifestError::TooLarge);
    }


    fn manifest_with(id: &str, version: &str, dependencies: &[(&str, &str)]) -> Manifest {
        Manifest {
            schema_version: 1,
            id: id.into(),
            name: id.into(),
            version: version.into(),
            api: ApiCompatibility {
                min: "0.2.0".into(),
                max: "0.3.0".into(),
            },
            entrypoint: "scripts/main.dream".into(),
            capabilities: Vec::new(),
            dependencies: dependencies
                .iter()
                .map(|(dependency, range)| ((*dependency).into(), (*range).into()))
                .collect(),
        }
    }

    #[test]
    fn dependency_order_is_stable_and_dependencies_come_first() {
        let manifests = [
            manifest_with("example.app", "1.0.0", &[("example.base", "^1.0")]),
            manifest_with("example.zed", "1.0.0", &[]),
            manifest_with("example.base", "1.2.0", &[]),
        ];
        assert_eq!(
            resolve_load_order(&manifests).unwrap(),
            vec!["example.base", "example.app", "example.zed"]
        );
    }

    #[test]
    fn dependency_resolver_rejects_missing_and_incompatible_dependencies() {
        let missing = [manifest_with(
            "example.app",
            "1.0.0",
            &[("example.missing", "^1.0")],
        )];
        assert!(matches!(
            resolve_load_order(&missing),
            Err(DependencyError::MissingDependency { .. })
        ));

        let incompatible = [
            manifest_with("example.app", "1.0.0", &[("example.base", "^2.0")]),
            manifest_with("example.base", "1.5.0", &[]),
        ];
        assert!(matches!(
            resolve_load_order(&incompatible),
            Err(DependencyError::VersionMismatch { .. })
        ));
    }

    #[test]
    fn dependency_resolver_rejects_cycles_and_duplicate_ids() {
        let cycle = [
            manifest_with("example.alpha", "1.0.0", &[("example.beta", "^1.0")]),
            manifest_with("example.beta", "1.0.0", &[("example.alpha", "^1.0")]),
        ];
        assert!(matches!(
            resolve_load_order(&cycle),
            Err(DependencyError::DependencyCycle(_))
        ));

        let duplicate = [
            manifest_with("example.same", "1.0.0", &[]),
            manifest_with("example.same", "1.1.0", &[]),
        ];
        assert_eq!(
            resolve_load_order(&duplicate),
            Err(DependencyError::DuplicateModId("example.same".into()))
        );
    }

    fn archive_file(path: &str, size: u64) -> ArchiveEntry {
        ArchiveEntry {
            path: path.into(),
            kind: ArchiveEntryKind::File,
            compressed_bytes: size,
            uncompressed_bytes: size,
            encrypted: false,
        }
    }

    fn valid_inventory() -> Vec<ArchiveEntry> {
        vec![
            archive_file("manifest.json", 300),
            archive_file("scripts/main.dream", 100),
            archive_file("assets/icon.png", 50),
        ]
    }

    #[test]
    fn accepts_safe_archive_inventory_and_requires_entrypoint() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        assert_eq!(
            validate_archive_inventory(&valid_inventory(), &manifest),
            Ok(())
        );
        let mut missing = valid_inventory();
        missing.remove(1);
        assert_eq!(
            validate_archive_inventory(&missing, &manifest),
            Err(ArchiveError::MissingEntrypoint("scripts/main.dream".into()))
        );
    }

    #[test]
    fn rejects_traversal_absolute_and_nonportable_archive_paths() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        for path in ["../evil", "scripts/../../evil.dream", "/manifest.json", "C:/evil", "scripts\\\\evil.dream", "scripts/.hidden.dream"] {
            let mut entries = valid_inventory();
            entries[1].path = path.into();
            assert!(matches!(validate_archive_inventory(&entries, &manifest), Err(ArchiveError::UnsafePath(_))));
        }
    }

    #[test]
    fn rejects_duplicate_casefolded_paths_and_unsupported_types() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        let mut duplicate = valid_inventory();
        duplicate.push(archive_file("SCRIPTS/MAIN.DREAM", 100));
        assert!(matches!(
            validate_archive_inventory(&duplicate, &manifest),
            Err(ArchiveError::DuplicatePath(_))
        ));

        let mut symlink = valid_inventory();
        symlink[2].kind = ArchiveEntryKind::Symlink;
        assert!(matches!(
            validate_archive_inventory(&symlink, &manifest),
            Err(ArchiveError::UnsupportedEntryType(_))
        ));
    }

    #[test]
    fn rejects_encryption_size_ratio_and_entry_count_abuse() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        let mut encrypted = valid_inventory();
        encrypted[1].encrypted = true;
        assert!(matches!(
            validate_archive_inventory(&encrypted, &manifest),
            Err(ArchiveError::EncryptedEntry(_))
        ));

        let mut bomb = valid_inventory();
        bomb[1].uncompressed_bytes = 10_001;
        bomb[1].compressed_bytes = 1;
        assert!(matches!(
            validate_archive_inventory(&bomb, &manifest),
            Err(ArchiveError::SuspiciousCompressionRatio(_))
        ));

        let too_many = vec![archive_file("assets/file.bin", 1); MAX_ARCHIVE_ENTRIES + 1];
        assert_eq!(
            validate_archive_inventory(&too_many, &manifest),
            Err(ArchiveError::TooManyEntries)
        );
    }

    #[test]
    fn rejects_missing_manifest_and_oversized_files() {
        let manifest = parse_manifest(VALID.as_bytes()).unwrap();
        let mut missing = valid_inventory();
        missing.remove(0);
        assert_eq!(
            validate_archive_inventory(&missing, &manifest),
            Err(ArchiveError::MissingManifest)
        );

        let mut oversized = valid_inventory();
        oversized[2].uncompressed_bytes = MAX_ARCHIVE_FILE_BYTES + 1;
        assert!(matches!(
            validate_archive_inventory(&oversized, &manifest),
            Err(ArchiveError::FileTooLarge(_))
        ));
    }
}
