//! Pure, default-disabled lifecycle policy for validated DreamMod packages.
//! This module does not execute scripts or install files.

use crate::{
    bridge::{PlayerSnapshot, PlayerSnapshotBatch, SnapshotError, PLAYER_READ_CAPABILITY},
    CapabilityGate, CapabilityPlanError, Manifest, ValidatedPackage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    AwaitingApproval,
    Ready,
    Enabled,
    Disabled,
    Failed,
}

pub const MAX_FAILURE_REASON_BYTES: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidManifest,
    InvalidTransition { from: LifecycleState, operation: &'static str },
    Approval(CapabilityPlanError),
    NotEnabled,
    CapabilityDenied(String),
    FailureReasonTooLong,
    InvalidSnapshot(SnapshotError),
}

/// Tracks one package's explicit approval and activation state.
/// A package starts disabled; it can never become enabled merely by being parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModLifecycle {
    manifest: Manifest,
    state: LifecycleState,
    gate: Option<CapabilityGate>,
    failure: Option<String>,
}

impl ModLifecycle {
    pub fn from_validated_package(package: &ValidatedPackage) -> Self {
        Self {
            manifest: package.manifest.clone(),
            state: LifecycleState::AwaitingApproval,
            gate: None,
            failure: None,
        }
    }

    pub fn state(&self) -> LifecycleState {
        self.state
    }

    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }

    /// Apply explicit approvals. Even full approval only moves to Ready; it does not enable.
    pub fn approve(&mut self, approved: &[String]) -> Result<(), LifecycleError> {
        if self.state != LifecycleState::AwaitingApproval {
            return Err(self.invalid_transition("approve"));
        }
        let gate = CapabilityGate::from_approvals(&self.manifest, approved)
            .map_err(LifecycleError::Approval)?;
        self.gate = Some(gate);
        self.state = LifecycleState::Ready;
        Ok(())
    }

    /// Explicit activation is a separate operation from granting permissions.
    pub fn enable(&mut self) -> Result<(), LifecycleError> {
        if !matches!(self.state, LifecycleState::Ready | LifecycleState::Disabled) {
            return Err(self.invalid_transition("enable"));
        }
        self.state = LifecycleState::Enabled;
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), LifecycleError> {
        if self.state != LifecycleState::Enabled {
            return Err(self.invalid_transition("disable"));
        }
        self.state = LifecycleState::Disabled;
        Ok(())
    }

    /// A failed mod is fail-closed: its gate is discarded and it cannot be re-enabled.
    pub fn fail(&mut self, reason: impl Into<String>) -> Result<(), LifecycleError> {
        if self.state == LifecycleState::Failed {
            return Err(self.invalid_transition("fail"));
        }
        let reason = reason.into();
        if reason.len() > MAX_FAILURE_REASON_BYTES || reason.chars().any(char::is_control) {
            return Err(LifecycleError::FailureReasonTooLong);
        }
        self.gate = None;
        self.failure = Some(reason);
        self.state = LifecycleState::Failed;
        Ok(())
    }

    /// Check activation state without requiring a manifest capability.
    /// Used for the mod's own entrypoint and lifecycle-owned operations.
    pub fn ensure_enabled(&self) -> Result<(), LifecycleError> {
        if self.state == LifecycleState::Enabled {
            Ok(())
        } else {
            Err(LifecycleError::NotEnabled)
        }
    }

    /// Enforce both activation state and the per-mod capability grant.
    pub fn require(&self, capability: &str) -> Result<(), LifecycleError> {
        self.ensure_enabled()?;
        self.gate
            .as_ref()
            .ok_or(LifecycleError::NotEnabled)?
            .require(capability)
            .map_err(|denied| LifecycleError::CapabilityDenied(denied.0))
    }

    fn invalid_transition(&self, operation: &'static str) -> LifecycleError {
        LifecycleError::InvalidTransition {
            from: self.state,
            operation,
        }
    }
}


/// The only supported entry point for a host-side privileged operation.
/// The callback is never invoked unless the mod is enabled and the capability
/// is explicitly granted. Horizon adapters should route every privileged API
/// operation through this boundary rather than exposing raw host handles.
pub struct HostAdapter<'a> {
    lifecycle: &'a ModLifecycle,
}

impl<'a> HostAdapter<'a> {
    pub fn new(lifecycle: &'a ModLifecycle) -> Self {
        Self { lifecycle }
    }

