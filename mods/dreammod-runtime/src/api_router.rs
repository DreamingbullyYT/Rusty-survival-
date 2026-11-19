use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
}
