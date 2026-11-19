//! Host-independent declarations for Horizon texture-atlas registration.
//!
//! This validates and deterministically snapshots declarations. It does not call Horizon,
//! parse atlas JSON, or load/stitch sprites.
use std::collections::{BTreeSet, HashSet};

pub const MAX_ATLAS_DECLARATIONS: usize = 32;
pub const MAX_ATLAS_ID_BYTES: usize = 255;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtlasDeclaration {
    pub owner: String,
    pub atlas_id: String,
    pub definition_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtlasError {
    InvalidOwner(String),
    InvalidIdentifier(String),
    NamespaceMismatch { owner: String, identifier: String },
    InvalidDefinitionPath(String),
    DuplicateAtlas(String),
    LimitExceeded { maximum: usize },
}

/// Sorted immutable input for a host hook; only enabled owners are included.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AtlasSnapshot {
    declarations: Vec<AtlasDeclaration>,
}

impl AtlasDeclaration {
    pub fn new(owner: impl Into<String>, atlas_id: impl Into<String>, definition_id: impl Into<String>) -> Result<Self, AtlasError> {
        let owner = owner.into();
        let atlas_id = atlas_id.into();
        let definition_id = definition_id.into();
        if !valid_owner(&owner) { return Err(AtlasError::InvalidOwner(owner)); }
        for identifier in [&atlas_id, &definition_id] {
            if !valid_identifier(identifier) { return Err(AtlasError::InvalidIdentifier(identifier.clone())); }
            if !identifier.starts_with(&format!("{owner}:")) {
                return Err(AtlasError::NamespaceMismatch { owner: owner.clone(), identifier: identifier.clone() });
            }
        }
        let definition_path = definition_id.split_once(':').expect("validated identifier").1;
        if definition_path.split('/').any(|part| part.is_empty() || part == "." || part == "..") || definition_path.contains('\\') {
            return Err(AtlasError::InvalidDefinitionPath(definition_id));
        }
        Ok(Self { owner, atlas_id, definition_id })
    }

    /// Horizon resolves namespace:path to assets/namespace/atlases/path.json.
    pub fn asset_path(&self) -> String {
        let path = self.definition_id.split_once(':').expect("validated identifier").1;
        format!("assets/{}/atlases/{path}.json", self.owner)
    }
}

impl AtlasSnapshot {
    pub fn build(enabled_owners: &BTreeSet<String>, declarations: Vec<AtlasDeclaration>) -> Result<Self, AtlasError> {
        let mut filtered = declarations.into_iter().filter(|d| enabled_owners.contains(&d.owner)).collect::<Vec<_>>();
        let mut ids = HashSet::new();
        for declaration in &filtered {
            AtlasDeclaration::new(&declaration.owner, &declaration.atlas_id, &declaration.definition_id)?;
            if !ids.insert(declaration.atlas_id.clone()) { return Err(AtlasError::DuplicateAtlas(declaration.atlas_id.clone())); }
        }
        if filtered.len() > MAX_ATLAS_DECLARATIONS { return Err(AtlasError::LimitExceeded { maximum: MAX_ATLAS_DECLARATIONS }); }
        filtered.sort_by(|a, b| a.atlas_id.cmp(&b.atlas_id).then_with(|| a.definition_id.cmp(&b.definition_id)));
        Ok(Self { declarations: filtered })
    }

    pub fn declarations(&self) -> &[AtlasDeclaration] { &self.declarations }
    pub fn is_empty(&self) -> bool { self.declarations.is_empty() }
}

fn valid_owner(value: &str) -> bool {
    value.len() <= 128 && value.split('.').count() >= 2 && value.split('.').all(|part| {
        !part.is_empty() && !part.starts_with('-') && !part.ends_with('-')
            && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    })
}

fn valid_identifier(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_ATLAS_ID_BYTES || value.contains('\\') || value.chars().any(char::is_control) { return false; }
    let Some((namespace, path)) = value.split_once(':') else { return false; };
    !namespace.is_empty() && !path.is_empty() && !path.contains(':')
        && namespace.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.'))
        && path.split('/').all(|part| !part.is_empty() && part != "." && part != ".."
            && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_maps_definition_to_package_asset_path() {
        let d = AtlasDeclaration::new("example.mod", "example.mod:icons", "example.mod:ui/icons").unwrap();
        assert_eq!(d.asset_path(), "assets/example.mod/atlases/ui/icons.json");
    }

    #[test]
    fn rejects_foreign_namespace_and_traversal() {
        assert!(matches!(AtlasDeclaration::new("example.mod", "foreign:icons", "example.mod:icons"), Err(AtlasError::NamespaceMismatch { .. })));
        assert!(matches!(AtlasDeclaration::new("example.mod", "example.mod:icons", "example.mod:../secret"), Err(AtlasError::InvalidIdentifier(_))));
        assert!(matches!(AtlasDeclaration::new("example.mod", "example.mod:icons", "example.mod:icons\\bad"), Err(AtlasError::InvalidIdentifier(_))));
    }

    #[test]
    fn snapshot_contains_only_enabled_mods_and_sorts_deterministically() {
        let a = AtlasDeclaration::new("example.mod", "example.mod:z", "example.mod:z").unwrap();
        let b = AtlasDeclaration::new("example.mod", "example.mod:a", "example.mod:a").unwrap();
        let disabled = AtlasDeclaration::new("example.off", "example.off:atlas", "example.off:atlas").unwrap();
        let enabled = BTreeSet::from(["example.mod".to_owned()]);
        let snapshot = AtlasSnapshot::build(&enabled, vec![a, disabled, b]).unwrap();
        assert_eq!(snapshot.declarations().iter().map(|d| d.atlas_id.as_str()).collect::<Vec<_>>(), vec!["example.mod:a", "example.mod:z"]);
    }

    #[test]
    fn rejects_duplicate_atlas_ids_and_excess_count() {
        let enabled = BTreeSet::from(["example.mod".to_owned()]);
        let a = AtlasDeclaration::new("example.mod", "example.mod:one", "example.mod:one").unwrap();
        let duplicate = AtlasDeclaration::new("example.mod", "example.mod:one", "example.mod:two").unwrap();
        assert!(matches!(AtlasSnapshot::build(&enabled, vec![a.clone(), duplicate]), Err(AtlasError::DuplicateAtlas(_))));
        let many = (0..=MAX_ATLAS_DECLARATIONS).map(|i| AtlasDeclaration::new("example.mod", format!("example.mod:atlas{i}"), format!("example.mod:atlas{i}")).unwrap()).collect();
        assert!(matches!(AtlasSnapshot::build(&enabled, many), Err(AtlasError::LimitExceeded { maximum: MAX_ATLAS_DECLARATIONS })));
    }
}