    pub fn invoke<T>(
        &self,
        capability: &str,
        operation: impl FnOnce() -> T,
    ) -> Result<T, LifecycleError> {
        self.lifecycle.require(capability)?;
        Ok(operation())
    }

    /// Read one owned player snapshot. The Horizon callback runs only after
    /// the explicit entities.read grant has been checked.
    pub fn read_player_snapshot(
        &self,
        operation: impl FnOnce() -> Result<PlayerSnapshot, SnapshotError>,
    ) -> Result<PlayerSnapshot, LifecycleError> {
        self.lifecycle.require(PLAYER_READ_CAPABILITY)?;
        operation().map_err(LifecycleError::InvalidSnapshot)
    }

    /// Read a bounded player batch with one capability check and one host crossing.
    pub fn read_player_snapshots(
        &self,
        operation: impl FnOnce() -> Result<PlayerSnapshotBatch, SnapshotError>,
    ) -> Result<PlayerSnapshotBatch, LifecycleError> {
        self.lifecycle.require(PLAYER_READ_CAPABILITY)?;
        operation().map_err(LifecycleError::InvalidSnapshot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiCompatibility;
    use std::collections::BTreeMap;

    fn package() -> ValidatedPackage {
        ValidatedPackage {
            manifest: Manifest {
                schema_version: 1,
                id: "example.lifecycle".into(),
                name: "Lifecycle test".into(),
                version: "1.0.0".into(),
                api: ApiCompatibility {
                    min: "0.2.0".into(),
                    max: "0.3.0".into(),
                },
                entrypoint: "scripts/main.dream".into(),
                capabilities: vec!["world.read".into()],
                dependencies: BTreeMap::new(),
            },
            entries: Vec::new(),
            files: Vec::new(),
        }
    }


    #[test]
    fn host_adapter_never_calls_operation_without_authorization() {
        use std::cell::Cell;
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        let calls = Cell::new(0);
        let denied = HostAdapter::new(&lifecycle).invoke("world.read", || calls.set(calls.get() + 1));
        assert_eq!(denied, Err(LifecycleError::NotEnabled));
        assert_eq!(calls.get(), 0);

        lifecycle.approve(&["world.read".into()]).unwrap();
        lifecycle.enable().unwrap();
        let result = HostAdapter::new(&lifecycle)
            .invoke("world.read", || {
                calls.set(calls.get() + 1);
                42
            })
            .unwrap();
        assert_eq!(result, 42);
        assert_eq!(calls.get(), 1);

        let denied = HostAdapter::new(&lifecycle)
            .invoke("world.write", || calls.set(calls.get() + 1));
        assert_eq!(denied, Err(LifecycleError::CapabilityDenied("world.write".into())));
        assert_eq!(calls.get(), 1);

        lifecycle.disable().unwrap();
        let denied = HostAdapter::new(&lifecycle)
            .invoke("world.read", || calls.set(calls.get() + 1));
        assert_eq!(denied, Err(LifecycleError::NotEnabled));
        assert_eq!(calls.get(), 1);

        lifecycle.enable().unwrap();
        lifecycle.fail("script fault").unwrap();
        let denied = HostAdapter::new(&lifecycle)
            .invoke("world.read", || calls.set(calls.get() + 1));
        assert_eq!(denied, Err(LifecycleError::NotEnabled));
        assert_eq!(calls.get(), 1);
    }


    #[test]
    fn player_snapshot_adapter_checks_permission_before_reading() {
        use std::cell::Cell;

        let mut package = package();
        package.manifest.capabilities = vec![PLAYER_READ_CAPABILITY.into()];
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        let calls = Cell::new(0);
        let denied = HostAdapter::new(&lifecycle).read_player_snapshot(|| {
            calls.set(calls.get() + 1);
            PlayerSnapshot::new(7, "Dreamer_7".into(), "minecraft:overworld".into(), [1.0, 64.0, 2.0])
        });
        assert_eq!(denied, Err(LifecycleError::NotEnabled));
        assert_eq!(calls.get(), 0);

        lifecycle.approve(&[PLAYER_READ_CAPABILITY.into()]).unwrap();
        lifecycle.enable().unwrap();
        let snapshot = HostAdapter::new(&lifecycle)
            .read_player_snapshot(|| {
                calls.set(calls.get() + 1);
                PlayerSnapshot::new(7, "Dreamer_7".into(), "minecraft:overworld".into(), [1.0, 64.0, 2.0])
            })
            .unwrap();
        assert_eq!(snapshot.name(), "Dreamer_7");
        assert_eq!(calls.get(), 1);

        let invalid = HostAdapter::new(&lifecycle).read_player_snapshot(|| {
            PlayerSnapshot::new(7, "bad name".into(), "minecraft:overworld".into(), [1.0, 64.0, 2.0])
        });
        assert_eq!(invalid, Err(LifecycleError::InvalidSnapshot(SnapshotError::InvalidPlayerName)));
    }


    #[test]
    fn player_snapshot_batch_checks_permission_once_before_host_read() {
        use std::cell::Cell;

        let mut package = package();
        package.manifest.capabilities = vec![PLAYER_READ_CAPABILITY.into()];
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        let calls = Cell::new(0);
        let denied = HostAdapter::new(&lifecycle).read_player_snapshots(|| {
            calls.set(calls.get() + 1);
            PlayerSnapshotBatch::new(Vec::new())
        });
        assert_eq!(denied, Err(LifecycleError::NotEnabled));
        assert_eq!(calls.get(), 0);

        lifecycle.approve(&[PLAYER_READ_CAPABILITY.into()]).unwrap();
        lifecycle.enable().unwrap();
        let batch = HostAdapter::new(&lifecycle)
            .read_player_snapshots(|| {
                calls.set(calls.get() + 1);
                let player = PlayerSnapshot::new(
                    7,
                    "Dreamer_7".into(),
                    "minecraft:overworld".into(),
                    [0.0; 3],
                )?;
                PlayerSnapshotBatch::new(vec![player])
            })
            .unwrap();
        assert_eq!(batch.len(), 1);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn failure_reason_is_bounded_and_control_free() {
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        assert_eq!(
            lifecycle.fail("x".repeat(MAX_FAILURE_REASON_BYTES + 1)),
            Err(LifecycleError::FailureReasonTooLong)
        );
        assert_eq!(lifecycle.state(), LifecycleState::AwaitingApproval);
        assert_eq!(lifecycle.fail("bad\nreason"), Err(LifecycleError::FailureReasonTooLong));
        assert_eq!(lifecycle.state(), LifecycleState::AwaitingApproval);
    }

    #[test]
    fn approval_does_not_automatically_enable_a_mod() {
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        assert_eq!(lifecycle.state(), LifecycleState::AwaitingApproval);
        assert_eq!(lifecycle.require("world.read"), Err(LifecycleError::NotEnabled));

        lifecycle.approve(&["world.read".into()]).unwrap();
        assert_eq!(lifecycle.state(), LifecycleState::Ready);
        assert_eq!(lifecycle.require("world.read"), Err(LifecycleError::NotEnabled));
    }

    #[test]
    fn activation_and_capability_checks_are_both_required() {
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        lifecycle.approve(&["world.read".into()]).unwrap();
        lifecycle.enable().unwrap();

        assert_eq!(lifecycle.state(), LifecycleState::Enabled);
        assert_eq!(lifecycle.require("world.read"), Ok(()));
        assert_eq!(
            lifecycle.require("world.write"),
            Err(LifecycleError::CapabilityDenied("world.write".into()))
        );

        lifecycle.disable().unwrap();
        assert_eq!(lifecycle.require("world.read"), Err(LifecycleError::NotEnabled));
    }

    #[test]
    fn failed_mod_loses_capabilities_and_cannot_be_reenabled() {
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        lifecycle.approve(&["world.read".into()]).unwrap();
        lifecycle.enable().unwrap();
        lifecycle.fail("script fault").unwrap();

        assert_eq!(lifecycle.state(), LifecycleState::Failed);
        assert_eq!(lifecycle.failure(), Some("script fault"));
        assert_eq!(lifecycle.require("world.read"), Err(LifecycleError::NotEnabled));
        assert!(matches!(
            lifecycle.enable(),
            Err(LifecycleError::InvalidTransition { .. })
        ));
    }

    #[test]
    fn rejects_invalid_transitions_and_unrequested_approvals() {
        let package = package();
        let mut lifecycle = ModLifecycle::from_validated_package(&package);
        assert!(matches!(
            lifecycle.enable(),
            Err(LifecycleError::InvalidTransition { .. })
        ));
        assert!(matches!(
            lifecycle.approve(&["world.write".into()]),
            Err(LifecycleError::Approval(CapabilityPlanError::UnrequestedCapability(_)))
        ));
        assert_eq!(lifecycle.state(), LifecycleState::AwaitingApproval);
    }

}
