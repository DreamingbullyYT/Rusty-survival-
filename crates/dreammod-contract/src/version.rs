use core::fmt;

/// DreamMod public API semantic version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ApiVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ApiVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self { Self { major, minor, patch } }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Inclusive lower bound and exclusive upper bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApiVersionRange {
    min_inclusive: ApiVersion,
    max_exclusive: ApiVersion,
}

impl ApiVersionRange {
    pub fn new(min_inclusive: ApiVersion, max_exclusive: ApiVersion) -> Result<Self, VersionRangeError> {
        if min_inclusive >= max_exclusive {
            return Err(VersionRangeError::InvalidBounds);
        }
        Ok(Self { min_inclusive, max_exclusive })
    }

    pub const fn min_inclusive(&self) -> ApiVersion { self.min_inclusive }
    pub const fn max_exclusive(&self) -> ApiVersion { self.max_exclusive }

    pub fn contains(&self, version: ApiVersion) -> bool {
        version >= self.min_inclusive && version < self.max_exclusive
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionRangeError { InvalidBounds }
