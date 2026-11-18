# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## DM-001 — Horizon baseline and integration pin
Status: IMPLEMENTED
Current Core baseline: Horizon Nightly 2026-11-18 `0b14f8acfd97fa2c750e42795249dcc981178ca4`, matching SDK `sdk-0b14f8acfd97fa2c750e42795249dcc981178ca4`, horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, mod API `0.2`, game `26.3`. Re-audit before every upstream compatibility update.

## DM-002 — .hmod build-time integration and prebuilt distribution
Status: INVESTIGATING
The current Core manifest is pinned to the published Nightly and is API-only (`capabilities = []`, no native source patches or internals). A new workflow uses pinned `cargo-horizon-mod` to validate, compile and package the source `.hmod`; run 44284751206 is still building the tool. The official `horizon-launcher native-build` command can build this mod against the installed Nightly SDK in trusted CI. Remaining work: run that pipeline, inspect the native build folder/inventory and prove clean startup without player-side compilation.

## DM-003 — Interpreted language selection
Status: INVESTIGATING
Added an isolated Rhai 1.27.0 AST-evaluator prototype with Grain bytecode disabled and AST optimization disabled. It is not the production runtime and not the final language decision. Starlark was rejected because its evaluator compiles AST to bytecode. Sprite is a research-only tiny tree-walker reference, too limited to adopt directly. Rhai tests/Clippy passed in 44284795812; a comparable 10k-item batch benchmark was 462.94× slower than native on x86_64 CI. New array/map/string/variable/function budgets and an array-growth test were added; verify them in 44285379104. Keep Rhai provisional. Do not add bytecode, JIT or player-side native compilation.

## DM-004 — Deep subsystem replacement boundaries
Status: OPEN
Horizon internals cannot be assumed replaceable merely because hooks exist. Identify extension points and define complete contracts for AI, physics, rendering and other systems.

## DM-005 — Script performance in hot paths
Status: OPEN
Interpreted callbacks may be too slow at per-entity/per-vertex frequency. Use batching/coarse-grained dispatch and benchmark representative workloads before promising full replacement.

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
The minimal native `.hmod` source package under `mods/dreammod-core` passed `cargo check` against the exact Horizon source commit pinned in `horizon-mod.toml`. GitHub Actions run `44249522292`, job `134616491803`, step `Compile against Horizon mod API`: success. This verifies source/API compilation only; it does not build a distributable native library or launch the game.


## DM-012 — Validate the Rhai interpreted-source candidate
Status: INVESTIGATING
Added `mods/dreammod-runtime-prototype` and a dedicated workflow. The prototype uses Rhai 1.27.0 with `no_optimize` and without the optional Grain bytecode feature; it exercises event-style calls, batch host APIs, a script-side batch physics algorithm and operation limits. Await CI results, compare Starlark, inspect memory/CPU behavior and test ARM64 before any language decision.
