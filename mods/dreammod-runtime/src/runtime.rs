use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
};

use semver::Version;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

use crate::{
    api_router::{
        ApiProvider, ApiProviderPreference, ApiRequest, ApiRouter, ApiRouterError,
        ApiScriptRequest, ApiScriptResult, ApiScriptResultError,
    },
    manifest::{Capability, ManifestError, ModId, ModManifest, ModSide},
    package::{DmodPackage, ModsDirectoryScanner, ScanDirectoryError, ScanIssue},
    script::{ScriptEngine, ScriptError},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeSide {
    Client,
    Server,
}

#[derive(Clone, Debug)]
pub struct RuntimeEvent {
    pub name: String,
    pub side: RuntimeSide,
    pub tick: Option<u64>,
    pub payload: Value,
}

impl RuntimeEvent {
    pub fn new(name: impl Into<String>, side: RuntimeSide, payload: Value) -> Self {
        Self {
            name: name.into(),
            side,
            tick: None,
            payload,
        }
    }

    pub fn at_tick(mut self, tick: u64) -> Self {
        self.tick = Some(tick);
        self
    }

    fn function_name(&self) -> String {
        format!("on_{}", self.name)
    }
}

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    pub api_version: Version,
    pub max_failures_before_disable: u32,
    pub max_commands_per_event: usize,
    pub max_providers_per_event: usize,
    pub max_output_bytes: usize,
    pub max_command_payload_bytes: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            api_version: Version::new(0, 1, 0),
            max_failures_before_disable: 3,
            max_commands_per_event: 256,
            max_providers_per_event: 16,
            max_output_bytes: 256 * 1024,
            max_command_payload_bytes: 32 * 1024,
        }
    }
}

