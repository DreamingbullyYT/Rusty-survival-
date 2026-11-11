use hnmod_core::{ModuleDescriptor, ModuleRegistry, ProjectManifest, RegistryError};

const MANIFEST: &str = r#"
[project]
id = "hnmod"
name = "HNMod"
version = "0.1.0"

[[modules]]
id = "hnmod.core"
path = "crates/hnmod-core"

[[modules]]
id = "hnmod.earth"
path = "modules/earth"
requires = ["hnmod.core"]
"#;

#[test]
fn parses_and_validates_project_manifest() {
    let manifest = ProjectManifest::from_str(MANIFEST).expect("manifest should parse");
    assert_eq!(manifest.project.id, "hnmod");
    assert_eq!(manifest.modules.len(), 2);
}

#[test]
fn rejects_duplicate_module_ids() {
    let input = MANIFEST.replace(
        "path = \"modules/earth\"",
        "path = \"modules/earth\"\nid = \"hnmod.core\""
    );
    assert!(ProjectManifest::from_str(&input).is_err());
}

#[test]
fn resolves_dependencies_before_dependents() {
    let mut registry = ModuleRegistry::new();
    registry.register(ModuleDescriptor {
        id: "hnmod.earth".into(),
        path: "modules/earth".into(),
        requires: vec!["hnmod.core".into()],
    }).unwrap();
    registry.register(ModuleDescriptor {
        id: "hnmod.core".into(),
        path: "crates/hnmod-core".into(),
        requires: vec![],
    }).unwrap();

    let order = registry.resolve_load_order().unwrap();
    assert_eq!(order, vec!["hnmod.core", "hnmod.earth"]);
}

#[test]
fn reports_missing_dependencies() {
    let mut registry = ModuleRegistry::new();
    registry.register(ModuleDescriptor {
        id: "hnmod.earth".into(),
        path: "modules/earth".into(),
        requires: vec!["hnmod.core".into()],
    }).unwrap();

    assert!(matches!(
        registry.resolve_load_order(),
        Err(RegistryError::MissingDependency { .. })
    ));
}

#[test]
fn reports_dependency_cycles() {
    let mut registry = ModuleRegistry::new();
    registry.register(ModuleDescriptor {
        id: "hnmod.a".into(),
        path: "a".into(),
        requires: vec!["hnmod.b".into()],
    }).unwrap();
    registry.register(ModuleDescriptor {
        id: "hnmod.b".into(),
        path: "b".into(),
        requires: vec!["hnmod.a".into()],
    }).unwrap();

    assert!(matches!(registry.resolve_load_order(), Err(RegistryError::DependencyCycle(_))));
}
