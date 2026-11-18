from pathlib import Path

ROOT = Path(".ci/horizon")

NATIVE_MODS_TEST = r"""

#[cfg(test)]
mod dreammod_api_route_ci {
    use super::*;
    use horizon_platform::mod_package::dependencies;
    use horizon_platform::mod_package::manifest::Side;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[test]
    #[ignore = "CI-only: builds the API-only route from the signed SDK slices"]
    fn build_prebuilt_api_only_client_overlay() {
        let mod_dir = PathBuf::from(env::var_os("DREAMMOD_MOD_DIR").expect("DREAMMOD_MOD_DIR"));
        let output = PathBuf::from(
            env::var_os("DREAMMOD_PREBUILT_DIR").expect("DREAMMOD_PREBUILT_DIR"),
        );
        let build_id = env::var("DREAMMOD_HORIZON_BUILD").expect("DREAMMOD_HORIZON_BUILD");

        // Use Horizon's own signed SDK index and slice unpacker. No Minecraft assets or account
        // are involved: prebuilt::Route::Api needs only the signed runtime and API slices.
        let base = sdk_cache::Base::from_env();
        let keys = horizon_release_sign::sdk_keys_for("horizon-rust/horizon");
        let sdk = sdk_cache::index(&base, &build_id, &keys).expect("verify pinned SDK");
        assert_eq!(sdk.commit(), build_id);
        let _lease = crate::native_cache::Cache::acquire(&crate::backend::build_dir())
            .expect("acquire native build cache");
        let package = sdk_cache::slices(
            &_lease,
            &sdk,
            &base,
            prebuilt::Route::Api,
            Target::Client,
            &|_| {},
        )
        .expect("download and verify API-only runtime/API slices");

        let bundle = bundle_in(&mod_dir)
            .expect("read DreamMod Core package")
            .expect("DreamMod Core native package");
        let project = api::Project::read(&bundle).expect("read native API project");
        let declared = vec![dependencies::from_manifest(
            &bundle.manifest.dependencies,
            Side::Client,
        )];
        let resolution = dependencies::resolve_manifests(
            &[&bundle.manifest],
            Side::Client,
            &dependencies::Overrides::default(),
        )
        .expect("resolve mod dependencies");
        let projects = vec![project];

        let rustc = Command::new("rustup")
            .args(["which", "rustc"])
            .output()
            .expect("find pinned rustc");
        assert!(rustc.status.success(), "rustup which rustc failed");
        let rustc = String::from_utf8(rustc.stdout).expect("rustc path is UTF-8");
        let toolchain_dir = Path::new(rustc.trim())
            .parent()
            .and_then(Path::parent)
            .expect("rustc is inside a toolchain bin directory")
            .to_owned();
        let rust = Toolchain {
            dir: toolchain_dir,
            host: rust_toolchain::host().expect("supported Rust host"),
        };

        fs::create_dir_all(&output).expect("create prebuilt output");
        let builds = root().join("prebuilt-build");
        let (executable, installed) = prebuilt::compile(
            &rust,
            &package,
            &builds,
            &mod_dir,
            prebuilt::Route::Api,
            &projects,
            &declared,
            &resolution,
            None,
            &|_, _, _| {},
        )
        .expect("compile DreamMod through Horizon prebuilt::Route::Api");

        fs::copy(&executable, output.join(backend::CLIENT)).expect("copy prebuilt client");
        for library in &installed {
            let name = library.file_name().expect("native library file name");
            fs::copy(library, output.join(name)).expect("copy adjacent runtime/mod library");
        }

        let records = library_record::collect(&installed).expect("collect native library records");
        assert_eq!(records.len(), 1, "exactly one DreamMod Core library is expected");
        let record = serde_json::json!({ "libraries": records });
        fs::write(
            output.join("native-mods.json"),
            serde_json::to_vec_pretty(&record).expect("serialize native-mods.json"),
        )
        .expect("write native-mods.json");
    }
}
"""

LOADER_TEST = r"""

#[cfg(test)]
mod dreammod_ci_loader_test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    #[ignore = "CI-only: loads the API-route artifact through Horizon's real native host"]
    fn loads_dreammod_prebuilt_overlay() {
        let folder = PathBuf::from(
            std::env::var_os("DREAMMOD_PREBUILT_DIR").expect("DREAMMOD_PREBUILT_DIR"),
        );
        let loaded = load(&folder).expect("Horizon native host must load the prebuilt library");
        assert_eq!(loaded.len(), 1, "DreamMod Core should register one native entry");
    }
}
"""

def append_once(path: Path, marker: str, snippet: str) -> None:
    source = path.read_text()
    if marker in source:
        return
    path.write_text(source.rstrip() + snippet + "\n")

append_once(
    ROOT / "launcher/src/native_mods.rs",
    "mod dreammod_api_route_ci",
    NATIVE_MODS_TEST,
)
append_once(
    ROOT / "crates/horizon-mod-host/src/native_libraries.rs",
    "mod dreammod_ci_loader_test",
    LOADER_TEST,
)
