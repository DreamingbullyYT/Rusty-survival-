//! Deterministic owner for validated packages and their per-mod lifecycles.
//! This layer does not execute scripts or touch Horizon handles.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    lifecycle::{LifecycleError, LifecycleState, ModLifecycle},
    resolve_load_order, valid_archive_path, ArchiveEntryKind, DependencyError, Manifest,
    ValidatedPackage,
    resources::{ResourceError, ResourceRegistry},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModManagerError {
    Dependencies(DependencyError),
    UnknownMod(String),
    DependencyNotEnabled { mod_id: String, dependency: String },
    ActiveDependent { dependency: String, dependent: String },
    InvalidAssetPath,
    AssetNotFound(String),
    EntrypointNotFound(String),
    Lifecycle(LifecycleError),
    Resource(ResourceError),
    ResourceNamespaceMismatch { mod_id: String, resource_id: String },
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
    resources: ResourceRegistry,
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
            resources: ResourceRegistry::new(),
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

    /// Check one mod's enabled state and explicit grant before a Core callback.
    /// Callers can pass this method as the authorization closure to Core APIs.
    pub fn require_capability(
        &self,
        id: &str,
        capability: &str,
    ) -> Result<(), ModManagerError> {
        let managed = self
            .mods
            .get(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        managed
            .lifecycle
            .require(capability)
            .map_err(ModManagerError::Lifecycle)
    }

    /// Borrow the validated entrypoint source for the trusted interpreter.
    /// A script cannot be obtained from a package until its mod is Enabled.
    pub fn entrypoint_source(&self, id: &str) -> Result<&[u8], ModManagerError> {
        let managed = self
            .mods
            .get(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        managed
            .lifecycle
            .ensure_enabled()
            .map_err(ModManagerError::Lifecycle)?;
        let entrypoint = &managed.package.manifest.entrypoint;
        managed
            .package
            .files
            .iter()
            .find(|file| &file.path == entrypoint)
            .map(|file| file.contents.as_slice())
            .ok_or_else(|| ModManagerError::EntrypointNotFound(entrypoint.clone()))
    }

    /// Read an already-validated asset without extracting it from the package.
    /// The requesting mod must be Enabled and hold the explicit assets.read grant.
    pub fn read_asset(&self, id: &str, path: &str) -> Result<&[u8], ModManagerError> {
        let managed = self
            .mods
            .get(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        managed
            .lifecycle
            .require("assets.read")
            .map_err(ModManagerError::Lifecycle)?;

        if !path.starts_with("assets/")
            || !valid_archive_path(path, ArchiveEntryKind::File)
        {
            return Err(ModManagerError::InvalidAssetPath);
        }

        managed
            .package
            .files
            .iter()
            .find(|file| file.path == path)
            .map(|file| file.contents.as_slice())
            .ok_or_else(|| ModManagerError::AssetNotFound(path.to_owned()))
    }

    /// Atomically reserve resource identifiers for an enabled mod.
    /// This is host-independent bookkeeping; it does not publish Horizon resources.
    pub fn register_resource_ids(
        &mut self,
        id: &str,
        resource_ids: &[String],
    ) -> Result<Vec<String>, ModManagerError> {
        let managed = self
            .mods
            .get(id)
            .ok_or_else(|| ModManagerError::UnknownMod(id.to_owned()))?;
        managed
            .lifecycle
            .ensure_enabled()
            .map_err(ModManagerError::Lifecycle)?;

        let namespace_prefix = format!("{id}:");
        let mut transaction = self
            .resources
            .begin(id.to_owned())
            .map_err(ModManagerError::Resource)?;
        for resource_id in resource_ids {
            if !resource_id.starts_with(&namespace_prefix) {
                return Err(ModManagerError::ResourceNamespaceMismatch {
                    mod_id: id.to_owned(),
                    resource_id: resource_id.clone(),
                });
            }
            transaction
                .stage(resource_id.clone())
                .map_err(ModManagerError::Resource)?;
        }
        transaction
            .commit(&mut self.resources)
            .map_err(ModManagerError::Resource)
    }

    /// Return this mod's currently reserved identifiers in deterministic order.
    pub fn registered_resource_ids(&self, id: &str) -> Result<Vec<String>, ModManagerError> {
        if !self.mods.contains_key(id) {
            return Err(ModManagerError::UnknownMod(id.to_owned()));
        }
        Ok(self.resources.ids_for_owner(id))
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
            .map_err(ModManagerError::Lifecycle)?;
        self.resources.remove_owner(id);
        Ok(())
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
        self.resources.remove_owner(id);

        let mut failed = BTreeSet::from([id.to_owned()]);
        for dependent in &self.load_order {
            if failed.contains(dependent) {
                continue;
            }
            let depends_on_failed = self.dependencies[dependent]
                .iter()
                .any(|dependency| failed.contains(dependency));
            if depends_on_failed {
                if self.state(dependent)? != LifecycleState::Failed {
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
                }
                self.resources.remove_owner(dependent);
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
    fn entrypoint_source_is_borrowed_only_after_activation() {
        let mut script_package = package("example.script", &[], &[]);
        script_package.files[0].contents = b"print('hello')".to_vec();
        let mut manager = ModManager::new(vec![script_package]).unwrap();

        assert_eq!(
            manager.entrypoint_source("example.script"),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager.approve("example.script", &[]).unwrap();
        assert_eq!(
            manager.entrypoint_source("example.script"),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager.enable("example.script").unwrap();
        assert_eq!(
            manager.entrypoint_source("example.script"),
            Ok(&b"print('hello')"[..])
        );
    }

    #[test]
    fn asset_reads_require_enabled_state_and_assets_read_capability() {
        let mut asset_package = package("example.assets", &[], &["assets.read"]);
        asset_package.files.push(PackageFile {
            path: "assets/textures/icon.bin".into(),
            contents: vec![1, 2, 3, 4],
        });
        let mut manager = ModManager::new(vec![asset_package]).unwrap();

        assert_eq!(
            manager.read_asset("example.assets", "assets/textures/icon.bin"),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager.approve("example.assets", &[]).unwrap();
        manager.enable("example.assets").unwrap();
        assert_eq!(
            manager.read_asset("example.assets", "assets/textures/icon.bin"),
            Err(ModManagerError::Lifecycle(LifecycleError::CapabilityDenied(
                "assets.read".into()
            )))
        );

        let mut granted = package("example.assets", &[], &["assets.read"]);
        granted.files.push(PackageFile {
            path: "assets/textures/icon.bin".into(),
            contents: vec![1, 2, 3, 4],
        });
        let mut manager = ModManager::new(vec![granted]).unwrap();
        manager.approve("example.assets", &["assets.read".into()]).unwrap();
        manager.enable("example.assets").unwrap();
        assert_eq!(
            manager.read_asset("example.assets", "assets/textures/icon.bin"),
            Ok(&[1, 2, 3, 4][..])
        );
        assert_eq!(
            manager.read_asset("example.assets", "assets/../secret"),
            Err(ModManagerError::InvalidAssetPath)
        );
        assert_eq!(
            manager.read_asset("example.assets", "assets/missing.bin"),
            Err(ModManagerError::AssetNotFound("assets/missing.bin".into()))
        );
    }

    #[test]
    fn capability_check_can_be_used_as_core_authorization_callback() {
        let mut manager = ModManager::new(vec![package(
            "example.reader",
            &[],
            &["entities.read"],
        )])
        .unwrap();
        assert_eq!(
            manager.require_capability("example.reader", "entities.read"),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager
            .approve("example.reader", &["entities.read".into()])
            .unwrap();
        assert_eq!(
            manager.require_capability("example.reader", "entities.read"),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager.enable("example.reader").unwrap();
        assert_eq!(
            manager.require_capability("example.reader", "entities.read"),
            Ok(())
        );
        assert_eq!(
            manager.require_capability("example.reader", "world.write"),
            Err(ModManagerError::Lifecycle(LifecycleError::CapabilityDenied(
                "world.write".into()
            )))
        );
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
    fn failure_cascade_handles_already_failed_intermediate_mod() {
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

        manager.fail("example.middle", "independent fault").unwrap();
        manager.fail("example.base", "base fault").unwrap();
        for id in ["example.base", "example.middle", "example.addon"] {
            assert_eq!(manager.state(id), Ok(LifecycleState::Failed));
        }
    }

    #[test]
    fn resource_ids_register_only_for_enabled_mods_and_in_own_namespace() {
        let mut manager = ModManager::new(vec![package("example.assets", &[], &[])])
            .unwrap();
        let ids = vec!["example.assets:block/stone".to_owned()];

        assert_eq!(
            manager.register_resource_ids("example.assets", &ids),
            Err(ModManagerError::Lifecycle(LifecycleError::NotEnabled))
        );
        manager.approve("example.assets", &[]).unwrap();
        manager.enable("example.assets").unwrap();
        assert_eq!(
            manager.register_resource_ids(
                "example.assets",
                &["other.mod:block/stone".to_owned()]
            ),
            Err(ModManagerError::ResourceNamespaceMismatch {
                mod_id: "example.assets".into(),
                resource_id: "other.mod:block/stone".into(),
            })
        );
        assert_eq!(
            manager.register_resource_ids("example.assets", &ids),
            Ok(ids.clone())
        );
        assert_eq!(
            manager.registered_resource_ids("example.assets"),
            Ok(ids)
        );
    }

    #[test]
    fn resource_registration_is_atomic_and_rejects_cross_mod_conflicts() {
        let mut manager = ModManager::new(vec![
            package("example.first", &[], &[]),
            package("example.second", &[], &[]),
        ])
        .unwrap();
        for id in ["example.first", "example.second"] {
            manager.approve(id, &[]).unwrap();
            manager.enable(id).unwrap();
        }
        manager
            .register_resource_ids(
                "example.first",
                &["example.first:shared".to_owned()],
            )
            .unwrap();

        assert_eq!(
            manager.register_resource_ids(
                "example.second",
                &[
                    "example.second:new".to_owned(),
                    "example.first:shared".to_owned(),
                ],
            ),
            Err(ModManagerError::ResourceNamespaceMismatch {
                mod_id: "example.second".into(),
                resource_id: "example.first:shared".into(),
            })
        );
        assert_eq!(
            manager.registered_resource_ids("example.second"),
            Ok(Vec::new())
        );
    }

    #[test]
    fn disable_releases_all_resource_ids_owned_by_mod() {
        let mut manager = ModManager::new(vec![package("example.assets", &[], &[])])
            .unwrap();
        manager.approve("example.assets", &[]).unwrap();
        manager.enable("example.assets").unwrap();
        manager
            .register_resource_ids(
                "example.assets",
                &[
                    "example.assets:item/a".to_owned(),
                    "example.assets:block/a".to_owned(),
                ],
            )
            .unwrap();

        assert_eq!(manager.registered_resource_ids("example.assets").unwrap().len(), 2);
        manager.disable("example.assets").unwrap();
        assert_eq!(manager.registered_resource_ids("example.assets"), Ok(Vec::new()));
    }

    #[test]
    fn dependency_failure_releases_resources_for_entire_dependent_chain() {
        let mut manager = ModManager::new(vec![
            package("example.base", &[], &[]),
            package("example.addon", &[( "example.base", "^1.0")], &[]),
        ])
        .unwrap();
        for id in ["example.base", "example.addon"] {
            manager.approve(id, &[]).unwrap();
            manager.enable(id).unwrap();
            manager
                .register_resource_ids(
                    id,
                    &[format!("{id}:item/registered")],
                )
                .unwrap();
        }

        manager.fail("example.base", "host fault").unwrap();
        assert_eq!(manager.registered_resource_ids("example.base"), Ok(Vec::new()));
        assert_eq!(manager.registered_resource_ids("example.addon"), Ok(Vec::new()));
        assert_eq!(manager.state("example.base"), Ok(LifecycleState::Failed));
        assert_eq!(manager.state("example.addon"), Ok(LifecycleState::Failed));
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
