use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiOperation {
    pub domain: String,
    pub name: String,
}

impl ApiOperation {
    pub fn new(domain: impl Into<String>, name: impl Into<String>) -> Result<Self, ApiRouterError> {
        let domain = domain.into();
        let name = name.into();
        if !valid_name(&domain) || !valid_name(&name) {
            return Err(ApiRouterError::InvalidOperation { domain, name });
        }
        Ok(Self { domain, name })
    }

    pub fn key(&self) -> String {
        format!("{}.{}", self.domain, self.name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiProviderKind {
    Horizon,
    DreamMod,
    Custom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiOperationSafety {
    ReadOnly,
    Mutating,
}

#[derive(Clone, Debug)]
pub struct ApiRequest {
    pub operation: ApiOperation,
    pub safety: ApiOperationSafety,
    pub input: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiProviderDescriptor {
    pub id: String,
    pub kind: ApiProviderKind,
    pub priority: i32,
    pub operations: BTreeSet<ApiOperation>,
}

pub trait ApiProvider: Send {
    fn descriptor(&self) -> &ApiProviderDescriptor;
    fn invoke(&mut self, request: &ApiRequest) -> Result<Value, String>;

    /// Context-aware path. Existing self-contained providers remain compatible.
    fn invoke_with_context(
        &mut self,
        request: &ApiRequest,
        _context: &mut dyn ApiOperationContext,
    ) -> Result<Value, String> {
        self.invoke(request)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApiProviderPreference {
    HorizonFirst,
    DreamModFirst,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApiRouterError {
    #[error("invalid API operation '{domain}.{name}'")]
    InvalidOperation { domain: String, name: String },
    #[error("invalid provider ID '{0}'")]
    InvalidProviderId(String),
    #[error("provider '{0}' declares invalid API operations")]
    InvalidProviderDescriptor(String),
    #[error("provider '{0}' is not registered")]
    ProviderNotRegistered(String),
    #[error("no provider supports operation '{0}'")]
    UnsupportedOperation(String),
    #[error("provider '{provider}' failed operation '{operation}': {message}")]
    ProviderFailed {
        provider: String,
        operation: String,
        message: String,
    },
    #[error("execution context does not match operation '{operation}'")]
    ContextMismatch { operation: String },
}

pub struct ApiRouter {
    providers: BTreeMap<String, Box<dyn ApiProvider>>,
    preferences: BTreeMap<String, ApiProviderPreference>,
    default_preference: ApiProviderPreference,
}

impl Default for ApiRouter {
    fn default() -> Self {
        Self::new(ApiProviderPreference::HorizonFirst)
    }
}

impl ApiRouter {
    pub fn new(default_preference: ApiProviderPreference) -> Self {
        Self {
            providers: BTreeMap::new(),
            preferences: BTreeMap::new(),
            default_preference,
        }
    }

    pub fn register_provider(
        &mut self,
        provider: Box<dyn ApiProvider>,
    ) -> Result<(), ApiRouterError> {
        let descriptor = provider.descriptor();
        if !valid_name(&descriptor.id) {
            return Err(ApiRouterError::InvalidProviderId(descriptor.id.clone()));
        }
        if descriptor.operations.iter().any(|operation| {
            !valid_name(&operation.domain) || !valid_name(&operation.name)
        }) {
            return Err(ApiRouterError::InvalidProviderDescriptor(descriptor.id.clone()));
        }
        self.providers.insert(descriptor.id.clone(), provider);
        Ok(())
    }

    pub fn unregister_provider(&mut self, provider_id: &str) -> Result<(), ApiRouterError> {
        self.providers
            .remove(provider_id)
            .map(|_| ())
            .ok_or_else(|| ApiRouterError::ProviderNotRegistered(provider_id.to_owned()))
    }

    pub fn set_preference(
        &mut self,
        domain: impl Into<String>,
        preference: ApiProviderPreference,
    ) -> Result<(), ApiRouterError> {
        let domain = domain.into();
        if !valid_name(&domain) {
            return Err(ApiRouterError::InvalidOperation {
                domain,
                name: "preference".to_owned(),
            });
        }
        self.preferences.insert(domain, preference);
        Ok(())
    }

    pub fn preference(&self, domain: &str) -> ApiProviderPreference {
        self.preferences
            .get(domain)
            .copied()
            .unwrap_or(self.default_preference)
    }

    pub fn call(&mut self, request: ApiRequest) -> Result<Value, ApiRouterError> {
        if !valid_name(&request.operation.domain) || !valid_name(&request.operation.name) {
            return Err(ApiRouterError::InvalidOperation {
                domain: request.operation.domain,
                name: request.operation.name,
            });
        }
        let operation_key = request.operation.key();
        let preference = self.preference(&request.operation.domain);
        let mut candidates = self
            .providers
            .iter()
            .filter(|(_, provider)| provider.descriptor().operations.contains(&request.operation))
            .map(|(id, provider)| {
                let descriptor = provider.descriptor();
                let preferred = matches!(
                    (preference, descriptor.kind),
                    (ApiProviderPreference::HorizonFirst, ApiProviderKind::Horizon)
                        | (ApiProviderPreference::DreamModFirst, ApiProviderKind::DreamMod)
                );
                (id.clone(), preferred, descriptor.priority)
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)))
        });

        let Some((provider_id, _, _)) = candidates.first() else {
            return Err(ApiRouterError::UnsupportedOperation(operation_key));
        };
        let provider_id = provider_id.clone();
        let provider = self.providers.get_mut(&provider_id).expect("candidate exists");
        provider.invoke(&request).map_err(|message| ApiRouterError::ProviderFailed {
            provider: provider_id,
            operation: operation_key,
            message,
        })
    }

    /// Calls a provider with a borrowed, operation-scoped host context.
    /// Context mismatch is rejected before provider code is invoked.
    pub fn call_with_context(
        &mut self,
        request: ApiRequest,
        context: &mut dyn ApiOperationContext,
    ) -> Result<Value, ApiRouterError> {
        if !valid_name(&request.operation.domain) || !valid_name(&request.operation.name) {
            return Err(ApiRouterError::InvalidOperation {
                domain: request.operation.domain,
                name: request.operation.name,
            });
        }
        let operation_key = request.operation.key();
        if context.operation() != &request.operation || context.safety() != request.safety {
            return Err(ApiRouterError::ContextMismatch {
                operation: operation_key,
            });
        }
        let preference = self.preference(&request.operation.domain);
        let mut candidates = self
            .providers
            .iter()
            .filter(|(_, provider)| provider.descriptor().operations.contains(&request.operation))
            .map(|(id, provider)| {
                let descriptor = provider.descriptor();
                let preferred = matches!(
                    (preference, descriptor.kind),
                    (ApiProviderPreference::HorizonFirst, ApiProviderKind::Horizon)
                        | (ApiProviderPreference::DreamModFirst, ApiProviderKind::DreamMod)
                );
                (id.clone(), preferred, descriptor.priority)
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|a, b| {
            b.1.cmp(&a.1)
                .then_with(|| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)))
        });

        let Some((provider_id, _, _)) = candidates.first() else {
            return Err(ApiRouterError::UnsupportedOperation(operation_key));
        };
        let provider_id = provider_id.clone();
        let provider = self.providers.get_mut(&provider_id).expect("candidate exists");
        provider
            .invoke_with_context(&request, context)
            .map_err(|message| ApiRouterError::ProviderFailed {
                provider: provider_id,
                operation: operation_key,
                message,
            })
    }

    pub fn providers(&self) -> Vec<ApiProviderDescriptor> {
        self.providers
            .values()
            .map(|provider| provider.descriptor().clone())
            .collect()
    }
}

fn valid_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-'
        })
}

/// Request emitted by a script for the runtime's event-scoped API bridge.
/// The first bridge version is deferred: the result is delivered in a later event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiScriptRequest {
    pub request_id: String,
    pub operation: ApiOperation,
    pub safety: ApiOperationSafety,
    #[serde(default)]
    pub input: Value,
}