pub trait PermissionPolicy {
    fn granted_capabilities(&self, manifest: &ModManifest) -> BTreeSet<Capability>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DenyAll;

impl PermissionPolicy for DenyAll {
    fn granted_capabilities(&self, _manifest: &ModManifest) -> BTreeSet<Capability> {
        BTreeSet::new()
    }
}

/// Convenience policy for unit tests only. Production Core must use launcher/user grants.
#[derive(Clone, Copy, Debug, Default)]
pub struct AllowAllForTests;

impl PermissionPolicy for AllowAllForTests {
    fn granted_capabilities(&self, manifest: &ModManifest) -> BTreeSet<Capability> {
        manifest.requested_capabilities()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CommandEnvelope {
    pub namespace: String,
    pub name: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderMode {
    Replace,
    Before,
    After,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProviderRegistration {
    pub subsystem: String,
    pub function: String,
    #[serde(default)]
    pub priority: i32,
    pub mode: ProviderMode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuthorizedCommand {
    pub mod_id: ModId,
    pub command: CommandEnvelope,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegisteredProvider {
    pub mod_id: ModId,
    pub registration: ProviderRegistration,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeniedOperation {
    pub mod_id: ModId,
    pub operation: String,
    pub reason: String,
}

pub trait CommandAuthorizer {
    /// Return the capability required by a command. Unknown commands must return None.
    fn required_capability(&self, command: &CommandEnvelope) -> Option<Capability>;

    /// Return the capability required to register/replace a subsystem provider.
    /// Unknown subsystems must return None.
    fn required_provider_capability(&self, provider: &ProviderRegistration) -> Option<Capability>;
}

#[derive(Clone, Debug, Default)]
pub struct LoadReport {
    pub loaded: Vec<ModId>,
}

/// Result of discovering packages in Horizon's shared mods directory and loading
/// every validated package into this runtime.
#[derive(Clone, Debug, Default)]
pub struct ModsDirectoryLoadReport {
    pub load: LoadReport,
    pub issues: Vec<ScanIssue>,
    pub ignored_hmod_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DispatchReport {
    pub commands: Vec<AuthorizedCommand>,
    pub providers: Vec<RegisteredProvider>,
    pub denied: Vec<DeniedOperation>,
    pub failures: Vec<ScriptFailure>,
}

/// Engine boundary for applying an already-authorized command batch.
///
/// Implementations must validate the complete batch before committing mutations;
/// a failure must not leave a partially-applied batch. Native subsystems remain
/// responsible for authoritative state validation.
pub trait EngineCommandHost {
    type Error;

    fn execute_atomic_batch(&mut self, commands: &[AuthorizedCommand]) -> Result<(), Self::Error>;
}

/// Dispatch result plus optional native-host execution result.
/// The host is not called when dispatch produced no authorized commands.
pub struct HostDispatchReport<E> {
    pub dispatch: DispatchReport,
    pub execution: Option<Result<(), E>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptFailure {
    pub mod_id: ModId,
    pub function: String,
    pub message: String,
    pub disabled: bool,
}

struct LoadedMod<I> {
    manifest: ModManifest,
    script: I,
    granted: BTreeSet<Capability>,
    consecutive_failures: u32,
    enabled: bool,
    providers: BTreeMap<String, ProviderRegistration>,
    pending_api_results: Vec<ApiScriptResult>,
}

pub struct DreamModRuntime<E: ScriptEngine> {
    engine: E,
    config: RuntimeConfig,
    mods: BTreeMap<ModId, LoadedMod<E::Instance>>,
    load_order: Vec<ModId>,
    api_router: ApiRouter,
}

impl<E: ScriptEngine> DreamModRuntime<E> {
    pub fn new(engine: E, config: RuntimeConfig) -> Self {
        Self {
            engine,
            config,
            mods: BTreeMap::new(),
            load_order: Vec::new(),
            api_router: ApiRouter::default(),
        }
    }

    /// Scan the shared Horizon mods folder using default limits, then load
    /// every valid .dmod package. Horizon .hmod files are left to Horizon.
    pub fn load_from_horizon_mods_dir<P: PermissionPolicy>(
        &mut self,
        directory: impl AsRef<std::path::Path>,
        policy: &P,
    ) -> Result<ModsDirectoryLoadReport, RuntimeError> {
        self.load_from_horizon_mods_dir_with_scanner(
            directory,
            &ModsDirectoryScanner::default(),
            policy,
        )
    }

    /// Same as the default entrypoint, but lets the caller provide explicit
    /// scanner/package limits for testing or a host policy.
    pub fn load_from_horizon_mods_dir_with_scanner<P: PermissionPolicy>(
        &mut self,
        directory: impl AsRef<std::path::Path>,
        scanner: &ModsDirectoryScanner,
        policy: &P,
    ) -> Result<ModsDirectoryLoadReport, RuntimeError> {
        let scan = scanner
            .scan(directory)
            .map_err(RuntimeError::ScanDirectory)?;
        let packages = scan
            .packages
            .into_iter()
            .map(|discovered| discovered.package)
            .collect();
        let issues = scan.issues;
        let ignored_hmod_count = scan.ignored_hmod_count;
        let load = self.load_packages(packages, policy)?;
        Ok(ModsDirectoryLoadReport {
            load,
            issues,
            ignored_hmod_count,
        })
    }

    pub fn load_packages<P: PermissionPolicy>(
        &mut self,
        packages: Vec<DmodPackage>,
        policy: &P,
    ) -> Result<LoadReport, RuntimeError> {
        if !self.mods.is_empty() {
            return Err(RuntimeError::AlreadyLoaded);
        }

        let order = resolve_load_order(&packages, &self.config.api_version)?;
        let mut packages_by_id = packages
            .into_iter()
            .map(|package| (package.manifest.id.clone(), package))
            .collect::<BTreeMap<_, _>>();
        let mut staged = Vec::with_capacity(order.len());

        for id in &order {
            let package = packages_by_id
                .remove(id)
                .ok_or_else(|| RuntimeError::MissingPackage(id.clone()))?;
            let manifest = package.manifest;
            let requested = manifest.requested_capabilities();
            let granted = policy
                .granted_capabilities(&manifest)
                .intersection(&requested)
                .copied()
                .collect::<BTreeSet<_>>();

            for capability in &manifest.required_capabilities {
                if !granted.contains(capability) {
                    return Err(RuntimeError::RequiredCapabilityDenied {
                        mod_id: id.clone(),
                        capability: *capability,
                    });
                }
            }

            let program = self.engine.parse_source(&package.entry_source)?;
            let script = self.engine.instantiate(program);
            staged.push((
                id.clone(),
                LoadedMod {
                    manifest,
                    script,
                    granted,
                    consecutive_failures: 0,
                    enabled: false,
                    providers: BTreeMap::new(),
                    pending_api_results: Vec::new(),
                },
            ));
        }

        let mut loaded = Vec::with_capacity(staged.len());
        for (id, instance) in staged {
            self.mods.insert(id.clone(), instance);
            self.load_order.push(id.clone());
            loaded.push(id);
        }

        Ok(LoadReport { loaded })
    }

    pub fn dispatch_event<A: CommandAuthorizer>(
        &mut self,
        event: &RuntimeEvent,
        authorizer: &A,
    ) -> DispatchReport {
        self.dispatch_event_inner(event, authorizer, false)
    }

    /// Dispatch an event and pass its authorized commands to the engine host.
    /// Authorization and response validation happen before the host sees data.
    pub fn dispatch_event_and_execute<A: CommandAuthorizer, H: EngineCommandHost>(
        &mut self,
        event: &RuntimeEvent,
        authorizer: &A,
        host: &mut H,
    ) -> HostDispatchReport<H::Error> {
        let dispatch = self.dispatch_event(event, authorizer);
        let execution = if dispatch.commands.is_empty() {
            None
        } else {
            Some(host.execute_atomic_batch(&dispatch.commands))
        };
        HostDispatchReport {
            dispatch,
            execution,
        }
    }

    /// Lifecycle callbacks such as on_load/on_unload are dispatched regardless
    /// of the regular event subscription list in manifest.json.
    pub fn dispatch_lifecycle_event<A: CommandAuthorizer>(
        &mut self,
        event: &RuntimeEvent,
        authorizer: &A,
    ) -> DispatchReport {
        self.dispatch_event_inner(event, authorizer, true)
    }

    fn dispatch_event_inner<A: CommandAuthorizer>(
        &mut self,
        event: &RuntimeEvent,
        authorizer: &A,
        lifecycle: bool,
    ) -> DispatchReport {
        let mut report = DispatchReport::default();
        let function = event.function_name();
        let ids = self.load_order.clone();
        let engine = &self.engine;
        let config = &self.config;
        let api_router = &mut self.api_router;

        for id in ids {
            let Some(instance) = self.mods.get_mut(&id) else {
                continue;
            };
            if !instance.enabled
                || !side_matches(instance.manifest.side, event.side)
                || (!lifecycle && !instance.manifest.events.contains(&event.name))
            {
                continue;
            }

            let event_payload = with_api_results(
                &event.payload,
                std::mem::take(&mut instance.pending_api_results),
            );
            match engine.call(&mut instance.script, &function, &event_payload) {
                Ok(None) => {
                    instance.consecutive_failures = 0;
                }
                Ok(Some(value)) => {
                    let result = collect_response(
                        &id,
                        &instance.granted,
                        value,
                        authorizer,
                        config,
                        &mut instance.providers,
                        &mut report,
                    );
                    match result {
                        Ok(api_requests) => {
                            instance.pending_api_results = resolve_script_api_requests(
                                &instance.granted,
                                api_requests,
                                authorizer,
                                config,
                                api_router,
                            );
                            instance.consecutive_failures = 0;
                        },
                        Err(message) => {
                            let failure = record_failure(
                                &id,
                                &function,
                                message,
                                &mut instance.consecutive_failures,
                                &mut instance.enabled,
                                config.max_failures_before_disable,
                            );
                            if failure.disabled {
                                instance.providers.clear();
                            }
                            report.failures.push(failure);
                        }
                    }
                }
                Err(error) => {
                    let failure = record_failure(
                        &id,
                        &function,
                        error.to_string(),
                        &mut instance.consecutive_failures,
                        &mut instance.enabled,
                        config.max_failures_before_disable,
                    );
                    if failure.disabled {
                        instance.providers.clear();
                    }
                    report.failures.push(failure);
                }
            }
        }

        report
    }

    pub fn invoke_provider(
        &mut self,
        mod_id: &ModId,
        subsystem: &str,
        input: &Value,
    ) -> Result<Option<Value>, ScriptFailure> {
        let Some(instance) = self.mods.get_mut(mod_id) else {
            return Err(ScriptFailure {
                mod_id: mod_id.clone(),
                function: subsystem.to_owned(),
                message: "mod is not loaded".to_owned(),
                disabled: false,
            });
        };
        if !instance.enabled {
            return Err(ScriptFailure {
                mod_id: mod_id.clone(),
                function: subsystem.to_owned(),
                message: "mod is disabled".to_owned(),
                disabled: true,
            });
        }
        let Some(provider) = instance.providers.get(subsystem) else {
            return Ok(None);
        };

        let function = provider.function.clone();
        match self.engine.call(&mut instance.script, &function, input) {
            Ok(result) => {
                instance.consecutive_failures = 0;
                Ok(result)
            }
            Err(error) => {
                let failure = record_failure(
                    mod_id,
                    &function,
                    error.to_string(),
                    &mut instance.consecutive_failures,
                    &mut instance.enabled,
                    self.config.max_failures_before_disable,
                );
                if failure.disabled {
                    instance.providers.clear();
                    instance.pending_api_results.clear();
                }
                Err(failure)
            }
        }
    }

    /// Register or hot-replace a shared API provider. Providers advertise supported
    /// operations; mods are routed without binding to Horizon or engine internals.
    pub fn register_api_provider(
        &mut self,
        provider: Box<dyn ApiProvider>,
    ) -> Result<(), ApiRouterError> {
        self.api_router.register_provider(provider)
    }

    pub fn unregister_api_provider(&mut self, provider_id: &str) -> Result<(), ApiRouterError> {
        self.api_router.unregister_provider(provider_id)
    }

    pub fn set_api_provider_preference(
        &mut self,
        domain: impl Into<String>,
        preference: ApiProviderPreference,
    ) -> Result<(), ApiRouterError> {
        self.api_router.set_preference(domain, preference)
    }

    /// Calls the shared API on behalf of a loaded, enabled mod. Every operation
    /// must be recognized by the authorizer and its capability must be granted.
    pub fn call_api<A: CommandAuthorizer>(
        &mut self,
        mod_id: &ModId,
        request: ApiRequest,
        authorizer: &A,
    ) -> Result<Value, ApiCallError> {
        if !crate::api_router::ApiOperation::new(
            request.operation.domain.clone(),
            request.operation.name.clone(),
        )
        .is_ok()
        {
            return Err(ApiCallError::InvalidOperation);
        }
        let input_bytes = serde_json::to_vec(&request.input)
            .map_err(|_| ApiCallError::InvalidOperation)?
            .len();
        if input_bytes > self.config.max_command_payload_bytes {
            return Err(ApiCallError::PayloadTooLarge);
        }
        let instance = self.mods.get(mod_id).ok_or(ApiCallError::ModNotLoaded)?;
        if !instance.enabled {
            return Err(ApiCallError::ModDisabled);
        }
        let command = CommandEnvelope {
            namespace: request.operation.domain.clone(),
            name: request.operation.name.clone(),
            payload: request.input.clone(),
        };
        let required = authorizer
            .required_capability(&command)
            .ok_or(ApiCallError::UnknownOperation)?;
        if !instance.granted.contains(&required) {
            return Err(ApiCallError::CapabilityDenied(required));
        }
        let response = self.api_router.call(request).map_err(ApiCallError::Router)?;
        let response_bytes = serde_json::to_vec(&response)
            .map_err(|_| ApiCallError::InvalidOperation)?
            .len();
        if response_bytes > self.config.max_output_bytes {
            return Err(ApiCallError::ResponseTooLarge);
        }
        Ok(response)
    }

    pub fn api_providers(&self) -> Vec<crate::api_router::ApiProviderDescriptor> {
        self.api_router.providers()
    }

    /// Explicitly activate a mod after package loading and permission approval.
    /// Loading never executes callbacks; callers must opt in per mod.
    pub fn enable_mod(&mut self, mod_id: &ModId) -> bool {
        if let Some(instance) = self.mods.get_mut(mod_id) {
            instance.enabled = true;
            instance.consecutive_failures = 0;
            true
        } else {
            false
        }
    }

    pub fn disable_mod(&mut self, mod_id: &ModId) -> bool {
        if let Some(instance) = self.mods.get_mut(mod_id) {
            instance.enabled = false;
            instance.providers.clear();
            instance.pending_api_results.clear();
            true
        } else {
            false
        }
    }

    pub fn unload_mod(&mut self, mod_id: &ModId) -> bool {
        let removed = self.mods.remove(mod_id).is_some();
        if removed {
            self.load_order.retain(|id| id != mod_id);
        }
        removed
    }

    pub fn loaded_mods(&self) -> &[ModId] {
        &self.load_order
    }

    pub fn is_enabled(&self, mod_id: &ModId) -> bool {
        self.mods
            .get(mod_id)
            .is_some_and(|instance| instance.enabled)
    }
}

fn collect_response<A: CommandAuthorizer>(
    mod_id: &ModId,
    granted: &BTreeSet<Capability>,
    value: Value,
    authorizer: &A,
    config: &RuntimeConfig,
    providers: &mut BTreeMap<String, ProviderRegistration>,
    report: &mut DispatchReport,
) -> Result<Vec<ApiScriptRequest>, String> {
    let encoded = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    if encoded.len() > config.max_output_bytes {
        return Err(format!(
            "script output is too large: {} bytes (limit {})",
            encoded.len(),
            config.max_output_bytes
        ));
    }

    let response: ScriptResponse = serde_json::from_value(value)
        .map_err(|error| format!("invalid script response: {error}"))?;
    if response.commands.len() > config.max_commands_per_event {
        return Err("script emitted too many commands".to_owned());
    }
    if response.providers.len() > config.max_providers_per_event {
        return Err("script registered too many subsystem providers".to_owned());
    }
    if response.api_requests.len() > config.max_commands_per_event {
        return Err("script emitted too many API requests".to_owned());
    }
    let mut request_ids = BTreeSet::new();
    for request in &response.api_requests {
        request.validate().map_err(|error| error.to_string())?;
        if !request_ids.insert(request.request_id.clone()) {
            return Err(format!("duplicate API request ID: {}", request.request_id));
        }
    }

    for command in response.commands {
        let operation = format!("{}.{}", command.namespace, command.name);
        let payload_bytes = serde_json::to_vec(&command.payload)
            .map_err(|error| format!("invalid command payload: {error}"))?
            .len();
        if payload_bytes > config.max_command_payload_bytes {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: "command payload exceeds the configured limit".to_owned(),
            });
            continue;
        }

        let Some(required) = authorizer.required_capability(&command) else {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: "unknown command".to_owned(),
            });
            continue;
        };
        if !granted.contains(&required) {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: format!("missing capability {required:?}"),
            });
            continue;
        }

        report.commands.push(AuthorizedCommand {
            mod_id: mod_id.clone(),
            command,
        });
    }

    for provider in response.providers {
        let operation = format!("provider:{}", provider.subsystem);
        if !is_safe_identifier(&provider.subsystem)
            || !is_safe_identifier(&provider.function)
            || !(-1_000..=1_000).contains(&provider.priority)
        {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: "invalid provider registration".to_owned(),
            });
            continue;
        }

        let Some(required) = authorizer.required_provider_capability(&provider) else {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: "unknown or unsupported subsystem".to_owned(),
            });
            continue;
        };
        if !granted.contains(&required) {
            report.denied.push(DeniedOperation {
                mod_id: mod_id.clone(),
                operation,
                reason: format!("missing capability {required:?}"),
            });
            continue;
        }

