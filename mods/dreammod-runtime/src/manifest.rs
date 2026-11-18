use std::{fmt, path::Path, str::FromStr};

use semver::{Version, VersionReq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModId(String);

impl ModId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ManifestError> {
        let value = value.into();
        let valid = (3..=96).contains(&value.len())
            && value.is_ascii()
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
            })
            && value.as_bytes()[0].is_ascii_alphanumeric()
            && value.as_bytes()[value.len() - 1].is_ascii_alphanumeric()
            && !value.contains("..")
            && !value.contains("__")
            && !value.contains("--");

        if !valid {
            return Err(ManifestError::InvalidModId(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ModId {
    type Err = ManifestError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for ModId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ModId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Capability {
    #[serde(rename = "world.read")]
    WorldRead,
    #[serde(rename = "world.write")]
    WorldWrite,
    #[serde(rename = "entity.read")]
    EntityRead,
    #[serde(rename = "entity.write")]
    EntityWrite,
    #[serde(rename = "physics.override")]
    PhysicsOverride,
    #[serde(rename = "ai.override")]
    AiOverride,
    #[serde(rename = "render.override")]
    RenderOverride,
    #[serde(rename = "audio.control")]
    AudioControl,
    #[serde(rename = "ui.control")]
    UiControl,
    #[serde(rename = "network.access")]
    NetworkAccess,
    #[serde(rename = "registry.write")]
    RegistryWrite,
    #[serde(rename = "commands.register")]
    CommandsRegister,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModSide {
    Client,
    Server,
    #[default]
    Both,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModDependency {
    pub id: ModId,
    pub version: VersionReq,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModManifest {
    pub schema_version: u32,
    pub id: ModId,
    pub version: Version,
    pub display_name: String,
    pub dreammod_api: VersionReq,
    pub entry: String,
    #[serde(default)]
    pub side: ModSide,
    #[serde(default)]
    pub load_priority: i32,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<ModDependency>,
    #[serde(default)]
    pub required_capabilities: Vec<Capability>,
    #[serde(default)]
    pub optional_capabilities: Vec<Capability>,
}

impl ModManifest {
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.schema_version != 1 {
            return Err(ManifestError::UnsupportedSchema(self.schema_version));
        }
        if self.display_name.trim().is_empty() || self.display_name.len() > 96 {
            return Err(ManifestError::InvalidDisplayName);
        }
        if !is_safe_relative_path(&self.entry) || !self.entry.ends_with(".dream") {
            return Err(ManifestError::InvalidEntry(self.entry.clone()));
        }
        if !(-1_000..=1_000).contains(&self.load_priority) {
            return Err(ManifestError::InvalidLoadPriority(self.load_priority));
        }

        let mut event_names = std::collections::BTreeSet::new();
        for event in &self.events {
            if !is_valid_event_name(event) || !event_names.insert(event) {
                return Err(ManifestError::InvalidEvent(event.clone()));
            }
        }

        let mut dependency_ids = std::collections::BTreeSet::new();
        for dependency in &self.dependencies {
            if dependency.id == self.id || !dependency_ids.insert(&dependency.id) {
                return Err(ManifestError::InvalidDependency(dependency.id.to_string()));
            }
        }

        let required: std::collections::BTreeSet<_> =
            self.required_capabilities.iter().copied().collect();
        let optional: std::collections::BTreeSet<_> =
            self.optional_capabilities.iter().copied().collect();
        if required.len() != self.required_capabilities.len()
            || optional.len() != self.optional_capabilities.len()
            || !required.is_disjoint(&optional)
        {
            return Err(ManifestError::DuplicateOrOverlappingCapabilities);
        }

        Ok(())
    }

    pub fn requested_capabilities(&self) -> std::collections::BTreeSet<Capability> {
        self.required_capabilities
            .iter()
            .chain(self.optional_capabilities.iter())
            .copied()
            .collect()
    }
}

pub fn is_safe_relative_path(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with('/')
        || value.contains('\\')
        || value.contains(':')
        || value.contains('\0')
    {
        return false;
    }

    let path = Path::new(value);
    path.components()
        .all(|component| matches!(component, std::path::Component::Normal(_)))
}

pub fn is_valid_event_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    matches!(bytes.next(), Some(b'a'..=b'z'))
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && value.len() <= 64
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("invalid mod id: {0}")]
    InvalidModId(String),
    #[error("unsupported .dmod schema version: {0}")]
    UnsupportedSchema(u32),
    #[error("display_name must contain 1 to 96 characters")]
    InvalidDisplayName,
    #[error("unsafe or invalid script entry path: {0}")]
    InvalidEntry(String),
    #[error("load_priority must be between -1000 and 1000, got {0}")]
    InvalidLoadPriority(i32),
    #[error("invalid or duplicate event name: {0}")]
    InvalidEvent(String),
    #[error("invalid or duplicate dependency: {0}")]
    InvalidDependency(String),
    #[error("capabilities must be unique and required/optional sets must not overlap")]
    DuplicateOrOverlappingCapabilities,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_ids_are_strict_and_stable() {
        assert!(ModId::parse("dreammod.example").is_ok());
        assert!(ModId::parse("BadUppercase").is_err());
        assert!(ModId::parse("../escape").is_err());
        assert!(ModId::parse("a").is_err());
    }

    #[test]
    fn safe_paths_reject_traversal_and_windows_paths() {
        assert!(is_safe_relative_path("scripts/main.dream"));
        assert!(!is_safe_relative_path("../main.dream"));
        assert!(!is_safe_relative_path("/absolute/main.dream"));
        assert!(!is_safe_relative_path("scripts\\main.dream"));
        assert!(!is_safe_relative_path("C:/mods/main.dream"));
    }

    #[test]
    fn manifest_rejects_duplicate_events_and_overlapping_permissions() {
        let manifest: ModManifest = serde_json::from_str(
            r#"{
                "schema_version": 1,
                "id": "dreammod.example",
                "version": "1.0.0",
                "display_name": "Example",
                "dreammod_api": "^0.1.0",
                "entry": "scripts/main.dream",
                "events": ["server_tick", "server_tick"],
                "required_capabilities": ["world.read"],
                "optional_capabilities": ["world.read"]
            }"#,
        )
        .unwrap();

        assert!(matches!(
            manifest.validate(),
            Err(ManifestError::InvalidEvent(_))
        ));
    }
}
