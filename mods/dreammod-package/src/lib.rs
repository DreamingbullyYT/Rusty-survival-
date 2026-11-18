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
    let Some(first) = parts.next() else { return false };
    first == "scripts"
        && parts.all(|part| !part.is_empty() && part != "." && part != "..")
        && path.ends_with(".dream")
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
        assert!(matches!(parse_manifest(input.as_bytes()), Err(ManifestError::InvalidJson(_))));
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
        for path in ["../main.dream", "scripts/../../evil.dream", "/scripts/main.dream", "scripts\\main.dream", "assets/main.dream", "scripts/main.lua"] {
            let input = VALID.replace("scripts/main.dream", path);
            assert_eq!(parse_manifest(input.as_bytes()).unwrap_err(), ManifestError::InvalidEntrypoint);
        }
    }

    #[test]
    fn rejects_unknown_and_duplicate_capabilities() {
        let unknown = VALID.replace("world.read", "process.spawn");
        assert_eq!(parse_manifest(unknown.as_bytes()).unwrap_err(), ManifestError::UnknownCapability("process.spawn".into()));
        let duplicate = VALID.replace("\"world.read\"", "\"events.listen\"");
        assert_eq!(parse_manifest(duplicate.as_bytes()).unwrap_err(), ManifestError::DuplicateCapability("events.listen".into()));
    }

    #[test]
    fn rejects_self_dependencies_and_invalid_dependency_ranges() {
        let self_dep = VALID.replace("example.base", "example.weather");
        assert_eq!(parse_manifest(self_dep.as_bytes()).unwrap_err(), ManifestError::InvalidDependencyId("example.weather".into()));
        let bad_range = VALID.replace("^1.0.0", "latest");
        assert_eq!(parse_manifest(bad_range.as_bytes()).unwrap_err(), ManifestError::InvalidDependencyRange("example.base".into()));
    }

    #[test]
    fn rejects_oversized_manifest_before_parsing() {
        let input = vec![b' '; MAX_MANIFEST_BYTES + 1];
        assert_eq!(parse_manifest(&input).unwrap_err(), ManifestError::TooLarge);
    }
}
