# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## DM-001 — Horizon baseline and integration pin
Status: IMPLEMENTED
Initial baseline pinned for the Core bootstrap: Horizon `1c45cf95d8431a7b49d94df007f7e75a890370c9`, horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, mod API `0.2`, game `26.3`. Re-audit before every upstream compatibility update; this is not a claim of compatibility with newer Horizon revisions.

## DM-002 — .hmod build-time integration and prebuilt distribution
Status: INVESTIGATING
The minimal `.hmod` Core source compiles against the pinned Horizon API in CI (run 44249522292). This is only `cargo check`, not a native library build or game launch. Pinned Horizon source confirms the Route::Api path for API-only mods: it compiles mod dylibs against prebuilt runtime/API slices and returns the unchanged game executable plus libraries; installation writes native-mods.json, which the host validates and loads. Remaining work: run the official native-build pipeline in CI, inspect the output/inventory and prove clean startup without player-side compilation.

## DM-003 — Interpreted language selection
Status: INVESTIGATING
Added an isolated Rhai 1.27.0 AST-evaluator prototype with Grain bytecode disabled and AST optimization disabled. It is not the production runtime and not the final language decision. Compare with Starlark 0.14.x and verify source parsing, execution model, Android/ARM64, throughput, API ergonomics, sandboxing, diagnostics and maintenance. Do not add bytecode, JIT or player-side native compilation.

## DM-004 — Deep subsystem replacement boundaries
Status: OPEN
Horizon internals cannot be assumed replaceable merely because hooks exist. Identify extension points and define complete contracts for AI, physics, rendering and other systems.

## DM-005 — Script performance in hot paths
Status: OPEN
Interpreted callbacks may be too slow at per-entity/per-vertex frequency. Use batching/coarse-grained dispatch and benchmark representative workloads before promising full replacement.

## DM-006 — Script sandbox and capabilities
Status: OPEN
Need per-mod isolation, explicit capability grants, memory/execution budgets, safe resource access, cancellation and fault containment.

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
Evidence: docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md (Horizon 1c45cf95d8431a7b49d94df007f7e75a890370c9; horizon-mixins e59da9094f0c2fdda6c627fe9eff22dec6a906f0).
Required action: build a minimal native .hmod, produce the prebuilt library/inventory/package layout in CI, install it into a clean pinned Horizon distribution without running launcher native-build, and verify successful startup.


## DM-011 — Compile the first DreamMod Core bootstrap against pinned Horizon
Status: VERIFIED
The minimal native `.hmod` source package under `mods/dreammod-core` passed `cargo check` against the exact Horizon source commit pinned in `horizon-mod.toml`. GitHub Actions run `44249522292`, job `134616491803`, step `Compile against Horizon mod API`: success. This verifies source/API compilation only; it does not build a distributable native library or launch the game.


## DM-012 — Validate the Rhai interpreted-source candidate
Status: INVESTIGATING
Added `mods/dreammod-runtime-prototype` and a dedicated workflow. The prototype uses Rhai 1.27.0 with `no_optimize` and without the optional Grain bytecode feature; it exercises event-style calls, batch host APIs, a script-side batch physics algorithm and operation limits. Await CI results, compare Starlark, inspect memory/CPU behavior and test ARM64 before any language decision.
