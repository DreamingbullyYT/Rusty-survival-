use crate::manifest::ModuleDescriptor;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ModuleRegistry {
    modules: BTreeMap<String, ModuleDescriptor>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RegistryError {
    #[error("module '{0}' is already registered")]
    Duplicate(String),
    #[error("module '{module}' requires missing module '{dependency}'")]
    MissingDependency { module: String, dependency: String },
    #[error("dependency cycle detected involving '{0}'")]
    DependencyCycle(String),
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, module: ModuleDescriptor) -> Result<(), RegistryError> {
        if self.modules.contains_key(&module.id) {
            return Err(RegistryError::Duplicate(module.id));
        }
        self.modules.insert(module.id.clone(), module);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ModuleDescriptor> {
        self.modules.get(id)
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    /// Returns a deterministic dependency-first load order.
    pub fn resolve_load_order(&self) -> Result<Vec<String>, RegistryError> {
        for module in self.modules.values() {
            for dependency in &module.requires {
                if !self.modules.contains_key(dependency) {
                    return Err(RegistryError::MissingDependency {
                        module: module.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }

        let mut temporary = BTreeSet::new();
        let mut permanent = BTreeSet::new();
        let mut ordered = Vec::with_capacity(self.modules.len());

        for id in self.modules.keys() {
            self.visit(id, &mut temporary, &mut permanent, &mut ordered)?;
        }
        Ok(ordered)
    }

    fn visit(
        &self,
        id: &str,
        temporary: &mut BTreeSet<String>,
        permanent: &mut BTreeSet<String>,
        ordered: &mut Vec<String>,
    ) -> Result<(), RegistryError> {
        if permanent.contains(id) {
            return Ok(());
        }
        if !temporary.insert(id.to_owned()) {
            return Err(RegistryError::DependencyCycle(id.to_owned()));
        }

        let module = &self.modules[id];
        let mut dependencies = module.requires.clone();
        dependencies.sort();
        for dependency in dependencies {
            self.visit(&dependency, temporary, permanent, ordered)?;
        }

        temporary.remove(id);
        permanent.insert(id.to_owned());
        ordered.push(id.to_owned());
        Ok(())
    }
}
