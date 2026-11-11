//! HNMod's engine-independent foundation.
//!
//! This crate deliberately does not depend on Horizon internals. The native
//! .hnmod adapter will live in a separate integration layer after its exact
//! upstream API and mixin format have been pinned and verified.

pub mod manifest;
pub mod registry;

pub use manifest::{ModuleDescriptor, ProjectManifest, ProjectPackage};
pub use registry::{ModuleRegistry, RegistryError};
