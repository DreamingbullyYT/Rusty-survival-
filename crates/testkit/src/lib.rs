//! Data contracts and validation for version-pinned vanilla reference fixtures.
//!
//! This crate is test infrastructure, not part of the runtime engine. Synthetic
//! fixtures exercise this contract only and are never evidence of Java parity.

use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

pub const FIXTURE_SCHEMA_VERSION: u32 = 1;
pub const TARGET_MINECRAFT_VERSION: &str = "26.3";

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FixtureKind {
    Synthetic,
    VanillaReference,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    ServerJar,
    ClientJar,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fixture {
    pub schema_version: u32,
    pub id: String,
    pub kind: FixtureKind,
    pub target: Target,
    pub subsystem: String,
    #[serde(default)]
    pub artifact: Option<Artifact>,
    pub scenario: Scenario,
    pub expected: Value,
    #[serde(default)]
    pub normalization: Normalization,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Target {
    pub edition: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Artifact {
    pub kind: ArtifactKind,
    pub sha1: String,
    pub size_bytes: u64,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scenario {
    #[serde(default)]
    pub seed: Option<i64>,
    #[serde(default)]
    pub dimension: Option<String>,
    #[serde(default)]
    pub coordinates: Option<Coordinates>,
    #[serde(default)]
    pub settings: BTreeMap<String, Value>,
    pub initial_state: Value,
    #[serde(default)]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Action {
    pub operation: String,
    #[serde(default)]
    pub ticks: u32,
    #[serde(default)]
    pub input: Value,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Normalization {
    #[serde(default)]
    pub ignored_paths: Vec<String>,
    #[serde(default)]
    pub float_tolerance: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Provenance {
    pub capture_tool: String,
    pub java_runtime: String,
    pub captured_at: String,
    pub source: String,
}

impl Fixture {
    /// Parse and validate a fixture, returning every detected contract violation.
    pub fn parse_and_validate(input: &str) -> Result<Self, Vec<String>> {
        let fixture: Self = serde_json::from_str(input)
            .map_err(|error| vec![format!("invalid fixture JSON: {error}")])?;
        let errors = fixture.validate();
        if errors.is_empty() {
            Ok(fixture)
        } else {
            Err(errors)
        }
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.schema_version != FIXTURE_SCHEMA_VERSION {
            errors.push(format!(
                "schema_version must be {FIXTURE_SCHEMA_VERSION}"
            ));
        }
        if self.id.trim().is_empty() {
            errors.push("id must not be empty".to_owned());
        }
        if self.subsystem.trim().is_empty() {
            errors.push("subsystem must not be empty".to_owned());
        }
        if self.target.edition != "java" {
            errors.push("target.edition must be java".to_owned());
        }
        if self.target.version != TARGET_MINECRAFT_VERSION {
            errors.push(format!(
                "target.version must be {TARGET_MINECRAFT_VERSION}"
            ));
        }

        match (&self.kind, &self.artifact) {
            (FixtureKind::VanillaReference, None) => {
                errors.push("vanilla_reference fixtures require an artifact".to_owned());
            }
            (_, Some(artifact)) => {
                if artifact.sha1.len() != 40
                    || !artifact.sha1.bytes().all(|byte| byte.is_ascii_hexdigit())
                {
                    errors.push("artifact.sha1 must contain exactly 40 hexadecimal characters".to_owned());
                }
                if artifact.size_bytes == 0 {
                    errors.push("artifact.size_bytes must be greater than zero".to_owned());
                }
                let official_host = artifact
                    .url
                    .strip_prefix("https://")
                    .and_then(|url| url.split('/').next())
                    .is_some_and(|host| {
                        host == "piston-data.mojang.com"
                            || host == "launcher.mojang.com"
                    });
                if !official_host {
                    errors.push("artifact.url must use an approved official Mojang HTTPS host".to_owned());
                }
            }
            (FixtureKind::Synthetic, None) => {}
        }

        if self.scenario.dimension.as_deref() == Some("") {
            errors.push("scenario.dimension must not be empty when present".to_owned());
        }
        if self.scenario.actions.iter().any(|action| action.operation.trim().is_empty()) {
            errors.push("every action.operation must not be empty".to_owned());
        }
        if self.normalization.ignored_paths.iter().any(|path| path.trim().is_empty()) {
            errors.push("normalization.ignored_paths must not contain empty paths".to_owned());
        }
        if let Some(tolerance) = self.normalization.float_tolerance {
            if !tolerance.is_finite() || tolerance < 0.0 {
                errors.push("normalization.float_tolerance must be finite and non-negative".to_owned());
            }
        }

        for (field, value) in [
            ("provenance.capture_tool", self.provenance.capture_tool.as_str()),
            ("provenance.java_runtime", self.provenance.java_runtime.as_str()),
            ("provenance.captured_at", self.provenance.captured_at.as_str()),
            ("provenance.source", self.provenance.source.as_str()),
        ] {
            if value.trim().is_empty() {
                errors.push(format!("{field} must not be empty"));
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SYNTHETIC_FIXTURE: &str = include_str!("../tests/fixtures/synthetic-coordinate.json");

    #[test]
    fn accepts_labeled_synthetic_fixture() {
        let fixture = Fixture::parse_and_validate(SYNTHETIC_FIXTURE)
            .expect("the checked-in synthetic fixture should be valid");
        assert_eq!(fixture.kind, FixtureKind::Synthetic);
        assert_eq!(fixture.target.version, TARGET_MINECRAFT_VERSION);
        assert!(fixture.artifact.is_none());
    }

    #[test]
    fn rejects_non_target_version() {
        let input = SYNTHETIC_FIXTURE.replace("\"26.3\"", "\"26.2\"");
        let errors = Fixture::parse_and_validate(&input).unwrap_err();
        assert!(errors.iter().any(|error| error.contains("target.version")));
    }

    #[test]
    fn rejects_invalid_tolerance() {
        let input = SYNTHETIC_FIXTURE.replace("0.0001", "-0.5");
        let errors = Fixture::parse_and_validate(&input).unwrap_err();
        assert!(errors.iter().any(|error| error.contains("float_tolerance")));
    }

    #[test]
    fn rejects_vanilla_fixture_without_official_artifact() {
        let input = SYNTHETIC_FIXTURE.replace("\"synthetic\"", "\"vanilla_reference\"");
        let errors = Fixture::parse_and_validate(&input).unwrap_err();
        assert!(errors.iter().any(|error| error.contains("require an artifact")));
    }

    #[test]
    fn rejects_untrusted_artifact_host() {
        let mut fixture = Fixture::parse_and_validate(SYNTHETIC_FIXTURE).unwrap();
        fixture.artifact = Some(Artifact {
            kind: ArtifactKind::ServerJar,
            sha1: "a".repeat(40),
            size_bytes: 1,
            url: "https://example.com/server.jar".to_owned(),
        });
        let errors = fixture.validate();
        assert!(errors.iter().any(|error| error.contains("official Mojang HTTPS host")));
    }
}
