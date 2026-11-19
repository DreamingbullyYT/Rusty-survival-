use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Stable operation name shared by mods and engine adapters (e.g. world.block.get).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ApiOperation {
    pub domain: String,
    pub name: String,
}

impl ApiOperation {
    pub fn new(domain: impl Into<String>, name: impl Into<String>) -> Result<Self, ApiRouterError> {
        let domain = domain.into();
        let name = name.into();
        if !valid_api_name(&domain) || !valid_api_name(&name) {
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
    /// Higher priority wins within a provider preference.
    pub priority: i32,
    pub operations: BTreeSet<ApiOperation>,
}

/// Provider boundary: implementations adapt to Horizon's API or DreamMod's engine API.
pub trait ApiProvider: Send {
    fn descriptor(&self) -> &ApiProviderDescriptor;
    fn invoke(&mut self, request: &ApiRequest) -> Result<Value, String>;
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
    #[error("invalid provider id '{0}'")]
    InvalidProviderId(String),
    #[error("provider '{0}' is not registered")]
    ProviderNotRegistered(String),
    #[error("no provider supports operation '{0}'")]
    UnsupportedOperation(String),
    #[error("provider '{provider}' failed operation '{operation}': {message}")]
    ProviderFailed { provider: String, operation: String, message: String },
}

/// Routes API operations to compatible providers. The active preference can be
/// changed at runtime and providers can be replaced/unregistered without changing
/// mod scripts. Provider errors are not retried: mutation fallback could duplicate
/// a partially committed operation.
pub struct ApiRouter {
    providers: BTreeMap<String, Box<dyn ApiProvider>>,
    preference: BTreeMap<String, ApiProviderPreference>,
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
            preference: BTreeMap::new(),
            default_preference,
        }
    }

    /// Registers or hot-replaces a provider by ID. The replacement becomes visible
    /// immediately to subsequent calls; existing calls are never interrupted.
    pub fn register_provider(
        &mut self,
        provider: Box<dyn ApiProvider>,
    ) -> Result<(), ApiRouterError> {
        let descriptor = provider.descriptor();
        if !valid_api_name(&descriptor.id) {
            return Err(ApiRouterError::InvalidProviderId(descriptor.id.clone()));
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
        if !valid_api_name(&domain) {
            return Err(ApiRouterError::InvalidOperation {
                domain,
                name: "*".to_owned(),
            });
        }
        self.preference.insert(domain, preference);
        Ok(())
    }

    pub fn preference(&self, domain: &str) -> ApiProviderPreference {
        self.preference
            .get(domain)
            .copied()
            .unwrap_or(self.default_preference)
    }

    pub fn call(&mut self, request: ApiRequest) -> Result<Value, ApiRouterError> {
        let key = request.operation.key();
        let preference = self.preference(&request.operation.domain);
        let mut candidates = self.providers
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
        candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0))));

        let Some((provider_id, _, _)) = candidates.first() else {
            return Err(ApiRouterError::UnsupportedOperation(key));
        };
        let provider_id = provider_id.clone();
        let provider = self.providers.get_mut(&provider_id).expect("candidate provider exists");
        provider.invoke(&request).map_err(|message| ApiRouterError::ProviderFailed {
            provider: provider_id,
            operation: key,
            message,
        })
    }

    pub fn providers(&self) -> Vec<ApiProviderDescriptor> {
        self.providers.values().map(|p| p.descriptor().clone()).collect()
    }
}

fn valid_api_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct FakeProvider {
        descriptor: ApiProviderDescriptor,
        answer: Value,
        fail: bool,
    }

    impl ApiProvider for FakeProvider {
        fn descriptor(&self) -> &ApiProviderDescriptor {
            &self.descriptor
        }

        fn invoke(&mut self, request: &ApiRequest) -> Result<Value, String> {
            if self.fail {
                return Err("simulated provider failure".into());
            }
            Ok(json!({"provider": self.descriptor.id, "input": request.input}))
        }
    }

    fn operation() -> ApiOperation {
        ApiOperation::new("world", "block_get").unwrap()
    }

    fn fake(id: &str, kind: ApiProviderKind, priority: i32, fail: bool) -> FakeProvider {
        FakeProvider {
            descriptor: ApiProviderDescriptor {
                id: id.into(),
                kind,
                priority,
                operations: [operation()].into(),
            },
            answer: json!({}),
            fail,
        }
    }

    fn request() -> ApiRequest {
        ApiRequest {
            operation: operation(),
            safety: ApiOperationSafety::ReadOnly,
            input: json!({"pos":[1,2,3]}),
        }
    }

    #[test]
    fn horizon_is_preferred_and_dreammod_is_fallback_when_missing() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(fake("dreammod", ApiProviderKind::DreamMod, 0, false))).unwrap();
        let first = router.call(request()).unwrap();
        assert_eq!(first["provider"], "dreammod");

        router.register_provider(Box::new(fake("horizon", ApiProviderKind::Horizon, 0, false))).unwrap();
        let second = router.call(request()).unwrap();
        assert_eq!(second["provider"], "horizon");
    }

    #[test]
    fn preference_switches_provider_without_reloading_mods() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(fake("horizon", ApiProviderKind::Horizon, 0, false))).unwrap();
        router.register_provider(Box::new(fake("dreammod", ApiProviderKind::DreamMod, 0, false))).unwrap();

        assert_eq!(router.call(request()).unwrap()["provider"], "horizon");
        router.set_preference("world", ApiProviderPreference::DreamModFirst).unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "dreammod");
    }

    #[test]
    fn provider_failure_does_not_retry_mutations_or_reads_implicitly() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(fake("horizon", ApiProviderKind::Horizon, 5, true))).unwrap();
        router.register_provider(Box::new(fake("dreammod", ApiProviderKind::DreamMod, 0, false))).unwrap();

        let error = router.call(request()).unwrap_err();
        assert!(matches!(error, ApiRouterError::ProviderFailed { provider, .. } if provider == "horizon"));
    }

    #[test]
    fn hot_replacement_and_removal_are_visible_to_next_call() {
        let mut router = ApiRouter::default();
        router.register_provider(Box::new(fake("horizon", ApiProviderKind::Horizon, 0, false))).unwrap();
        assert_eq!(router.call(request()).unwrap()["provider"], "horizon");
        router.register_provider(Box::new(fake("horizon", ApiProviderKind::Horizon, 0, true))).unwrap();
        assert!(router.call(request()).is_err());
        router.unregister_provider("horizon").unwrap();
        assert!(matches!(router.call(request()), Err(ApiRouterError::UnsupportedOperation(_))));
    }

    #[test]
    fn rejects_invalid_operation_and_provider_ids() {
        assert!(ApiOperation::new("../world", "block_get").is_err());
        let mut router = ApiRouter::default();
        assert!(router.register_provider(Box::new(fake("../horizon", ApiProviderKind::Horizon, 0, false))).is_err());
    }
}
