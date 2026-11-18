//! Pure, default-disabled lifecycle policy for validated DreamMod packages.
//! This module does not execute scripts or install files.

use crate::{CapabilityGate, CapabilityPlanError, Manifest, ValidatedPackage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    AwaitingApproval,
    Ready,
    Enabled,
    Disabled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidManifest,
    InvalidTransition { from: LifecycleState, operation: &'static str },
    Approval(CapabilityPlanError),
    NotEnabled,
    CapabilityDenied(String),
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
        self.gate = None;
        self.failure = Some(reason.into());
        self.state = LifecycleState::Failed;
        Ok(())
    }

    /// Enforce both activation state and the per-mod capability grant.
    pub fn require(&self, capability: &str) -> Result<(), LifecycleError> {
        if self.state != LifecycleState::Enabled {
            return Err(LifecycleError::NotEnabled);
        }
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
