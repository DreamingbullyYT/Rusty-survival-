use semver::Version;
use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectManifest {
    pub project: ProjectPackage,
    #[serde(default)]
    pub compatibility: CompatibilityPins,
    #[serde(default)]
    pub modules: Vec<ModuleDescriptor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectPackage {
    pub id: String,
    pub name: String,
    pub version: Version,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CompatibilityPins {
    #[serde(default)]
    pub horizon_revision: Option<String>,
    #[serde(default)]
    pub horizon_mixins_revision: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleDescriptor {
    pub id: String,
    pub path: String,
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("could not read project manifest: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid project TOML: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("invalid project id '{0}': use lowercase letters, digits, '-' or '_'")]
    InvalidProjectId(String),
    #[error("invalid module id '{0}': use dot-separated lowercase identifiers")]
    InvalidModuleId(String),
    #[error("duplicate module id '{0}'")]
    DuplicateModule(String),
}

impl ProjectManifest {
    pub fn from_str(input: &str) -> Result<Self, ManifestError> {
        let manifest: Self = toml::from_str(input)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ManifestError> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn validate(&self) -> Result<(), ManifestError> {
        if !valid_project_id(&self.project.id) {
            return Err(ManifestError::InvalidProjectId(self.project.id.clone()));
        }

        let mut seen = std::collections::BTreeSet::new();
        for module in &self.modules {
            if !valid_module_id(&module.id) {
                return Err(ManifestError::InvalidModuleId(module.id.clone()));
            }
            if !seen.insert(module.id.as_str()) {
                return Err(ManifestError::DuplicateModule(module.id.clone()));
            }
        }
        Ok(())
    }
}

fn valid_project_id(id: &str) -> bool {
    !id.is_empty()
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

fn valid_module_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('.').all(|part| {
            !part.is_empty()
                && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        })
}
