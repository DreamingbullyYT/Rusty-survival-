# Roadmap

## Phase 0 — Project continuity
- [x] Preserve PMCS, validator, regression tests and GitHub Actions workflow.
- [x] Verify legacy-aware session ID validation and its duplicate/reuse regression tests (Actions run 44299837834 passed; final PMCS run 44300145443 passed).
- [x] Record the 2026-11-18 DreamMod project pivot in persistent memory.
- [x] Define the initial DreamMod architecture and prebuilt-distribution model.

## Phase 1 — Horizon integration research
- [x] Audit current Horizon mod API/host, package formats, native route and WASM runtime.
- [x] Pin initial Horizon and horizon-mixins revisions in the Core manifest.
- [x] Add minimal native .hmod Core source and pinned-source compile workflow.
- [x] Source-verify Horizon's API-only prebuilt native library route (the CI artifact and clean startup proof remain open).
- [x] Pass official cargo horizon-mod check and package a native source .hmod in CI (run 44285877944; artifact inspected).
- [ ] Produce and inspect a CI-built API-only client engine/native-library overlay using signed SDK slices and Horizon's real host loader; current run 44289897914 pending; prior attempts exposed and fixed Rustup HOME isolation and separate launcher workspace selection; no Mojang assets are included.
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
- [x] Add prototype validation for physics provider batch input/output shape and finite numeric values (commit 33639ee0; Actions verification pending).

## Phase 3 — Core runtime
- [x] Define version-1 `.dmod` manifest and capability declaration schema in `mods/dreammod-package`.
- [x] Implement side-effect-free manifest parsing/validation for schema version, strict fields, IDs, semantic versions/API ranges, safe entrypoint paths, allowlisted/unique capabilities, dependency IDs/ranges and manifest size/count limits.
- [x] Add a side-effect-free archive inventory policy that rejects traversal, non-portable paths, case-folded duplicates, symlinks/other types, encrypted entries, unapproved roots, missing manifest/entrypoint and resource-exhaustion metadata before decompression.
- [x] Integrate an in-memory ZIP reader with strict local-header/central-directory consistency checks, bounded metadata preflight and CRC-checked streaming; no filesystem extraction (CI run 44299317538 passed: 19 tests + Clippy).
- [x] Implement deterministic dependency validation and load-order planning (runtime lifecycle remains open).
- [x] Verify the standalone package contract workflow: formatting, 19 unit tests and Clippy passed on commit 4e983c0 (Actions run 44299317538).
- [x] Verify bounded in-memory retention of validated script/asset bytes; Actions run 44300015439 passed formatting, 19 tests and Clippy.
- [x] Add pure default-deny capability grant planning with explicit approval input (Actions run 44300324687 passed 21 tests and Clippy).
- [x] Add a private-state CapabilityGate guard helper; Actions run 44300879584 passed 22 tests and Clippy. Wiring it into actual Horizon host API calls remains open.
- [x] Add a default-disabled lifecycle policy: validated → awaiting approval → ready → explicitly enabled; disable blocks calls and failure discards grants. Four lifecycle regression tests added.
- [x] Add a generic guarded host adapter that checks activation and capability before invoking a host callback; add bounded failure diagnostics. Concrete Horizon API wiring remains open.
- [x] Extend guard regression coverage so disabled and failed mods cannot invoke host callbacks; package CI run 44304258172 passed formatting, tests and Clippy.
- [x] Define an owned, validated player snapshot DTO and a typed `entities.read` adapter gate; concrete Horizon host implementation and runtime exposure remain open.
- [x] Add a bounded player snapshot batch (256 max) with duplicate-UUID rejection and one capability check per batch; concrete Horizon collector and script binding remain open; package Actions run 44309674771 passed formatting, tests and Clippy.
- [x] Inspect the exact pinned Horizon Hooks/Host/phase surface and record the bridge contract in docs/dreammod/HORIZON_HOST_BRIDGE.md.
- [x] Implement the first Core-side bounded online-player snapshot collector against the pinned ServerHost API; Core workflow run 44310547586 passed SDK build, host/WASM validation and packaging.
- [x] Require a per-mod player-read authorization interface before Core touches ServerHost; regression tests prove denial happens before the host callback. Binding the interface to actual ModLifecycle instances remains open.
- [x] Add Core-local snapshot validation/batch-limit unit tests.
- [x] Add explicit Core CI formatting and cargo test steps after pinned SDK setup; first execution result pending.
- [ ] Verify the corrected gated collector source commit eadd1aa in Core Actions; earlier run targeted an intermediate source-edit regression.
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


## Next — performance and bounded loading (2026-11-18)
1. Verify package CI for the new 60-second deadline and contiguous UUID validation.
2. Verify Core CI after installing rustfmt; confirm cargo test actually executes.
3. Add realistic benchmark fixtures for snapshot batch validation and host-call batching; later measure ARM64 before making FPS claims.
4. When the runtime loader exists, create one deadline at load start and pass it through dependency resolution, asset/script parsing and registration. Poll cooperatively and fail the mod closed on expiry; do not block the game thread.
5. Only then add immutable per-dispatch snapshot reuse with explicit invalidation/thread rules.
