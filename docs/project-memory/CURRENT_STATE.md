# Current State

Last updated: 2026-11-18
Status: CORE_BOOTSTRAP_CI_VERIFIED; RHAI_CANDIDATE_PROTOTYPE_IMPLEMENTED; RUNTIME_CI_REPAIR_IN_PROGRESS
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
- Added mods/dreammod-core as a minimal native .hmod Core bootstrap using Horizon's typed Hooks API.
- Added .github/workflows/dreammod-core.yml to compile the Core source against the exact pinned Horizon source checkout.
- Verified Core CI run 44249522292 on commit d6ca63ecea51facfaeda46e37e15e0664d188814: the pinned Horizon API `cargo check` passed.
- Added `mods/dreammod-runtime-prototype`, an isolated Rhai AST-evaluator candidate with event dispatch, batch API, subsystem-algorithm and operation-budget tests.
- Added `.github/workflows/dreammod-runtime-prototype.yml` to run candidate tests and a coarse desktop benchmark.
- Existing Rust engine-core/testkit, reference tooling and Android Vulkan POC are historical artifacts from the previous project direction; they are not DreamMod functionality.

## Not implemented / not verified
- The minimal native `.hmod` source skeleton exists and its pinned-source API compile check is VERIFIED in CI run 44249522292.
- The Rhai candidate prototype is IMPLEMENTED. The first behavior test run exposed Rust Vec versus Rhai Array boundary errors; the conversion fix is committed, but the latest workflow stopped at a rustfmt import-order check before rerunning behavior tests. It is not selected as the final language and is not integrated into DreamMod Core.
- Pinned Horizon source now confirms the API-only Route::Api build path: Core's current Hooks-only manifest qualifies, and the route returns the unchanged prebuilt client plus native libraries. No prebuilt native library package has yet been produced or loaded.
- No production interpreter selected or integrated; the isolated Rhai candidate is only an evaluation prototype.
- No .dmod manifest/schema, package loader, lifecycle, dependency resolver or hot reload.
- No public DreamMod API, event bus, extension registry or subsystem replacement contract.
- No sandbox, capability enforcement, execution/memory budgets or diagnostics.
- No representative benchmarks or Android runtime validation for DreamMod.
- No reproducible prebuilt Horizon distribution containing DreamMod Core.
- No compatibility tests against a pinned current Horizon revision.

## Next actions
1. Verify the corrected Rhai candidate workflow after the import-order formatting fix; resolve any remaining test, Clippy or benchmark failures.
2. Use the pinned official native-build pipeline to produce and inspect an API-only prebuilt distribution, then prove clean startup.
3. Compare Rhai and Starlark against the no-bytecode/no-JIT requirement and representative workloads.
4. Define the minimal package manifest, loader lifecycle and capability model.
5. Implement one minimal .dmod end-to-end before broad API expansion.
6. Design extension contracts for adding and replacing systems, then benchmark.
7. Keep launcher work out of this repository.
