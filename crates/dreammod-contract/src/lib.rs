//! Engine-independent, dependency-free contracts shared by DreamMod and engine adapters.
//!
//! This crate intentionally contains no Horizon/HNMods or engine-specific types.

mod capability;
mod diagnostic;
mod feature;
mod ids;
mod version;

pub use capability::{CapabilityId, CapabilitySet};
pub use diagnostic::{Diagnostic, DiagnosticCode, Severity};
pub use feature::{ApiFeatureId, FeatureIdError};
pub use ids::{Handle, HandleKind, ModId, ResourceId};
pub use version::{ApiVersion, ApiVersionRange};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_ids_are_lowercase_and_bounded() {
        assert!(ModId::new("dreamingbully.example").is_ok());
        assert!(ModId::new("my-mod_2").is_ok());
        assert!(ModId::new("BadName").is_err());
        assert!(ModId::new("../escape").is_err());
        assert!(ModId::new("").is_err());
    }

    #[test]
    fn resource_ids_require_namespace_and_safe_path() {
        let id = ResourceId::new("dreammod", "vehicles/small_car").unwrap();
        assert_eq!(id.to_string(), "dreammod:vehicles/small_car");
        assert!(ResourceId::new("BadNamespace", "item").is_err());
        assert!(ResourceId::new("minecraft", "../secret").is_err());
        assert!(ResourceId::new("x", "a//b").is_err());
    }

    #[test]
    fn capability_names_are_domain_operation_and_grants_are_deterministic() {
        assert!(CapabilityId::new("world.read").is_ok());
        assert!(CapabilityId::new("physics.body.create").is_ok());
        assert!(CapabilityId::new("World.Read").is_err());
        assert!(CapabilityId::new("filesystem").is_err());

        let mut grants = CapabilitySet::new();
        let read = CapabilityId::new("world.read").unwrap();
        assert!(grants.grant(read.clone()));
        assert!(!grants.grant(read.clone()));
        assert!(grants.contains(&read));
        assert_eq!(grants.len(), 1);
        assert!(grants.revoke(&read));
        assert!(!grants.contains(&read));
    }

    #[test]
    fn api_features_have_domain_names() {
        assert!(ApiFeatureId::new("physics.vehicle.suspension").is_ok());
        assert!(ApiFeatureId::new("invalid").is_err());
    }

    #[test]
    fn api_ranges_use_inclusive_minimum_exclusive_maximum() {
        let v = ApiVersion::new(1, 4, 2);
        let range = ApiVersionRange::new(ApiVersion::new(1, 2, 0), ApiVersion::new(2, 0, 0))
            .unwrap();
        assert!(range.contains(v));
        assert!(!range.contains(ApiVersion::new(2, 0, 0)));
        assert!(!range.contains(ApiVersion::new(1, 1, 9)));
    }

    #[test]
    fn opaque_handles_include_generation_and_kind() {
        let h = Handle::new(42, 3, HandleKind::Entity);
        assert_eq!(h.id(), 42);
        assert_eq!(h.generation(), 3);
        assert_eq!(h.kind(), HandleKind::Entity);
    }

    #[test]
    fn diagnostics_are_structured() {
        let d = Diagnostic::new(
            DiagnosticCode::CapabilityDenied,
            Severity::Warning,
            "Capability denied",
        )
        .with_subject("mod.foo")
        .with_detail("world.read");
        assert_eq!(d.code(), DiagnosticCode::CapabilityDenied);
        assert_eq!(d.subject(), Some("mod.foo"));
        assert_eq!(d.detail(), Some("world.read"));
    }
}
