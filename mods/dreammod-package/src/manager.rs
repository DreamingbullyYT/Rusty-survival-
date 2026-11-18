//! Deterministic owner for validated packages and their per-mod lifecycles.
//! This layer does not execute scripts or touch Horizon handles.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    lifecycle::{LifecycleError, LifecycleState, ModLifecycle},
    resolve_load_order, DependencyError, Manifest, ValidatedPackage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModManagerError {
    Dependencies(DependencyError),
    UnknownMod(String),
    DependencyNotEnabled { mod_id: String, dependency: String },
    ActiveDependent { dependency: String, dependent: String },
    Lifecycle(LifecycleError),
}

struct ManagedMod {
    package: ValidatedPackage,
    lifecycle: ModLifecycle,
}

/// Owns validated package bytes and one lifecycle per mod ID.
/// Construction validates the complete dependency graph before exposing any mod.
pub struct ModManager {
    mods: BTreeMap<String, ManagedMod>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    load_order: Vec<String>,
}

impl ModManager {
    pub fn new(packages: Vec<ValidatedPackage>) -> Result<Self, ModManagerError> {
        let manifests: Vec<Manifest> = packages
            .iter()
            .map(|package| package.manifest.clone())
            .collect();
        let load_order = resolve_load_order(&manifests)
            .map_err(ModManagerError::Dependencies)?;

        let mut mods = BTreeMap::new();
        let mut dependencies = BTreeMap::new();
        for package in packages {
            let id = package.manifest.id.clone();
            let required = package.manifest.dependencies.keys().cloned().collect();
            let lifecycle = ModLifecycle::from_validated_package(&package);
            dependencies.insert(id.clone(), required);
            mods.insert(id, ManagedMod { package, lifecycle });
        }

        Ok(Self {
            mods,
            dependencies,
            load_order,
        })
    }

    /// Dependencies precede dependents; ties follow lexicographic mod ID order.
    pub fn load_order(&self) -> &[String] {
        &self.load_order
    }

    pub fn package(&self, id: &str) -> Option<&ValidatedPackage> {
        self.mods.get(id).map(|managed| &managed.package)
    }

    pub fn lifecycle(&self, id: &str) -> Option<&ModLifecycle> {
        self.mods.get(id).map(|managed| &managed.lifecycle)
    }

