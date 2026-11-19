/// Validated engine/API feature key used for capability negotiation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ApiFeatureId(String);

impl ApiFeatureId {
    pub fn new(value: impl Into<String>) -> Result<Self, FeatureIdError> {
        let value = value.into();
        let mut parts = value.split('.');
        let valid = parts.clone().count() >= 2
            && parts.all(|part| {
                !part.is_empty()
                    && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
            });
        if value.len() > 96 || !valid {
            return Err(FeatureIdError::Invalid);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureIdError { Invalid }
