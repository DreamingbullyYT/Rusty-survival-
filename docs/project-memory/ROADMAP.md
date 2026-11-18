# Roadmap

## Phase 0 — Project continuity
- [x] Preserve PMCS, validator, regression tests and GitHub Actions workflow.
- [x] Record the 2026-11-18 DreamMod project pivot in persistent memory.
- [x] Define the initial DreamMod architecture and prebuilt-distribution model.

## Phase 1 — Horizon integration research
- [x] Audit current Horizon mod API/host, package formats, native route and WASM runtime.
- [x] Pin initial Horizon and horizon-mixins revisions in the Core manifest.
- [x] Add minimal native .hmod Core source and pinned-source compile workflow.
- [x] Source-verify Horizon's API-only prebuilt native library route (the CI artifact and clean startup proof remain open).
- [ ] Produce, inspect and launch a CI-built API-only native distribution.
- [x] Pin Horizon revision, horizon-mixins revision and API version for the bootstrap.
- [x] Get the minimal Core bootstrap CI compile check passing (Actions run 44249522292; pinned Horizon API cargo check succeeded).
- [ ] Produce and verify a prebuilt Horizon package containing the bootstrap.
- [ ] Confirm ordinary installation does not invoke native mod compilation on the player device.

## Phase 2 — Runtime decision and prototype
- [x] Establish initial candidate shortlist (Rhai AST evaluator and Starlark AST interpreter); final selection remains OPEN.
- [x] Add an isolated Rhai AST-evaluator prototype with typed test host calls and an explicit operation budget.
- [x] Add tests for event-style dispatch, batch world query and script-side batch physics replacement.
- [ ] Verify the new runtime prototype CI workflow and fix any failures.
- [ ] Compare Starlark with the same representative workloads.
- [ ] Benchmark event dispatch, batch world/entity queries and coarse subsystem replacement on desktop and ARM64 devices.
- [ ] Select a runtime only after performance, safety and Android/target compatibility evidence.
- [ ] Define execution budgets, memory quotas, cancellation and diagnostics.

## Phase 3 — Core runtime
- [ ] Define .dmod manifest and permissions schema.
- [ ] Implement package validation and safe discovery.
- [ ] Implement interpreted runtime lifecycle, dependencies and deterministic load order.
- [ ] Implement sandbox and capability enforcement.
- [ ] Load and run one minimal example .dmod end-to-end.

## Phase 4 — Extensible engine API
- [ ] Define stable versioned APIs for events, gameplay, world, entities, assets, UI and lifecycle.
- [ ] Implement system registration and ordering.
- [ ] Define subsystem provider contracts and override priority/fallback.
- [ ] Add AI and physics replacement prototypes with correctness/performance tests.
- [ ] Add rendering extension/replacement only after a viable batched contract is proven.
- [ ] Add compatibility tests against pinned and candidate Horizon revisions.

## Phase 5 — Release engineering
- [ ] Reproducible build pipeline for prebuilt Horizon + DreamMod Core.
- [ ] Generate compatibility metadata and checksums.
- [ ] Verify package update/reload behavior independently of Core rebuilds.
- [ ] Document trust, permissions, API compatibility and mod-author tooling.

## Exit criteria before engine implementation
This heading is retained for PMCS compatibility; the former standalone engine exit criteria are superseded. Active implementation may begin after Phase 1 integration points and the runtime prototype plan are evidence-backed.
