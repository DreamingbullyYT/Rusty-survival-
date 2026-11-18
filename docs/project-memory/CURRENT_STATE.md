# Current State

Last updated: 2026-11-18
Status: CORE_NATIVE_PACKAGE_ADDED; NIGHTLY_SDK_PINNED; CORE_PACKAGE_CI_PENDING; RHAI_CANDIDATE_TESTS_AND_CLIPPY_VERIFIED
Repository HEAD: resolve dynamically from current main; last inspected commit before this work: 25d839d79b5238207ed9829f202eae5cd947ffcb

## Direction
- Active project: DreamMod, a deeply extensible interpreted modding runtime for Horizon.
- Build DreamMod Core as a trusted native .hmod mod against a pinned Horizon SDK; use the existing mod API/host and native route. Verify prebuilt library loading without launcher compilation.
- The launcher distributes a prebuilt Horizon package containing DreamMod Core.
- Ordinary .dmod packages are interpreted dynamically; no compilation or bytecode build on player devices.
- Launcher implementation is separate.

## Implemented
- Existing PMCS continuity docs, validator, regression tests and GitHub Actions workflow.
- New docs/specs/DREAMMOD_ARCHITECTURE.md: build-time/runtime separation, integration model, extensibility, performance, security, language evaluation criteria, package draft and phased plan.
- New docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md: current Horizon mod API/host, native and WASM routes, pinned revisions, and prebuilt-library feasibility.
- Added mods/dreammod-core as a native trait mod source package: horizon-mod.toml (API 0.2, no capabilities, [native] pinned to the published 2026-11-18 Nightly and matching horizon-mixins revision), Cargo.toml and typed Hooks bootstrap.
- Updated .github/workflows/dreammod-core.yml to install the pinned cargo-horizon-mod tool, check the native package/source/API use against the pinned Nightly SDK, compile host and wasm32 targets, package a source .hmod and upload it as a short-lived CI artifact. Workflow run 44284751206 is pending.
- Historical direct API check run 44249522292 passed against the earlier source pin; the new official package check/build workflow is the active verification gate.
- Added `mods/dreammod-runtime-prototype`, an isolated Rhai AST-evaluator candidate with event dispatch, batch API, subsystem-algorithm and operation-budget tests.
- Added `.github/workflows/dreammod-runtime-prototype.yml` to run candidate tests and a coarse desktop benchmark.
- Existing Rust engine-core/testkit, reference tooling and Android Vulkan POC are historical artifacts from the previous project direction; they are not DreamMod functionality.

## Not implemented / not verified
- The minimal native `.hmod` source skeleton exists and its pinned-source API compile check is VERIFIED in CI run 44249522292.
- The Rhai candidate prototype is IMPLEMENTED. CI run 44283982705 passed rustfmt, all 4 behavior/budget tests and Clippy. Its benchmark ran, but review showed the Rust-native workload was optimized away (371 ns for 500,000 outputs), so that timing is invalid. Commit 89ed3a5 added black_box guards; the follow-up rustfmt check found a line-wrap issue, corrected in 00a3360. A fresh benchmark run is pending. It is not selected as the final language and is not integrated into DreamMod Core.
- Pinned Horizon source now confirms the API-only Route::Api build path: Core's current Hooks-only manifest qualifies, and the route returns the unchanged prebuilt client plus native libraries. No prebuilt native library package has yet been produced or loaded.
- No production interpreter selected or integrated; the isolated Rhai candidate is only an evaluation prototype.
- No .dmod manifest/schema, package loader, lifecycle, dependency resolver or hot reload.
- No public DreamMod API, event bus, extension registry or subsystem replacement contract.
- No sandbox, capability enforcement, execution/memory budgets or diagnostics.
- No representative benchmarks or Android runtime validation for DreamMod.
- No reproducible prebuilt Horizon distribution containing DreamMod Core.
- No compatibility tests against a pinned current Horizon revision.

## Next actions
1. Verify the new official native package workflow (44284751206), fix any issues, and inspect the generated source .hmod artifact.
2. Verify the black_box-protected benchmark (latest run 44284795812) and record diagnostic desktop timings; do not use the old 371 ns Rust result.
2. Use the pinned official native-build pipeline to produce and inspect an API-only prebuilt distribution, then prove clean startup.
3. Compare Rhai and Starlark against the no-bytecode/no-JIT requirement and representative workloads.
4. Define the minimal package manifest, loader lifecycle and capability model.
5. Implement one minimal .dmod end-to-end before broad API expansion.
6. Design extension contracts for adding and replacing systems, then benchmark.
7. Keep launcher work out of this repository.
