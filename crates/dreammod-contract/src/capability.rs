/// A validated capability key. Examples: `world.read`, `physics.body.create`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityError> {
        let value = value.into();
        let mut parts = value.split('.');
        let valid = parts.clone().count() >= 2
            && parts.all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-'));
        if value.len() > 96 || !valid {
            return Err(CapabilityError::Invalid);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityError { Invalid }
