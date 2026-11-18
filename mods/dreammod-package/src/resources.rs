//! Transactional ownership bookkeeping for mod-provided resource identifiers.
//!
//! This registry deliberately does not call Horizon or publish renderer/UI resources.
//! It provides an atomic reservation layer that a future host adapter can commit before
//! publishing external resources, and release when a mod is disabled or fails.

use std::collections::{BTreeMap, BTreeSet};

pub const MAX_RESOURCES_PER_MOD: usize = 1_024;
pub const MAX_REGISTERED_RESOURCES: usize = 16_384;
pub const MAX_RESOURCE_ID_BYTES: usize = 255;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    InvalidOwner,
    InvalidId(String),
    DuplicateInTransaction(String),
    AlreadyRegistered { id: String, owner: String },
    PerModLimit { maximum: usize },
    GlobalLimit { maximum: usize },
}

/// Committed resource identifier ownership. No external side effects are performed.
#[derive(Debug, Default)]
pub struct ResourceRegistry {
    owners: BTreeMap<String, String>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a transaction. Staged identifiers remain invisible until commit succeeds.
    pub fn begin(&self, owner: impl Into<String>) -> Result<ResourceTransaction, ResourceError> {
        let owner = owner.into();
        if !valid_owner(&owner) {
            return Err(ResourceError::InvalidOwner);
        }
        Ok(ResourceTransaction {
            owner,
            staged: BTreeSet::new(),
        })
    }

    pub fn owner_of(&self, id: &str) -> Option<&str> {
        self.owners.get(id).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.owners.len()
    }

    pub fn is_empty(&self) -> bool {
        self.owners.is_empty()
    }

    /// Release every committed identifier belonging to one mod.
    /// Returns removed identifiers in deterministic lexicographic order.
    pub fn remove_owner(&mut self, owner: &str) -> Vec<String> {
        let removed: Vec<String> = self
            .owners
            .iter()
            .filter(|(_, registered_owner)| registered_owner.as_str() == owner)
            .map(|(id, _)| id.clone())
            .collect();
        for id in &removed {
            self.owners.remove(id);
        }
        removed
    }
}

/// Uncommitted reservations. Dropping this value is a rollback.
#[derive(Debug)]
pub struct ResourceTransaction {
    owner: String,
    staged: BTreeSet<String>,
}

impl ResourceTransaction {
    pub fn stage(&mut self, id: impl Into<String>) -> Result<(), ResourceError> {
        let id = id.into();
        if !valid_resource_id(&id) {
            return Err(ResourceError::InvalidId(id));
        }
        if !self.staged.insert(id.clone()) {
            return Err(ResourceError::DuplicateInTransaction(id));
        }
        if self.staged.len() > MAX_RESOURCES_PER_MOD {
            self.staged.remove(&id);
            return Err(ResourceError::PerModLimit {
                maximum: MAX_RESOURCES_PER_MOD,
            });
        }
        Ok(())
    }

    /// Atomically reserve all staged IDs. On any error, the registry is unchanged.
    pub fn commit(self, registry: &mut ResourceRegistry) -> Result<Vec<String>, ResourceError> {
        for id in &self.staged {
            if let Some(owner) = registry.owners.get(id) {
                return Err(ResourceError::AlreadyRegistered {
                    id: id.clone(),
                    owner: owner.clone(),
                });
            }
        }

        let total = registry.owners.len().saturating_add(self.staged.len());
        if total > MAX_REGISTERED_RESOURCES {
            return Err(ResourceError::GlobalLimit {
                maximum: MAX_REGISTERED_RESOURCES,
            });
        }

        let committed: Vec<String> = self.staged.into_iter().collect();
        for id in &committed {
            registry.owners.insert(id.clone(), self.owner.clone());
        }
        Ok(committed)
    }
}

