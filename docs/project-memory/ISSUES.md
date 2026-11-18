# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## DM-001 — Horizon baseline and integration pin
Status: IMPLEMENTED
Current Core baseline: Horizon Nightly 2026-11-18 `0b14f8acfd97fa2c750e42795249dcc981178ca4`, matching SDK `sdk-0b14f8acfd97fa2c750e42795249dcc981178ca4`, horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, mod API `0.2`, game `26.3`. Re-audit before every upstream compatibility update.

## DM-002 — .hmod build-time integration and prebuilt distribution
Status: INVESTIGATING
The current Core manifest is pinned to the published Nightly and uses the required `capabilities = ["registries"]` field from Horizon's official native template; it has no source mixins or internals. The official `cargo-horizon-mod check` workflow passed in run 44285877944 and uploaded a source `.hmod` artifact. A separate official-launcher prebuilt workflow was added; first run failed because the runner lacked `libxkbcommon-x11.so.0`, and headless GUI dependencies were added in 6bd45dd. Fresh prebuilt run is pending. Remaining work: pass source validation, run native-build, inspect the build folder/inventory and prove clean startup without player-side compilation.

## DM-003 — Interpreted language selection
Status: INVESTIGATING
Added an isolated Rhai 1.27.0 AST-evaluator prototype with Grain bytecode disabled and AST optimization disabled. It is not the production runtime and not the final language decision. Starlark was rejected because its evaluator compiles AST to bytecode. Sprite is a research-only tiny tree-walker reference, too limited to adopt directly. Rhai tests/Clippy passed in 44284795812; a comparable 10k-item batch benchmark was 462.94× slower than native on x86_64 CI. New array/map/string/variable/function budgets and the array-growth test passed in 44285379104. Keep Rhai provisional. Do not add bytecode, JIT or player-side native compilation.

## DM-004 — Deep subsystem replacement boundaries
Status: OPEN
Horizon internals cannot be assumed replaceable merely because hooks exist. Identify extension points and define complete contracts for AI, physics, rendering and other systems.

## DM-005 — Script performance in hot paths
Status: OPEN
Interpreted callbacks are 462.94× slower than Rust in the current synthetic 10k-item Rhai benchmark on x86_64 CI. Do not promise arbitrary per-entity/per-vertex script loops for physics/rendering. Use coarse provider dispatch with Rust-owned batch iteration. A CI-prebuilt native kernel tier could be considered later, but is not implemented and would be separate from interpreted `.dmod` scripts.

## DM-006 — Script sandbox and capabilities
Status: OPEN
Need per-mod isolation, explicit capability grants, exact memory accounting, safe resource access, cancellation and fault containment. The Rhai prototype now tests coarse limits (operations, call depth, aggregate arrays/maps/strings, variables and functions); production enforcement remains open.

## DM-007 — API stability and upstream compatibility
Status: OPEN
DreamMod must pin Horizon/mixins/hooks revisions and version its public API. Define compatibility ranges and migration rules.

## DM-008 — Package format and lifecycle
Status: OPEN
Manifest, permissions, dependency resolution, deterministic load order, enable/disable, reload and error reporting are draft requirements only.

## DM-009 — Historical engine prototype scope
Status: INVESTIGATING
Existing engine-core, testkit, Mojang artifact tooling, Minelib audits and Android Vulkan POC were built for the former independent-engine direction. Preserve them for history; do not treat them as DreamMod implementation or extend them without a new explicit scope decision.


## DM-010 — Reuse existing Horizon mod platform and prebuilt native loading
Status: INVESTIGATING
Horizon already provides a typed mod API, native host, content/native/WASM package routes, hooks, Parts, attachments, capabilities, API injection points and native mixins/bridges/loop slots. Do not duplicate these. The existing WASM runtime uses a child-process Wasmtime compilation path on the player's machine when the module cache is cold, so it does not meet strict interpreted-source/no-compilation requirements. Native .hmod source is normally compiled by the launcher, but the native host also loads adjacent prebuilt libraries with SHA-256 and build identity checks. A CI-built DreamMod Core native library may be shippable without local compilation; this end-to-end path is not yet proven.
Evidence: docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md (source route traced at Horizon `1c45cf...` and rechecked at current Nightly `0b14f8acfd97fa2c750e42795249dcc981178ca4`; horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`).
Required action: build a minimal native .hmod, produce the prebuilt library/inventory/package layout in CI, install it into a clean pinned Horizon distribution without running launcher native-build, and verify successful startup.


## DM-011 — Compile the first DreamMod Core bootstrap against pinned Horizon
Status: VERIFIED
Historical direct API `cargo check` passed in run `44249522292` against an earlier Horizon source pin. The current native package has since moved to the published Nightly and is being checked with the official `cargo-horizon-mod` tool; that check has not passed yet. This issue is therefore INVESTIGATING, not a current verified build.


## DM-012 — Validate the Rhai interpreted-source candidate
Status: INVESTIGATING
Added `mods/dreammod-runtime-prototype` and a dedicated workflow. Rhai 1.27.0 AST evaluation uses `no_optimize` and excludes Grain bytecode. Formatting, four behavior tests, Clippy and resource-limit tests passed (runs 44284795812 and 44285379104). The comparable 10k-item benchmark was 462.94× slower than native on x86_64 CI. Starlark was rejected because it compiles AST to bytecode; Sprite remains research-only due to its tiny feature set. Keep Rhai provisional, design coarse provider APIs and benchmark ARM64 before any language decision.
