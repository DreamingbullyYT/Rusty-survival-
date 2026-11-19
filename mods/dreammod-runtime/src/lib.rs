//! DreamMod's player-side mod runtime.
//!
//! .dmod packages contain source and assets only. This crate parses package
//! metadata, validates archives, interprets source through a pluggable engine,
//! and returns capability-checked command batches for the Horizon adapter.
//! It never compiles mod source to native code or bytecode.

pub mod manifest;
pub mod package;
pub mod runtime;
pub mod script;

pub use manifest::{Capability, ModDependency, ModId, ModManifest, ModSide};
pub use package::{
    DiscoveredDmod, DmodPackage, ModsDirectoryLimits, ModsDirectoryScan, ModsDirectoryScanner,
    PackageError, PackageLimits, PackageLoader, ScanDirectoryError, ScanIssue,
};
pub use runtime::{
    AllowAllForTests, AuthorizedCommand, CommandAuthorizer, CommandEnvelope, DeniedOperation,
    DenyAll, DispatchReport, DreamModRuntime, EngineCommandHost, HostDispatchReport, LoadReport, ModsDirectoryLoadReport,
    PermissionPolicy, ProviderMode, ProviderRegistration, RegisteredProvider, RuntimeConfig,
    RuntimeError, RuntimeEvent, RuntimeSide, ScriptFailure,
};
pub use script::{ExecutionLimits, RhaiEngine, ScriptEngine, ScriptError};
