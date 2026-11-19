use std::collections::BTreeSet;

/// A validated capability key. Examples: `world.read`, `physics.body.create`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityError> {
        let value = value.into();
        let mut parts = value.split('.');
        let valid = parts.clone().count() >= 2
            && parts.all(|part| {
                !part.is_empty()
                    && part
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
            });
        if value.len() > 96 || !valid {
            return Err(CapabilityError::Invalid);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Deterministic set of granted capabilities for one mod lifecycle.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySet {
    grants: BTreeSet<CapabilityId>,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a grant. Returns true only if it was not already present.
    pub fn grant(&mut self, capability: CapabilityId) -> bool {
        self.grants.insert(capability)
    }

    /// Removes a grant. Returns true if a grant existed.
    pub fn revoke(&mut self, capability: &CapabilityId) -> bool {
        self.grants.remove(capability)
    }

    pub fn contains(&self, capability: &CapabilityId) -> bool {
        self.grants.contains(capability)
    }

    pub fn len(&self) -> usize {
        self.grants.len()
    }

    pub fn is_empty(&self) -> bool {
        self.grants.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CapabilityId> {
        self.grants.iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityError {
    Invalid,
}