        providers.insert(provider.subsystem.clone(), provider.clone());
        report.providers.push(RegisteredProvider {
            mod_id: mod_id.clone(),
            registration: provider,
        });
    }

    Ok(response.api_requests)
}


/// Inject completed asynchronous API requests into the next script event.
fn with_api_results(payload: &Value, results: Vec<ApiScriptResult>) -> Value {
    let results = serde_json::to_value(results).unwrap_or_else(|_| Value::Array(Vec::new()));
    match payload {
        Value::Object(object) => {
            let mut object = object.clone();
            object.insert("api_results".to_owned(), results);
            Value::Object(object)
        }
        other => serde_json::json!({ "event_payload": other, "api_results": results }),
    }
}

fn resolve_script_api_requests<A: CommandAuthorizer>(
    granted: &BTreeSet<Capability>,
    requests: Vec<ApiScriptRequest>,
    authorizer: &A,
    config: &RuntimeConfig,
    router: &mut ApiRouter,
) -> Vec<ApiScriptResult> {
    let mut results = Vec::with_capacity(requests.len());
    let mut total_bytes = 0_usize;

    for request in requests {
        let operation = request.operation.clone();
        let mut result = ApiScriptResult {
            request_id: request.request_id,
            operation: operation.clone(),
            result: None,
            error: None,
        };
        let payload_size = serde_json::to_vec(&request.input)
            .map_or(usize::MAX, |bytes| bytes.len());
        if payload_size > config.max_command_payload_bytes {
            result.error = Some(ApiScriptResultError::PayloadTooLarge);
        } else {
            let command = CommandEnvelope {
                namespace: operation.domain.clone(),
                name: operation.name.clone(),
                payload: request.input.clone(),
            };
            if let Some(required) = authorizer.required_capability(&command) {
                if !granted.contains(&required) {
                    result.error = Some(ApiScriptResultError::CapabilityDenied);
                } else {
                    let api_request = ApiRequest {
                        operation,
                        safety: request.safety,
                        input: request.input,
                    };
                    match router.call(api_request) {
                        Ok(value) => {
                            let response_size = serde_json::to_vec(&value)
                                .map_or(usize::MAX, |bytes| bytes.len());
                            if response_size > config.max_output_bytes {
                                result.error = Some(ApiScriptResultError::ResponseTooLarge);
                            } else {
                                result.result = Some(value);
                            }
                        }
                        Err(ApiRouterError::UnsupportedOperation(_)) => {
                            result.error = Some(ApiScriptResultError::ProviderUnavailable);
                        }
                        Err(ApiRouterError::ProviderFailed { .. }) => {
                            result.error = Some(ApiScriptResultError::ProviderFailed);
                        }
                        Err(_) => {
                            result.error = Some(ApiScriptResultError::InvalidRequest);
                        }
                    }
                }
            } else {
                result.error = Some(ApiScriptResultError::UnknownOperation);
            }
        }

        let mut encoded_size = serde_json::to_vec(&result)
            .map_or(usize::MAX, |bytes| bytes.len());
        if total_bytes.saturating_add(encoded_size) > config.max_output_bytes {
            result.result = None;
            result.error = Some(ApiScriptResultError::ResponseTooLarge);
            encoded_size = serde_json::to_vec(&result)
                .map_or(usize::MAX, |bytes| bytes.len());
        }
        if total_bytes.saturating_add(encoded_size) <= config.max_output_bytes {
            total_bytes += encoded_size;
            results.push(result);
        }
    }

    results
}