impl ApiScriptRequest {
    pub fn validate(&self) -> Result<(), ApiScriptRequestError> {
        if !valid_request_id(&self.request_id) {
            return Err(ApiScriptRequestError::InvalidRequestId);
        }
        if !valid_name(&self.operation.domain) || !valid_name(&self.operation.name) {
            return Err(ApiScriptRequestError::InvalidOperation);
        }
        Ok(())
    }
}

/// Result envelope delivered to the requesting mod only.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiScriptResult {
    pub request_id: String,
    pub operation: ApiOperation,
    #[serde(default)]
    pub result: Option<Value>,
    #[serde(default)]
    pub error: Option<ApiScriptResultError>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiScriptResultError {
    InvalidRequest,
    UnknownOperation,
    CapabilityDenied,
    ProviderUnavailable,
    ProviderFailed,
    PayloadTooLarge,
    ResponseTooLarge,
    ModDisabled,
}

#[derive(Debug, Error, Clone, Copy, Eq, PartialEq)]
pub enum ApiScriptRequestError {
    #[error("invalid request ID")]
    InvalidRequestId,
    #[error("invalid API operation identifier")]
    InvalidOperation,
}

fn valid_request_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct FakeProvider {
        descriptor: ApiProviderDescriptor,
        fail: bool,
    }

    impl ApiProvider for FakeProvider {
        fn descriptor(&self) -> &ApiProviderDescriptor {
            &self.descriptor
        }

        fn invoke(&mut self, request: &ApiRequest) -> Result<Value, String> {
            if self.fail {
                return Err("simulated provider failure".to_owned());
            }
            Ok(json!({"provider": self.descriptor.id, "input": request.input}))
        }
    }

    fn op() -> ApiOperation {
        ApiOperation::new("world", "block_get").unwrap()
    }

    fn provider(id: &str, kind: ApiProviderKind, priority: i32, fail: bool) -> FakeProvider {
        FakeProvider {
            descriptor: ApiProviderDescriptor {
                id: id.to_owned(),
                kind,
                priority,
                operations: [op()].into(),
            },
            fail,
        }
    }

    fn request() -> ApiRequest {
        ApiRequest {
            operation: op(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"position":[1,2,3]}),
        }
    }

    struct FakeOperationContext {
        operation: ApiOperation,
        safety: ApiOperationSafety,
        result: Value,
        calls: usize,
    }

    impl ApiOperationContext for FakeOperationContext {
        fn operation(&self) -> &ApiOperation {
            &self.operation
        }

        fn safety(&self) -> ApiOperationSafety {
            self.safety
        }

        fn execute(&mut self, _input: &Value) -> Result<Value, String> {
            self.calls += 1;
            Ok(self.result.clone())
        }
    }

    struct ContextProvider {
        descriptor: ApiProviderDescriptor,
    }

    impl ApiProvider for ContextProvider {
        fn descriptor(&self) -> &ApiProviderDescriptor {
            &self.descriptor
        }

        fn invoke(&mut self, _request: &ApiRequest) -> Result<Value, String> {
            Err("context required".to_owned())
        }

        fn invoke_with_context(
            &mut self,
            request: &ApiRequest,
            context: &mut dyn ApiOperationContext,
        ) -> Result<Value, String> {
            context.execute(&request.input)
        }
    }

    #[test]
    fn scoped_context_is_forwarded_to_context_aware_provider() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(ContextProvider {
            descriptor: ApiProviderDescriptor {
                id: "ctx_provider".to_owned(),
                kind: ApiProviderKind::DreamMod,
                priority: 0,
                operations: [op()].into(),
            },
        })).unwrap();

        let expected = json!({"block":"minecraft:stone"});
        let mut context = FakeOperationContext {
            operation: op(),
            safety: ApiOperationSafety::ReadOnly,
            result: expected.clone(),
            calls: 0,
        };

        assert_eq!(router.call_with_context(request(), &mut context).unwrap(), expected);
        assert_eq!(context.calls, 1);
    }

    #[test]
    fn mismatched_context_is_rejected_before_provider_invocation() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(ContextProvider {
            descriptor: ApiProviderDescriptor {
                id: "ctx_provider".to_owned(),
                kind: ApiProviderKind::DreamMod,
                priority: 0,
                operations: [op()].into(),
            },
        })).unwrap();

        let other_operation = ApiOperation::new("entity", "get").unwrap();
        let mut context = FakeOperationContext {
            operation: other_operation,
            safety: ApiOperationSafety::ReadOnly,
            result: Value::Null,
            calls: 0,
        };

        assert!(matches!(
            router.call_with_context(request(), &mut context),
            Err(ApiRouterError::ContextMismatch { .. })
        ));
        assert_eq!(context.calls, 0);
    }

    #[test]
    fn horizon_first_and_native_fallback_by_supported_operation() {
        let mut router = ApiRouter::default();
        router
            .register_provider(Box::new(provider("dreammod", ApiProviderKind::DreamMod, 0, false)))
            .unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "dreammod");
        router
            .register_provider(Box::new(provider("horizon", ApiProviderKind::Horizon, 0, false)))
            .unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "horizon");
    }

    #[test]
    fn preference_switches_provider_without_mod_reload() {
        let mut router = ApiRouter::default();
        router
            .register_provider(Box::new(provider("horizon", ApiProviderKind::Horizon, 0, false)))
            .unwrap();
        router
            .register_provider(Box::new(provider("dreammod", ApiProviderKind::DreamMod, 0, false)))
            .unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "horizon");
        router.set_preference("world", ApiProviderPreference::DreamModFirst).unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "dreammod");
    }

    #[test]
    fn provider_failure_is_not_automatically_retried() {
        let mut router = ApiRouter::default();
        router
            .register_provider(Box::new(provider("horizon", ApiProviderKind::Horizon, 5, true)))
            .unwrap();
        router
            .register_provider(Box::new(provider("dreammod", ApiProviderKind::DreamMod, 0, false)))
            .unwrap();
        assert!(matches!(
            router.call(request()),
            Err(ApiRouterError::ProviderFailed { provider, .. }) if provider == "horizon"
        ));
    }

    #[test]
    fn replacement_and_removal_apply_to_subsequent_calls() {
        let mut router = ApiRouter::default();
        router
            .register_provider(Box::new(provider("horizon", ApiProviderKind::Horizon, 0, false)))
            .unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "horizon");
        router
            .register_provider(Box::new(provider("horizon", ApiProviderKind::Horizon, 0, true)))
            .unwrap();
        assert!(router.call(request()).is_err());
        router.unregister_provider("horizon").unwrap();
        assert!(matches!(router.call(request()), Err(ApiRouterError::UnsupportedOperation(_))));
    }

    #[test]
    fn invalid_operation_and_provider_ids_are_rejected() {
        assert!(ApiOperation::new("../world", "block_get").is_err());
        let mut router = ApiRouter::default();
        assert!(router
            .register_provider(Box::new(provider("../horizon", ApiProviderKind::Horizon, 0, false)))
            .is_err());
    }
    #[test]
    fn script_request_envelopes_validate_and_round_trip() {
        let request = ApiScriptRequest {
            request_id: "lookup_1".to_owned(),
            operation: op(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"position": [1, 2, 3]}),
        };
        request.validate().unwrap();
        let encoded = serde_json::to_string(&request).unwrap();
        let decoded: ApiScriptRequest = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, request);

        let invalid_id = ApiScriptRequest {
            request_id: "../escape".to_owned(),
            ..request.clone()
        };
        assert_eq!(
            invalid_id.validate(),
            Err(ApiScriptRequestError::InvalidRequestId)
        );

        let invalid_operation = ApiScriptRequest {
            operation: ApiOperation {
                domain: "../world".to_owned(),
                name: "block_get".to_owned(),
            },
            ..request
        };
        assert_eq!(
            invalid_operation.validate(),
            Err(ApiScriptRequestError::InvalidOperation)
        );
    }

    #[test]
    fn script_result_envelope_serializes_error_without_data() {
        let result = ApiScriptResult {
            request_id: "req-2".to_owned(),
            operation: op(),
            result: None,
            error: Some(ApiScriptResultError::CapabilityDenied),
        };
        let decoded: ApiScriptResult =
            serde_json::from_str(&serde_json::to_string(&result).unwrap()).unwrap();
        assert_eq!(decoded, result);
        assert!(decoded.result.is_none());
        assert_eq!(decoded.error, Some(ApiScriptResultError::CapabilityDenied));
    }
}
