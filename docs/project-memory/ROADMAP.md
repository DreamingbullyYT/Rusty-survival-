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
- [x] Pass official cargo horizon-mod check and package a native source .hmod in CI (run 44285877944; artifact inspected).
- [ ] Produce and inspect a CI-built API-only client engine/native-library overlay using signed SDK slices and Horizon's real host loader; current run 44288685182 pending; no Mojang assets are included.
- [ ] Combine the overlay with a matching licensed Minecraft 26.3 installation for a complete playable client; never bypass the official ownership check.
- [x] Pin Horizon Nightly 2026-11-18, horizon-mixins revision and API version in the native trait mod manifest, including Horizon's required `registries` capability.
- [x] Get the minimal Core bootstrap CI compile check passing (Actions run 44249522292; pinned Horizon API cargo check succeeded).
- [ ] Produce and verify a prebuilt Horizon package containing the bootstrap.
- [ ] Confirm ordinary installation does not invoke native mod compilation on the player device.

## Phase 2 — Runtime decision and prototype
- [x] Establish candidate shortlist (Rhai AST evaluator plus a research-only tree-walking reference); reject Starlark because its evaluator compiles AST to bytecode.
- [x] Add an isolated Rhai AST-evaluator prototype with typed test host calls and an explicit operation budget.
- [x] Add tests for event-style dispatch, batch world query and script-side batch physics replacement.
- [x] Verify runtime formatting, 4 behavior/budget tests and Clippy after the Vec-to-Rhai-Array fix (Actions run 44283982705).
- [x] Add black_box guards after detecting that the first Rust-native benchmark result was optimized away.
- [x] Run the comparable benchmark with equal input cloning and observable output checks (Actions 44284795812: 462.94× slower than native for a 10k-item loop on x86_64 CI).
- [ ] Design/benchmark a coarse provider boundary that minimizes script crossings and retains native hot loops; test on ARM64 before selecting a language.
- [ ] Decide explicitly whether a future optional CI-prebuilt native kernel tier is allowed for hot subsystem replacements; keep it separate from interpreted `.dmod` scripts.
- [ ] Evaluate whether a tiny tree-walking interpreter can provide enough language features for DreamMod without creating an unmaintainable custom language.
- [ ] Benchmark event dispatch, batch world/entity queries and coarse subsystem replacement on desktop and ARM64 devices.
- [ ] Select a runtime only after performance, safety and Android/target compatibility evidence.
- [x] Add prototype operation/call-depth and per-array/per-map/per-string/variable/function limits plus an array-growth regression test (Actions 44285379104 passed; not aggregate heap quotas).
- [ ] Define production execution budgets, exact memory quotas, cancellation and diagnostics.

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