fn record_failure(
    mod_id: &ModId,
    function: &str,
    message: String,
    consecutive_failures: &mut u32,
    enabled: &mut bool,
    threshold: u32,
) -> ScriptFailure {
    *consecutive_failures = consecutive_failures.saturating_add(1);
    if *consecutive_failures >= threshold.max(1) {
        *enabled = false;
    }

    ScriptFailure {
        mod_id: mod_id.clone(),
        function: function.to_owned(),
        message,
        disabled: !*enabled,
    }
}

fn side_matches(mod_side: ModSide, runtime_side: RuntimeSide) -> bool {
    matches!(
        (mod_side, runtime_side),
        (ModSide::Both, _)
            | (ModSide::Client, RuntimeSide::Client)
            | (ModSide::Server, RuntimeSide::Server)
    )
}

fn is_safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn resolve_load_order(
    packages: &[DmodPackage],
    api_version: &Version,
) -> Result<Vec<ModId>, RuntimeError> {
    let mut by_id = BTreeMap::new();
    for package in packages {
        package.manifest.validate()?;
        if !package.manifest.dreammod_api.matches(api_version) {
            return Err(RuntimeError::ApiMismatch {
                mod_id: package.manifest.id.clone(),
                required: package.manifest.dreammod_api.clone(),
                actual: api_version.clone(),
            });
        }
        if by_id.insert(package.manifest.id.clone(), package).is_some() {
            return Err(RuntimeError::DuplicateMod(package.manifest.id.clone()));
        }
    }

    let mut indegree = by_id
        .keys()
        .cloned()
        .map(|id| (id, 0_usize))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing: BTreeMap<ModId, Vec<ModId>> = BTreeMap::new();

    for (id, package) in &by_id {
        for dependency in &package.manifest.dependencies {
            let Some(target) = by_id.get(&dependency.id) else {
                return Err(RuntimeError::MissingDependency {
                    mod_id: id.clone(),
                    dependency: dependency.id.clone(),
                });
            };
            if !dependency.version.matches(&target.manifest.version) {
                return Err(RuntimeError::DependencyVersionMismatch {
                    mod_id: id.clone(),
                    dependency: dependency.id.clone(),
                    required: dependency.version.clone(),
                    found: target.manifest.version.clone(),
                });
            }
            *indegree.get_mut(id).expect("all ids have indegree") += 1;
            outgoing
                .entry(dependency.id.clone())
                .or_default()
                .push(id.clone());
        }
    }

    let mut ready = BTreeSet::new();
    for (id, degree) in &indegree {
        if *degree == 0 {
            let priority = by_id[id].manifest.load_priority;
            ready.insert((Reverse(priority), id.clone()));
        }
    }

    let mut order = Vec::with_capacity(packages.len());
    while let Some((Reverse(_priority), id)) = ready.pop_first() {
        order.push(id.clone());
        if let Some(dependents) = outgoing.get(&id) {
            for dependent in dependents {
                let degree = indegree.get_mut(dependent).expect("dependent id exists");
                *degree -= 1;
                if *degree == 0 {
                    let priority = by_id[dependent].manifest.load_priority;
                    ready.insert((Reverse(priority), dependent.clone()));
                }
            }
        }
    }

    if order.len() != packages.len() {
        return Err(RuntimeError::DependencyCycle);
    }

    Ok(order)
}

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct ScriptResponse {
    commands: Vec<CommandEnvelope>,
    providers: Vec<ProviderRegistration>,
    api_requests: Vec<ApiScriptRequest>,
}

