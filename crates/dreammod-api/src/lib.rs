//! Engine-neutral mod registration contracts.
//!
//! This crate stores validated registrations. It does not execute scripts or call engine systems.

mod registration;

pub use registration::{Registration, RegistrationKind, RegistrationRegistry, RegistryError};

#[cfg(test)]
mod tests {
    use super::*;
    use dreammod_contract::{ApiFeatureId, ModId, ResourceId};

    fn owner() -> ModId {
        ModId::new("cars").unwrap()
    }

    fn registration(path: &str, kind: RegistrationKind, priority: i32) -> Registration {
        Registration::new(
            owner(),
            ResourceId::new("cars", path).unwrap(),
            kind,
            priority,
        )
        .unwrap()
    }

    #[test]
    fn batch_registration_is_atomic_and_namespace_owned() {
        let mut registry = RegistrationRegistry::new();
        let good = registration("command/drive", RegistrationKind::Command, 0);
        let duplicate = registration("command/drive", RegistrationKind::EventListener, 10);

        assert_eq!(
            registry.register_batch(&owner(), vec![good.clone(), duplicate]),
            Err(RegistryError::DuplicateId)
        );
        assert!(registry.is_empty());

        assert_eq!(registry.register_batch(&owner(), vec![good]), Ok(1));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn foreign_owner_and_foreign_namespace_are_rejected() {
        let mut registry = RegistrationRegistry::new();
        let wrong_owner = ModId::new("other").unwrap();
        let entry = registration("system/tick", RegistrationKind::System, 0);
        assert_eq!(
            registry.register_batch(&wrong_owner, vec![entry]),
            Err(RegistryError::OwnerMismatch)
        );

        let foreign_id = ResourceId::new("other", "command/x").unwrap();
        let foreign = Registration::new(owner(), foreign_id, RegistrationKind::Command, 0);
        assert_eq!(foreign, Err(RegistryError::NamespaceMismatch));
    }

    #[test]
    fn snapshots_have_deterministic_priority_order() {
        let mut registry = RegistrationRegistry::new();
        let entries = vec![
            registration("system/low", RegistrationKind::System, -1),
            registration("system/high", RegistrationKind::System, 5),
            registration("system/mid", RegistrationKind::System, 1),
        ];
        assert_eq!(registry.register_batch(&owner(), entries), Ok(3));
        let snapshot = registry.snapshot(Some(RegistrationKind::System));
        let names: Vec<_> = snapshot.iter().map(|r| r.id().path()).collect();
        assert_eq!(names, vec!["system/high", "system/mid", "system/low"]);
    }

    #[test]
    fn disabling_owner_removes_all_owned_registrations() {
        let mut registry = RegistrationRegistry::new();
        let entries = vec![
            registration("command/a", RegistrationKind::Command, 0),
            registration("event/b", RegistrationKind::EventListener, 0),
        ];
        assert_eq!(registry.register_batch(&owner(), entries), Ok(2));
        assert_eq!(registry.unregister_owner(&owner()), 2);
        assert!(registry.is_empty());
    }

    #[test]
    fn required_feature_is_preserved_in_registration() {
        let feature = ApiFeatureId::new("physics.vehicle.suspension").unwrap();
        let entry = registration("provider/suspension", RegistrationKind::Provider, 3)
            .with_required_feature(feature.clone());
        assert_eq!(entry.required_feature(), Some(&feature));
    }
}
