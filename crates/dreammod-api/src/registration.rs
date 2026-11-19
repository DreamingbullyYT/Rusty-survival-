use std::collections::{BTreeMap, BTreeSet};

use dreammod_contract::{ApiFeatureId, ModId, ResourceId};

/// Kind of extension point a registration occupies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RegistrationKind {
    Command,
    EventListener,
    System,
    Content,
    Provider,
    Resource,
}

/// Owned declaration registered by a mod.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Registration {
    owner: ModId,
    id: ResourceId,
    kind: RegistrationKind,
    priority: i32,
    required_feature: Option<ApiFeatureId>,
}

impl Registration {
    pub fn new(
        owner: ModId,
        id: ResourceId,
        kind: RegistrationKind,
        priority: i32,
    ) -> Result<Self, RegistryError> {
        if owner.as_str() != id.namespace() {
            return Err(RegistryError::NamespaceMismatch);
        }
        Ok(Self {
            owner,
            id,
            kind,
            priority,
            required_feature: None,
        })
    }

    pub fn with_required_feature(mut self, feature: ApiFeatureId) -> Self {
        self.required_feature = Some(feature);
        self
    }

    pub fn owner(&self) -> &ModId {
        &self.owner
    }

    pub fn id(&self) -> &ResourceId {
        &self.id
    }

    pub fn kind(&self) -> RegistrationKind {
        self.kind
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn required_feature(&self) -> Option<&ApiFeatureId> {
        self.required_feature.as_ref()
    }
}

/// Deterministic, owner-scoped registry with transactional batch registration.
#[derive(Clone, Debug, Default)]
pub struct RegistrationRegistry {
    entries: BTreeMap<ResourceId, Registration>,
}

impl RegistrationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates the complete batch before mutating the registry.
    pub fn register_batch(
        &mut self,
        owner: &ModId,
        batch: Vec<Registration>,
    ) -> Result<usize, RegistryError> {
        let mut batch_ids = BTreeSet::new();
        for entry in &batch {
            if entry.owner != *owner {
                return Err(RegistryError::OwnerMismatch);
            }
            if entry.id.namespace() != owner.as_str() {
                return Err(RegistryError::NamespaceMismatch);
            }
            if self.entries.contains_key(&entry.id) || !batch_ids.insert(entry.id.clone()) {
                return Err(RegistryError::DuplicateId);
            }
        }

        let count = batch.len();
        for entry in batch {
            self.entries.insert(entry.id.clone(), entry);
        }
        Ok(count)
    }

    /// Removes all registrations owned by one mod. Safe to call repeatedly.
    pub fn unregister_owner(&mut self, owner: &ModId) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, entry| entry.owner != *owner);
        before - self.entries.len()
    }

    pub fn get(&self, id: &ResourceId) -> Option<&Registration> {
        self.entries.get(id)
    }

    /// Snapshot sorted by priority (descending), then resource ID (ascending).
    pub fn snapshot(&self, kind: Option<RegistrationKind>) -> Vec<&Registration> {
        let mut entries: Vec<_> = self
            .entries
            .values()
            .filter(|entry| kind.is_none_or(|wanted| entry.kind == wanted))
            .collect();
        entries.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.id.cmp(&b.id)));
        entries
    }

    pub fn owner_snapshot(&self, owner: &ModId) -> Vec<&Registration> {
        self.entries
            .values()
            .filter(|entry| entry.owner == *owner)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryError {
    NamespaceMismatch,
    OwnerMismatch,
    DuplicateId,
}