#[derive(Debug, Error)]
pub enum ApiCallError {
    #[error("mod is not loaded")]
    ModNotLoaded,
    #[error("mod is disabled")]
    ModDisabled,
    #[error("API operation is unknown or unsupported by the authorization policy")]
    UnknownOperation,
    #[error("API operation identifier is invalid")]
    InvalidOperation,
    #[error("API request payload exceeds the configured byte limit")]
    PayloadTooLarge,
    #[error("API provider response exceeds the configured byte limit")]
    ResponseTooLarge,
    #[error("mod lacks capability {0:?}")]
    CapabilityDenied(Capability),
    #[error(transparent)]
    Router(#[from] ApiRouterError),
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("runtime already has loaded mods; unload them before replacing the package set")]
    AlreadyLoaded,
    #[error("duplicate mod id: {0}")]
    DuplicateMod(ModId),
    #[error("package for mod {0} disappeared during load")]
    MissingPackage(ModId),
    #[error("mod {mod_id} requires DreamMod API {required}, runtime provides {actual}")]
    ApiMismatch {
        mod_id: ModId,
        required: semver::VersionReq,
        actual: Version,
    },
    #[error("mod {mod_id} requires missing dependency {dependency}")]
    MissingDependency { mod_id: ModId, dependency: ModId },
    #[error("mod {mod_id} requires {dependency} {required}, found {found}")]
    DependencyVersionMismatch {
        mod_id: ModId,
        dependency: ModId,
        required: semver::VersionReq,
        found: Version,
    },
    #[error("mod dependency graph contains a cycle")]
    DependencyCycle,
    #[error("required capability {capability:?} was not granted to {mod_id}")]
    RequiredCapabilityDenied {
        mod_id: ModId,
        capability: Capability,
    },
    #[error("mods directory scan failed: {0}")]
    ScanDirectory(#[from] ScanDirectoryError),
    #[error("manifest error: {0}")]
    Manifest(#[from] ManifestError),
    #[error("script error: {0}")]
    Script(#[from] ScriptError),
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use serde_json::json;
    use zip::{ZipWriter, write::SimpleFileOptions};

    use crate::{
        manifest::{ModDependency, ModId, ModManifest, ModSide},
        package::DmodPackage,
        script::{ExecutionLimits, RhaiEngine},
    };

    use super::*;

    struct TestAuthorizer;

    impl CommandAuthorizer for TestAuthorizer {
        fn required_capability(&self, command: &CommandEnvelope) -> Option<Capability> {
            match (command.namespace.as_str(), command.name.as_str()) {
                ("world", "set_block") => Some(Capability::WorldWrite),
                ("world", "block_get") => Some(Capability::WorldRead),
                ("entity", "set_health") => Some(Capability::EntityWrite),
                _ => None,
            }
        }

        fn required_provider_capability(
            &self,
            provider: &ProviderRegistration,
        ) -> Option<Capability> {
            match provider.subsystem.as_str() {
                "physics" => Some(Capability::PhysicsOverride),
                "ai" => Some(Capability::AiOverride),
                "rendering" => Some(Capability::RenderOverride),
                _ => None,
            }
        }
    }

    fn package(
        id: &str,
        source: &str,
        events: &[&str],
        required: &[Capability],
        dependencies: Vec<ModDependency>,
        priority: i32,
    ) -> DmodPackage {
        let manifest = ModManifest {
            schema_version: 1,
            id: ModId::parse(id).unwrap(),
            version: Version::new(1, 0, 0),
            display_name: id.to_owned(),
            dreammod_api: semver::VersionReq::parse("^0.1.0").unwrap(),
            entry: "scripts/main.dream".to_owned(),
            side: ModSide::Both,
            load_priority: priority,
            events: events.iter().map(|event| (*event).to_owned()).collect(),
            dependencies,
            required_capabilities: required.to_vec(),
            optional_capabilities: Vec::new(),
        };
        DmodPackage {
            manifest,
            entry_source: source.to_owned(),
            files: BTreeMap::new(),
            sha256: "0".repeat(64),
        }
    }

    #[test]
    fn dispatches_commands_and_registers_subsystem_provider() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.example",
            r#"
                fn on_server_tick(event) {
                    #{
                        commands: [
                            #{ namespace: "world", name: "set_block", payload: #{ x: 4 } }
                        ],
                        providers: [
                            #{ subsystem: "physics", function: "custom_physics", priority: 20, mode: "replace" }
                        ]
                    }
                }
                fn custom_physics(input) { #{ processed: input.count } }
            "#,
            &["server_tick"],
            &[Capability::WorldWrite, Capability::PhysicsOverride],
            Vec::new(),
            0,
        );

        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        assert!(!runtime.is_enabled(&ModId::parse("dreammod.example").unwrap()));
        runtime.enable_mod(&ModId::parse("dreammod.example").unwrap());
        let event = RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({}));
        let report = runtime.dispatch_event(&event, &TestAuthorizer);

        assert_eq!(report.commands.len(), 1);
        assert_eq!(report.providers.len(), 1);
        assert_eq!(report.providers[0].registration.subsystem, "physics");

        let provider_output = runtime
            .invoke_provider(
                &ModId::parse("dreammod.example").unwrap(),
                "physics",
                &json!({ "count": 12 }),
            )
            .unwrap()
            .unwrap();
        assert_eq!(provider_output["processed"], 12);
    }

    struct EchoApiProvider {
        descriptor: crate::api_router::ApiProviderDescriptor,
    }

    impl crate::api_router::ApiProvider for EchoApiProvider {
        fn descriptor(&self) -> &crate::api_router::ApiProviderDescriptor {
            &self.descriptor
        }

        fn invoke(&mut self, request: &crate::api_router::ApiRequest) -> Result<Value, String> {
            Ok(json!({"source": self.descriptor.id, "payload": request.input}))
        }
    }

