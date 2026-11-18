# Current State

Last updated: 2026-11-18
Status: DREAMMOD_CORE_SKELETON_ADDED; CI_VERIFICATION_PENDING
Repository HEAD: resolve dynamically from current main; last inspected project audit commit 21ff8bd96672d0304533170781f9f5f8d05d36aa

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
- Existing Rust engine-core/testkit, reference tooling and Android Vulkan POC are historical artifacts from the previous project direction; they are not DreamMod functionality.

## Not implemented / not verified
- Minimal native .hmod source skeleton exists; CI compile result is pending.
- No prebuilt native library package has been produced or loaded.
- No interpreter selected or integrated.
- No .dmod manifest/schema, package loader, lifecycle, dependency resolver or hot reload.
- No public DreamMod API, event bus, extension registry or subsystem replacement contract.
- No sandbox, capability enforcement, execution/memory budgets or diagnostics.
- No representative benchmarks or Android runtime validation for DreamMod.
- No reproducible prebuilt Horizon distribution containing DreamMod Core.
- No compatibility tests against a pinned current Horizon revision.

## Next actions
1. Get the DreamMod Core API-check workflow green against the pinned Horizon source.
2. Prove a CI-built native library can be shipped and loaded without launcher-side compilation.
3. Evaluate interpreted scripting runtimes against the no-compilation requirement and benchmark representative calls.
4. Define the minimal package manifest, loader lifecycle and capability model.
5. Implement one minimal .dmod end-to-end before broad API expansion.
6. Design extension contracts for adding and replacing systems, then benchmark.
7. Keep launcher work out of this repository.