    pub fn state(&self, id: &str) -> Result<LifecycleState, ModManagerError> {
        self.lifecycle(id)
            .map(ModLifecycle::state)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))
    }

    /// Approvals are per-mod and never implicitly activate the package.
    pub fn approve(
        &mut self,
        id: &str,
        approved: &[String],
    ) -> Result<(), ModManagerError> {
        let managed = self
            .mods
            .get_mut(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        managed
            .lifecycle
            .approve(approved)
            .map_err(ModManagerError::Lifecycle)
    }

    /// A dependent can only start after every declared dependency is enabled.
    pub fn enable(&mut self, id: &str) -> Result<(), ModManagerError> {
        let required = self
            .dependencies
            .get(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        for dependency in required {
            if self.state(dependency)? != LifecycleState::Enabled {
                return Err(ModManagerError::DependencyNotEnabled {
                    mod_id: id.to_owned(),
                    dependency: dependency.clone(),
                });
            }
        }
        self.mods
            .get_mut(id)
            .expect("known mod was checked above")
            .lifecycle
            .enable()
            .map_err(ModManagerError::Lifecycle)
    }

    /// Prevents disabling a dependency while a direct dependent remains enabled.
    pub fn disable(&mut self, id: &str) -> Result<(), ModManagerError> {
        if !self.mods.contains_key(id) {
            return Err(ModManagerError::UnknownMod(id.to_owned()));
        }
        for (dependent, required) in &self.dependencies {
            if required.contains(id) && self.state(dependent)? == LifecycleState::Enabled {
                return Err(ModManagerError::ActiveDependent {
                    dependency: id.to_owned(),
                    dependent: dependent.clone(),
                });
            }
        }
        self.mods
            .get_mut(id)
            .expect("known mod was checked above")
            .lifecycle
            .disable()
            .map_err(ModManagerError::Lifecycle)
    }

    /// Fails the selected mod and all transitive dependents, in dependency-first order.
    /// This is fail-closed: dependents cannot continue after a dependency faults.
    pub fn fail(&mut self, id: &str, reason: &str) -> Result<(), ModManagerError> {
        if !self.mods.contains_key(id) {
            return Err(ModManagerError::UnknownMod(id.to_owned()));
        }
        self.mods
            .get_mut(id)
            .expect("known mod was checked above")
            .lifecycle
            .fail(reason.to_owned())
            .map_err(ModManagerError::Lifecycle)?;

        let mut failed = BTreeSet::from([id.to_owned()]);
        for dependent in &self.load_order {
            if failed.contains(dependent) {
                continue;
            }
            let depends_on_failed = self.dependencies[dependent]
                .iter()
                .any(|dependency| failed.contains(dependency));
            if depends_on_failed {
                let dependency = self.dependencies[dependent]
                    .iter()
                    .find(|dependency| failed.contains(*dependency))
                    .expect("failed dependency was found");
                let failure = format!("dependency '{dependency}' failed");
                self.mods
                    .get_mut(dependent)
                    .expect("load order only contains managed mods")
                    .lifecycle
                    .fail(failure)
                    .map_err(ModManagerError::Lifecycle)?;
                failed.insert(dependent.clone());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ApiCompatibility, PackageFile};

    fn package(id: &str, dependencies: &[(&str, &str)], capabilities: &[&str]) -> ValidatedPackage {
        ValidatedPackage {
            manifest: Manifest {
                schema_version: 1,
                id: id.into(),
                name: id.into(),
                version: "1.0.0".into(),
                api: ApiCompatibility {
                    min: "0.2.0".into(),
                    max: "0.3.0".into(),
                },
                entrypoint: "scripts/main.dream".into(),
                capabilities: capabilities.iter().map(|value| (*value).into()).collect(),
                dependencies: dependencies
                    .iter()
                    .map(|(id, version)| ((*id).into(), (*version).into()))
                    .collect(),
            },
            entries: Vec::new(),
            files: vec![PackageFile {
                path: "scripts/main.dream".into(),
                contents: b"".to_vec(),
            }],
        }
    }

    #[test]
    fn constructs_dependency_first_manager_and_retains_validated_packages() {
        let manager = ModManager::new(vec![
            package("example.addon", &[("example.base", "^1.0")], &[]),
            package("example.base", &[], &[]),
        ])
        .unwrap();

        assert_eq!(manager.load_order(), &["example.base", "example.addon"]);
        assert!(manager.package("example.addon").is_some());
        assert_eq!(manager.state("example.addon"), Ok(LifecycleState::AwaitingApproval));
    }

    #[test]
    fn dependency_must_be_enabled_before_dependent() {
        let mut manager = ModManager::new(vec![
            package("example.base", &[], &["world.read"]),
            package("example.addon", &[("example.base", "^1.0")], &[]),
        ])
        .unwrap();
        manager.approve("example.addon", &[]).unwrap();
        assert_eq!(
            manager.enable("example.addon"),
            Err(ModManagerError::DependencyNotEnabled {
                mod_id: "example.addon".into(),
                dependency: "example.base".into(),
            })
        );

        manager.approve("example.base", &["world.read".into()]).unwrap();
        manager.enable("example.base").unwrap();
        manager.enable("example.addon").unwrap();
        assert_eq!(manager.state("example.addon"), Ok(LifecycleState::Enabled));
    }

    #[test]
    fn cannot_disable_dependency_while_dependent_is_enabled() {
        let mut manager = ModManager::new(vec![
            package("example.base", &[], &[]),
            package("example.addon", &[("example.base", "^1.0")], &[]),
        ])
        .unwrap();
        manager.approve("example.base", &[]).unwrap();
        manager.enable("example.base").unwrap();
        manager.approve("example.addon", &[]).unwrap();
        manager.enable("example.addon").unwrap();

        assert_eq!(
            manager.disable("example.base"),
            Err(ModManagerError::ActiveDependent {
                dependency: "example.base".into(),
                dependent: "example.addon".into(),
            })
        );
        manager.disable("example.addon").unwrap();
        manager.disable("example.base").unwrap();
    }

    #[test]
    fn dependency_failure_cascades_to_transitive_dependents() {
        let mut manager = ModManager::new(vec![
            package("example.base", &[], &[]),
            package("example.middle", &[("example.base", "^1.0")], &[]),
            package("example.addon", &[("example.middle", "^1.0")], &[]),
        ])
        .unwrap();
        for id in ["example.base", "example.middle", "example.addon"] {
            manager.approve(id, &[]).unwrap();
            manager.enable(id).unwrap();
        }

        manager.fail("example.base", "host fault").unwrap();
        for id in ["example.base", "example.middle", "example.addon"] {
            assert_eq!(manager.state(id), Ok(LifecycleState::Failed));
        }
    }

    #[test]
    fn rejects_invalid_dependency_graph_before_managing_any_mod() {
        let result = ModManager::new(vec![package(
            "example.addon",
            &[("example.missing", "^1.0")],
            &[],
        )]);
        assert!(matches!(result, Err(ModManagerError::Dependencies(_))));
    }
}