    #[test]
    fn dmod_script_can_request_api_and_receive_result_on_next_event() {
        use crate::api_router::{ApiOperation, ApiOperationSafety, ApiProviderKind, ApiProviderDescriptor};

        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_id = ModId::parse("dreammod.script_api").unwrap();
        let source = r#"
            fn on_server_tick(event) {
                if event.api_results.len() == 0 {
                    #{
                        api_requests: [
                            #{
                                request_id: "block_lookup",
                                operation: #{ domain: "world", name: "block_get" },
                                safety: "read_only",
                                input: #{ x: 7, y: 64, z: -2 }
                            }
                        ]
                    }
                } else {
                    #{
                        commands: [
                            #{
                                namespace: "world",
                                name: "set_block",
                                payload: #{ x: event.api_results[0].result.payload.x, y: 65, z: -2 }
                            }
                        ]
                    }
                }
            }
        "#;
        let mod_package = package(
            "dreammod.script_api",
            source,
            &["server_tick"],
            &[Capability::WorldRead, Capability::WorldWrite],
            Vec::new(),
            0,
        );
        runtime.load_packages(vec![mod_package], &AllowAllForTests).unwrap();
        runtime.enable_mod(&mod_id);

        let operation = ApiOperation::new("world", "block_get").unwrap();
        runtime.register_api_provider(Box::new(EchoApiProvider {
            descriptor: ApiProviderDescriptor {
                id: "horizon_world".into(),
                kind: ApiProviderKind::Horizon,
                priority: 0,
                operations: [operation].into(),
            },
        })).unwrap();

        let event = RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({ "tick": 1 }));
        let first = runtime.dispatch_event(&event, &TestAuthorizer);
        assert!(first.commands.is_empty());
        assert!(first.failures.is_empty());

        let second = runtime.dispatch_event(&event, &TestAuthorizer);
        assert!(second.failures.is_empty(), "{:?}", second.failures);
        assert_eq!(second.commands.len(), 1);
        assert_eq!(second.commands[0].command.payload["x"], 7);
    }

    #[test]
    fn runtime_api_calls_require_enabled_mod_and_granted_capability_then_route() {
        use crate::api_router::{ApiOperation, ApiOperationSafety, ApiProviderKind, ApiRequest};

        let mut runtime = DreamModRuntime::new(
            RhaiEngine::default(),
            RuntimeConfig {
                max_output_bytes: 128,
                ..RuntimeConfig::default()
            },
        );
        let mod_id = ModId::parse("dreammod.api_test").unwrap();
        let mod_package = package(
            "dreammod.api_test",
            "fn on_server_tick(event) { #{} }",
            &["server_tick"],
            &[Capability::WorldRead],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();

        let operation = ApiOperation::new("world", "block_get").unwrap();
        runtime
            .register_api_provider(Box::new(EchoApiProvider {
                descriptor: crate::api_router::ApiProviderDescriptor {
                    id: "horizon_world".into(),
                    kind: ApiProviderKind::Horizon,
                    priority: 0,
                    operations: [operation.clone()].into(),
                },
            }))
            .unwrap();

        let request = ApiRequest {
            operation,
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"x": 3, "y": 4, "z": 5}),
        };
        assert!(matches!(
            runtime.call_api(&mod_id, request.clone(), &TestAuthorizer),
            Err(ApiCallError::ModDisabled)
        ));
        runtime.enable_mod(&mod_id);
        let output = runtime
            .call_api(&mod_id, request.clone(), &TestAuthorizer)
            .unwrap();
        assert_eq!(output["source"], "horizon_world");
        assert_eq!(output["payload"]["x"], 3);
        let oversized = ApiRequest {
            operation: request.operation.clone(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"blob": "x".repeat(40_000)}),
        };
        assert!(matches!(
            runtime.call_api(&mod_id, oversized, &TestAuthorizer),
            Err(ApiCallError::PayloadTooLarge)
        ));
        let large_response = ApiRequest {
            operation: request.operation.clone(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"blob": "y".repeat(200)}),
        };
        assert!(matches!(
            runtime.call_api(&mod_id, large_response, &TestAuthorizer),
            Err(ApiCallError::ResponseTooLarge)
        ));
    }

    #[test]
    fn runtime_api_call_denies_unknown_operation_and_missing_capability() {
        use crate::api_router::{ApiOperation, ApiOperationSafety, ApiRequest};

        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_id = ModId::parse("dreammod.no_cap").unwrap();
        let mod_package = package(
            "dreammod.no_cap",
            "fn on_server_tick(event) { #{} }",
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        runtime.load_packages(vec![mod_package], &DenyAll).unwrap();
        runtime.enable_mod(&mod_id);

        let request = ApiRequest {
            operation: ApiOperation::new("world", "block_get").unwrap(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({}),
        };
        assert!(matches!(
            runtime.call_api(&mod_id, request.clone(), &TestAuthorizer),
            Err(ApiCallError::CapabilityDenied(Capability::WorldRead))
        ));
        let unknown = ApiRequest {
            operation: ApiOperation::new("world", "mystery").unwrap(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({}),
        };
        assert!(matches!(
            runtime.call_api(&mod_id, unknown, &TestAuthorizer),
            Err(ApiCallError::UnknownOperation)
        ));
    }

    #[derive(Default)]
    struct RecordingHost {
        calls: usize,
        commands: Vec<AuthorizedCommand>,
    }

    impl EngineCommandHost for RecordingHost {
        type Error = &'static str;

        fn execute_atomic_batch(
            &mut self,
            commands: &[AuthorizedCommand],
        ) -> Result<(), Self::Error> {
            self.calls += 1;
            self.commands.extend_from_slice(commands);
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockBlockWorldHost {
        blocks: BTreeMap<(i32, i32, i32), String>,
        entity_health: BTreeMap<String, f32>,
    }

    impl EngineCommandHost for MockBlockWorldHost {
        type Error = String;

        fn execute_atomic_batch(
            &mut self,
            commands: &[AuthorizedCommand],
        ) -> Result<(), Self::Error> {
            // Stage mutations from every domain; commit only after the whole batch validates.
            enum Mutation {
                Block((i32, i32, i32), String),
                EntityHealth(String, f32),
            }
            let mut staged = Vec::with_capacity(commands.len());
            for command in commands {
                let payload = &command.command.payload;
                match (
                    command.command.namespace.as_str(),
                    command.command.name.as_str(),
                ) {
                    ("world", "set_block") => {
                        let x = payload["x"].as_i64().ok_or("x must be an integer")?;
                        let y = payload["y"].as_i64().ok_or("y must be an integer")?;
                        let z = payload["z"].as_i64().ok_or("z must be an integer")?;
                        if !(-30_000_000..=30_000_000).contains(&x)
                            || !(-64..=320).contains(&y)
                            || !(-30_000_000..=30_000_000).contains(&z)
                        {
                            return Err("block position is out of bounds".to_owned());
                        }
                        let block = payload["block"].as_str().ok_or("block must be a string")?;
                        let block_name = block
                            .strip_prefix("minecraft:")
                            .ok_or("unsupported block namespace")?;
                        if block_name.is_empty()
                            || !block_name.chars().all(|ch| {
                                ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_'
                            })
                        {
                            return Err("invalid block identifier".to_owned());
                        }
                        staged.push(Mutation::Block(
                            (x as i32, y as i32, z as i32),
                            block.to_owned(),
                        ));
                    }
                    ("entity", "set_health") => {
                        let entity = payload["entity"]
                            .as_str()
                            .ok_or("entity must be a string")?;
                        if entity.is_empty()
                            || entity.len() > 64
                            || !entity
                                .chars()
                                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
                        {
                            return Err("invalid entity handle".to_owned());
                        }
                        let health =
                            payload["health"].as_f64().ok_or("health must be numeric")? as f32;
                        if !health.is_finite() || !(0.0..=1000.0).contains(&health) {
                            return Err("health is out of bounds".to_owned());
                        }
                        staged.push(Mutation::EntityHealth(entity.to_owned(), health));
                    }
                    _ => return Err("unsupported mock-engine command".to_owned()),
                }
            }

            for mutation in staged {
                match mutation {
                    Mutation::Block(pos, block) => {
                        self.blocks.insert(pos, block);
                    }
                    Mutation::EntityHealth(entity, health) => {
                        self.entity_health.insert(entity, health);
                    }
                }
            }
            Ok(())
        }
    }

    #[test]
    fn mock_world_host_applies_valid_batch_and_rejects_invalid_batch_atomically() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.mock_world",
            r#"
                fn on_server_tick(event) {
                    #{ commands: [
                        #{ namespace: "world", name: "set_block",
                           payload: #{ x: 1, y: 2, z: 3, block: "minecraft:stone" } },
                        #{ namespace: "world", name: "set_block",
                           payload: #{ x: 4, y: 5, z: 6, block: "minecraft:dirt" } },
                        #{ namespace: "entity", name: "set_health",
                           payload: #{ entity: "player_1", health: 75.0 } }
                    ] }
                }
            "#,
            &["server_tick"],
            &[Capability::WorldWrite, Capability::EntityWrite],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        runtime.enable_mod(&ModId::parse("dreammod.mock_world").unwrap());
        let mut host = MockBlockWorldHost::default();

        let valid = runtime.dispatch_event_and_execute(
            &RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({})),
            &TestAuthorizer,
            &mut host,
        );
        assert_eq!(valid.dispatch.commands.len(), 3);
        assert_eq!(valid.execution, Some(Ok(())));
        assert_eq!(
            host.blocks.get(&(1, 2, 3)).map(String::as_str),
            Some("minecraft:stone")
        );
        assert_eq!(
            host.blocks.get(&(4, 5, 6)).map(String::as_str),
            Some("minecraft:dirt")
        );
        assert_eq!(host.entity_health.get("player_1"), Some(&75.0));

        // A second batch contains one valid write followed by an invalid block ID.
        let invalid_commands = vec![
            AuthorizedCommand {
                mod_id: ModId::parse("dreammod.mock_world").unwrap(),
                command: CommandEnvelope {
                    namespace: "entity".to_owned(),
                    name: "set_health".to_owned(),
                    payload: json!({ "entity": "player_1", "health": 20.0 }),
                },
            },
            AuthorizedCommand {
                mod_id: ModId::parse("dreammod.mock_world").unwrap(),
                command: CommandEnvelope {
                    namespace: "world".to_owned(),
                    name: "set_block".to_owned(),
                    payload: json!({ "x": 7, "y": 8, "z": 9, "block": "minecraft:gold_block" }),
                },
            },
            AuthorizedCommand {
                mod_id: ModId::parse("dreammod.mock_world").unwrap(),
                command: CommandEnvelope {
                    namespace: "world".to_owned(),
                    name: "set_block".to_owned(),
                    payload: json!({ "x": 10, "y": 11, "z": 12, "block": "../unsafe" }),
                },
            },
        ];
        let before_blocks = host.blocks.clone();
        let before_entities = host.entity_health.clone();
        assert!(host.execute_atomic_batch(&invalid_commands).is_err());
        assert_eq!(
            host.blocks, before_blocks,
            "invalid batch must not partially mutate world"
        );
        assert_eq!(
            host.entity_health, before_entities,
            "invalid batch must not partially mutate entities"
        );
    }

    #[test]
    fn event_commands_reach_host_only_after_authorization() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.host_test",
            r#"
                fn on_server_tick(event) {
                    #{ commands: [
                        #{ namespace: "world", name: "set_block",
                           payload: #{ x: 4, y: 5, z: 6, block: "stone" } }
                    ] }
                }
            "#,
            &["server_tick"],
            &[Capability::WorldWrite],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        let mod_id = ModId::parse("dreammod.host_test").unwrap();
        runtime.enable_mod(&mod_id);
        let mut host = RecordingHost::default();

        let result = runtime.dispatch_event_and_execute(
            &RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({})),
            &TestAuthorizer,
            &mut host,
        );

        assert_eq!(result.dispatch.commands.len(), 1);
        assert_eq!(result.execution, Some(Ok(())));
        assert_eq!(host.calls, 1);
        assert_eq!(host.commands[0].mod_id, mod_id);
        assert_eq!(host.commands[0].command.payload["block"], "stone");
    }

    #[test]
    fn host_is_not_called_for_empty_or_denied_command_batches() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.denied_host",
            r#"
                fn on_server_tick(event) {
                    #{ commands: [
                        #{ namespace: "world", name: "set_block", payload: #{} }
                    ] }
                }
            "#,
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        runtime.enable_mod(&ModId::parse("dreammod.denied_host").unwrap());
        let mut host = RecordingHost::default();

        let result = runtime.dispatch_event_and_execute(
            &RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({})),
            &TestAuthorizer,
            &mut host,
        );

        assert!(result.dispatch.commands.is_empty());
        assert_eq!(result.dispatch.denied.len(), 1);
        assert_eq!(result.execution, None);
        assert_eq!(host.calls, 0);
    }

    #[test]
    fn packages_are_loaded_disabled_until_explicit_activation() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.inactive",
            "fn on_server_tick(event) { #{} }",
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        let id = ModId::parse("dreammod.inactive").unwrap();
        assert!(!runtime.is_enabled(&id));
        let report = runtime.dispatch_event(
            &RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({})),
            &TestAuthorizer,
        );
        assert!(report.commands.is_empty());
        assert!(report.providers.is_empty());
        assert!(runtime.enable_mod(&id));
        assert!(runtime.is_enabled(&id));
        assert!(!runtime.enable_mod(&ModId::parse("dreammod.missing").unwrap()));
    }

    #[test]
    fn required_capabilities_are_denied_by_default() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.protected",
            "fn on_server_tick(event) { #{} }",
            &["server_tick"],
            &[Capability::WorldWrite],
            Vec::new(),
            0,
        );

        assert!(matches!(
            runtime.load_packages(vec![mod_package], &DenyAll),
            Err(RuntimeError::RequiredCapabilityDenied { .. })
        ));
    }

    #[test]
    fn dependencies_load_before_dependents_and_cycles_are_rejected() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let base = package(
            "dreammod.base",
            "fn on_load(event) { #{} }",
            &[],
            &[],
            Vec::new(),
            -100,
        );
        let dependent = package(
            "dreammod.dependent",
            "fn on_load(event) { #{} }",
            &[],
            &[],
            vec![ModDependency {
                id: ModId::parse("dreammod.base").unwrap(),
                version: semver::VersionReq::parse("^1.0.0").unwrap(),
            }],
            100,
        );

        let report = runtime
            .load_packages(vec![dependent, base], &AllowAllForTests)
            .unwrap();
        assert_eq!(
            report.loaded,
            vec![
                ModId::parse("dreammod.base").unwrap(),
                ModId::parse("dreammod.dependent").unwrap()
            ]
        );

        let mut cyclic_runtime =
            DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let a = package(
            "dreammod.alpha",
            "fn on_load(event) { #{} }",
            &[],
            &[],
            vec![ModDependency {
                id: ModId::parse("dreammod.beta").unwrap(),
                version: semver::VersionReq::parse("^1.0.0").unwrap(),
            }],
            0,
        );
        let b = package(
            "dreammod.beta",
            "fn on_load(event) { #{} }",
            &[],
            &[],
            vec![ModDependency {
                id: ModId::parse("dreammod.alpha").unwrap(),
                version: semver::VersionReq::parse("^1.0.0").unwrap(),
            }],
            0,
        );

        assert!(matches!(
            cyclic_runtime.load_packages(vec![a, b], &AllowAllForTests),
            Err(RuntimeError::DependencyCycle)
        ));
    }

    #[test]
    fn repeated_script_errors_disable_only_the_failing_mod() {
        let mut runtime = DreamModRuntime::new(
            RhaiEngine::new(ExecutionLimits {
                max_operations: 64,
                ..ExecutionLimits::default()
            }),
            RuntimeConfig {
                max_failures_before_disable: 2,
                ..RuntimeConfig::default()
            },
        );
        let bad = package(
            "dreammod.bad",
            "fn on_server_tick(event) { loop { } }",
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        let good = package(
            "dreammod.good",
            "fn on_server_tick(event) { #{ commands: [] } }",
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![bad, good], &AllowAllForTests)
            .unwrap();
        runtime.enable_mod(&ModId::parse("dreammod.bad").unwrap());
        runtime.enable_mod(&ModId::parse("dreammod.good").unwrap());

        let event = RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({}));
        let first = runtime.dispatch_event(&event, &TestAuthorizer);
        let second = runtime.dispatch_event(&event, &TestAuthorizer);

        assert_eq!(first.failures.len(), 1);
        assert_eq!(second.failures.len(), 1);
        assert!(!runtime.is_enabled(&ModId::parse("dreammod.bad").unwrap()));
        assert!(runtime.is_enabled(&ModId::parse("dreammod.good").unwrap()));
    }

    #[test]
    fn fault_disabling_mod_removes_provider_registrations() {
        let mut runtime = DreamModRuntime::new(
            RhaiEngine::new(ExecutionLimits {
                max_operations: 32,
                ..ExecutionLimits::default()
            }),
            RuntimeConfig {
                max_failures_before_disable: 1,
                ..RuntimeConfig::default()
            },
        );
        let mod_package = package(
            "dreammod.faulty_provider",
            r#"
                fn on_server_tick(event) {
                    #{ providers: [
                        #{ subsystem: "physics", function: "broken_physics", priority: 1, mode: "replace" }
                    ] }
                }
                fn broken_physics(input) { loop { } }
            "#,
            &["server_tick"],
            &[Capability::PhysicsOverride],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        let id = ModId::parse("dreammod.faulty_provider").unwrap();
        runtime.enable_mod(&id);
        let report = runtime.dispatch_event(
            &RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({})),
            &TestAuthorizer,
        );
        assert_eq!(report.providers.len(), 1);
        assert!(runtime.invoke_provider(&id, "physics", &json!({})).is_err());
        assert!(!runtime.is_enabled(&id));
        assert!(runtime.enable_mod(&id));
        assert_eq!(
            runtime.invoke_provider(&id, "physics", &json!({})),
            Ok(None)
        );
    }

    #[test]
    fn commands_without_granted_capabilities_are_filtered() {
        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let mod_package = package(
            "dreammod.limited",
            r#"
                fn on_server_tick(event) {
                    #{ commands: [
                        #{ namespace: "world", name: "set_block", payload: #{} }
                    ] }
                }
            "#,
            &["server_tick"],
            &[],
            Vec::new(),
            0,
        );
        runtime
            .load_packages(vec![mod_package], &AllowAllForTests)
            .unwrap();
        runtime.enable_mod(&ModId::parse("dreammod.limited").unwrap());

        let event = RuntimeEvent::new("server_tick", RuntimeSide::Server, json!({}));
        let report = runtime.dispatch_event(&event, &TestAuthorizer);
        assert!(report.commands.is_empty());
        assert_eq!(report.denied.len(), 1);
    }
    #[test]
    fn directory_loading_keeps_horizon_mods_separate_and_reports_bad_packages() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("Horizon.hmod"), b"horizon").unwrap();
        std::fs::write(directory.path().join("broken.dmod"), b"not a zip").unwrap();

        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let report = runtime
            .load_from_horizon_mods_dir(directory.path(), &DenyAll)
            .unwrap();

        assert!(report.load.loaded.is_empty());
        assert_eq!(report.ignored_hmod_count, 1);
        assert_eq!(report.issues.len(), 1);
        assert!(report.issues[0].reason.contains("zip"));
    }
    #[test]
    fn directory_loading_activates_valid_dmod_and_keeps_horizon_files_separate() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("Horizon.hmod"), b"horizon").unwrap();
        std::fs::write(directory.path().join("bad.dmod"), b"not a zip").unwrap();

        let valid_path = directory.path().join("valid.dmod");
        let output = std::fs::File::create(&valid_path).unwrap();
        let mut writer = ZipWriter::new(output);
        let options = SimpleFileOptions::default();
        let manifest = r#"{
            "schema_version":1,
            "id":"dreammod.valid",
            "version":"1.0.0",
            "display_name":"Valid",
            "dreammod_api":"^0.1.0",
            "entry":"scripts/main.dream",
            "side":"both",
            "events":["server_tick"]
        }"#;
        writer.start_file("manifest.json", options).unwrap();
        writer.write_all(manifest.as_bytes()).unwrap();
        writer.start_file("scripts/main.dream", options).unwrap();
        writer
            .write_all(b"fn on_server_tick(event) { #{ commands: [] } }")
            .unwrap();
        writer.finish().unwrap();

        let mut runtime = DreamModRuntime::new(RhaiEngine::default(), RuntimeConfig::default());
        let report = runtime
            .load_from_horizon_mods_dir(directory.path(), &DenyAll)
            .unwrap();

        assert_eq!(report.load.loaded.len(), 1);
        assert_eq!(report.load.loaded[0].as_str(), "dreammod.valid");
        assert_eq!(report.ignored_hmod_count, 1);
        assert_eq!(report.issues.len(), 1);
        assert!(report.issues[0].path.ends_with("bad.dmod"));
    }
}