fn valid_owner(value: &str) -> bool {
    value.len() <= 128
        && value.split('.').count() >= 2
        && value.split('.').all(|part| {
            !part.is_empty()
                && !part.starts_with('-')
                && !part.ends_with('-')
                && part.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
}

fn valid_resource_id(value: &str) -> bool {
    if value.is_empty()
        || value.len() > MAX_RESOURCE_ID_BYTES
        || value.contains('\\')
        || value.chars().any(char::is_control)
    {
        return false;
    }
    let Some((namespace, path)) = value.split_once(':') else {
        return false;
    };
    !namespace.is_empty()
        && !path.is_empty()
        && !path.contains(':')
        && namespace.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
        })
        && path.split('/').all(|part| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && part.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'_' | b'-' | b'.')
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_is_invisible_until_atomic_commit() {
        let mut registry = ResourceRegistry::new();
        let mut tx = registry.begin("example.mod").unwrap();
        tx.stage("example:block/a").unwrap();
        tx.stage("example:item/a").unwrap();
        assert_eq!(registry.len(), 0);
        assert_eq!(
            tx.commit(&mut registry).unwrap(),
            vec!["example:block/a", "example:item/a"]
        );
        assert_eq!(registry.len(), 2);
        assert_eq!(registry.owner_of("example:block/a"), Some("example.mod"));
    }

    #[test]
    fn dropping_uncommitted_transaction_rolls_back_everything() {
        let mut registry = ResourceRegistry::new();
        {
            let mut tx = registry.begin("example.mod").unwrap();
            tx.stage("example:block/a").unwrap();
        }
        assert!(registry.is_empty());
    }

    #[test]
    fn duplicate_and_invalid_ids_are_rejected() {
        let registry = ResourceRegistry::new();
        let mut tx = registry.begin("example.mod").unwrap();
        assert_eq!(tx.stage("example:block/a"), Ok(()));
        assert_eq!(
            tx.stage("example:block/a"),
            Err(ResourceError::DuplicateInTransaction("example:block/a".into()))
        );
        for id in ["", "plain", ":missing", "example:", "example:a:b", "example:../secret", "Example:block", "example:a//b"] {
            assert!(matches!(tx.stage(id), Err(ResourceError::InvalidId(_))), "{id}");
        }
    }

    #[test]
    fn conflicts_abort_the_whole_transaction_without_partial_registration() {
        let mut registry = ResourceRegistry::new();
        let mut first = registry.begin("example.first").unwrap();
        first.stage("example:block/shared").unwrap();
        first.commit(&mut registry).unwrap();

        let mut second = registry.begin("example.second").unwrap();
        second.stage("example:block/new").unwrap();
        second.stage("example:block/shared").unwrap();
        assert_eq!(
            second.commit(&mut registry),
            Err(ResourceError::AlreadyRegistered {
                id: "example:block/shared".into(),
                owner: "example.first".into(),
            })
        );
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.owner_of("example:block/new"), None);
    }

    #[test]
    fn owner_cleanup_is_deterministic_and_does_not_touch_other_mods() {
        let mut registry = ResourceRegistry::new();
        for (owner, id) in [
            ("example.z", "example:z"),
            ("example.a", "example:a2"),
            ("example.a", "example:a1"),
        ] {
            let mut tx = registry.begin(owner).unwrap();
            tx.stage(id).unwrap();
            tx.commit(&mut registry).unwrap();
        }
        assert_eq!(
            registry.remove_owner("example.a"),
            vec!["example:a1", "example:a2"]
        );
        assert_eq!(registry.len(), 1);
        assert_eq!(registry.owner_of("example:z"), Some("example.z"));
    }

    #[test]
    fn enforces_per_mod_and_global_quotas() {
        let mut registry = ResourceRegistry::new();
        for owner_index in 0..16 {
            let owner = format!("example.large{owner_index}");
            let mut tx = registry.begin(owner).unwrap();
            for index in 0..MAX_RESOURCES_PER_MOD {
                tx.stage(format!("example:owner{owner_index}/r{index}")).unwrap();
            }
            tx.commit(&mut registry).unwrap();
        }
        assert_eq!(registry.len(), MAX_REGISTERED_RESOURCES);

        let mut per_mod = registry.begin("example.permod").unwrap();
        for index in 0..MAX_RESOURCES_PER_MOD {
            per_mod.stage(format!("permod:r{index}")).unwrap();
        }
        assert_eq!(
            per_mod.stage("permod:overflow"),
            Err(ResourceError::PerModLimit { maximum: MAX_RESOURCES_PER_MOD })
        );

        let mut global = registry.begin("example.second").unwrap();
        global.stage("second:one").unwrap();
        assert_eq!(
            global.commit(&mut registry),
            Err(ResourceError::GlobalLimit { maximum: MAX_REGISTERED_RESOURCES })
        );
        assert_eq!(registry.len(), MAX_REGISTERED_RESOURCES);
    }

    #[test]
    fn rejects_invalid_owner_ids() {
        let registry = ResourceRegistry::new();
        for owner in ["", "single", "Example.mod", "example..mod", "example.-bad"] {
            assert!(matches!(registry.begin(owner), Err(ResourceError::InvalidOwner)));
        }
    }
}
