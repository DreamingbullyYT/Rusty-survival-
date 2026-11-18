# Current State

Last updated: 2026-11-18
Status: DREAMMOD_ARCHITECTURE_PROPOSED; IMPLEMENTATION_NOT_STARTED
Repository HEAD at last inspection: 422a9e3fc57ffce0c2e9399a9b7466a1f15a6e2a (architecture document commit; resolve current main dynamically before next change)

## Direction
- Active project: DreamMod, a deeply extensible interpreted modding runtime for Horizon.
- Build DreamMod Core as a trusted native .hmod mod against a pinned Horizon SDK; use the existing mod API/host and native route. Verify prebuilt library loading without launcher compilation.
- The launcher distributes a prebuilt Horizon package containing DreamMod Core.
- Ordinary .dmod packages are interpreted dynamically; no compilation or bytecode build on player devices.
- Launcher implementation is separate.

## Implemented
- Existing PMCS continuity docs, validator, regression tests and GitHub Actions workflow.
- New docs/specs/DREAMMOD_ARCHITECTURE.md: build-time/runtime separation, integration model, extensibility, performance, security, language evaluation criteria, package draft and phased plan.
- Existing Rust engine-core/testkit, reference tooling and Android Vulkan POC are historical artifacts from the previous project direction; they are not DreamMod functionality.

## Not implemented / not verified
- No DreamMod native .hmod Core or prebuilt library package.
- No interpreter selected or integrated.
- No .dmod manifest/schema, package loader, lifecycle, dependency resolver or hot reload.
- No public DreamMod API, event bus, extension registry or subsystem replacement contract.
- No sandbox, capability enforcement, execution/memory budgets or diagnostics.
- No representative benchmarks or Android runtime validation for DreamMod.
- No reproducible prebuilt Horizon distribution containing DreamMod Core.
- No compatibility tests against a pinned current Horizon revision.

## Next actions
1. Complete the Horizon mod platform audit and pin Horizon, horizon-mixins and hooks/API revisions; prove a CI-built native library can be shipped and loaded without local compilation.
2. Map the exact HNMod integration point and build a minimal Core bootstrap.
3. Evaluate interpreted scripting runtimes against the no-compilation requirement and benchmark representative calls.
4. Define the minimal package manifest, loader lifecycle and capability model.
5. Implement one minimal .dmod end-to-end before broad API expansion.
6. Design extension contracts for adding and replacing systems, then benchmark.
7. Keep launcher work out of this repository.
