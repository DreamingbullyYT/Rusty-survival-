//! Small shared contracts between the interpreted package layer and trusted Horizon Core.
//! This crate intentionally has no third-party dependencies.

/// Capability required before a mod may inspect online player data.
pub const PLAYER_READ_CAPABILITY: &str = "entities.read";

/// Implemented by each mod's lifecycle owner and consumed by the Core collector.
/// Implementations must check both enabled state and the explicit capability grant.
pub trait PlayerReadAuthorization {
    type Error;

    fn require_entities_read(&self) -> Result<(), Self::Error>;
}
