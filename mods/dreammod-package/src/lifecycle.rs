//! Pure, default-disabled lifecycle policy for validated DreamMod packages.
//! This module does not execute scripts or install files.

use crate::{CapabilityDenied, CapabilityGate, CapabilityPlanError, Manifest, ValidatedPackage};

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
            .map_err(|denied: CapabilityDenied| {
                LifecycleError::Approval(CapabilityPlanError::UnknownCapability(denied.0))
            })
    }

    fn invalid_transition(&self, operation: &'static str) -> LifecycleError {
        LifecycleError::InvalidTransition {
            from: self.state,
            operation,
        }
    }
}
