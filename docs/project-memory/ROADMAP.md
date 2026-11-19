# Roadmap

## Phase 0 — Project continuity and research
- [x] Maintain PMCS continuity files and validation workflow.
- [ ] Keep PMCS regression tests green and append session history.

## Phase 1 — Package safety and lifecycle
- [x] Strict .dmod archive/manifest validation.
- [x] Explicit capabilities, approvals, lifecycle and deterministic dependencies.
- [x] Asset and entrypoint access gates.
- [x] Resource ID ownership bookkeeping.
- [x] Atlas declaration batches owned by ModManager, enabled-owner snapshot and rollback validation (CI pending).

## Phase 2 — Runtime foundation
- [ ] Decide and implement interpreted-source runtime (Rhai remains prototype, not selected).
- [ ] Evaluate a WASM runtime only if it can use precompiled modules without player-side compilation; no DreamMod WASM support exists currently.
- [ ] Player native .so/DLL runtime is not implemented; Core native build integration is separate.
- [ ] Implement sandbox, execution budgets, lifecycle callbacks and fault isolation.
- [ ] Select scripting language only after comparative evaluation and representative benchmarks.
- [ ] Implement sandbox, execution budgets, lifecycle callbacks and fault isolation.
- [ ] Create direct, batch-oriented host APIs and extension contracts.

## Phase 3 — Deep engine extension
- [ ] Add extension points for gameplay/world/entity/render/audio/UI and replacement of supported subsystems.
- [ ] Integrate Horizon hooks and test reload/disable cleanup.
- [ ] Preserve compatibility through versioned contracts.

## Phase 4 — Integrated mod platform
- [ ] End-to-end .dmod loading, interpreted execution, host access and in-game validation.
- [ ] Test large mod sets, failure isolation, resource reloads and ARM64 performance.

## Atlas declarations groundwork
- [x] Export validation/snapshot helper with enabled-owner filtering, deterministic ordering, duplicate rejection and 32-entry cap.
- [x] Integrate batch replacement and snapshot into ModManager; release owner declarations on disable/failure.
- [ ] Verify package CI.
- [ ] Core hook consumes the direct snapshot and resubmits declarations on each resource reload (signature corrected; rustfmt cleanup pending).

## Exit criteria before engine implementation
- Confirm package/runtime safety architecture, extension contracts, host integration route, and representative compatibility tests before expanding engine implementation.

## Shared Horizon mods folder
- [x] Accept `horizon-mods/` as the common folder for player packages.
- [ ] Add a bounded deterministic `.dmod` scanner; ignore `.hmod` and unrelated files.
- [ ] Test mixed extensions, malformed packages, duplicate IDs, unsafe entries, and diagnostics; verify scanner never extracts archives or crosses folder boundaries.

## Shared Horizon mods folder scanner
- [x] Implement direct-child `.dmod` discovery in runtime library; ignore `.hmod` and unrelated files.
- [x] Reject duplicate package IDs deterministically; report malformed package, symlink and non-regular candidate diagnostics.
- [x] Add bounded package-count and aggregate archive-size limits with regression tests.
- [ ] Wire scanner to actual launcher/game instance `horizon-mods/` path and surface diagnostics in mod manager UI.
- [ ] Verify runtime CI.

- [x] Runtime CI passed after final rustfmt correction: 20 tests, formatting and Clippy.

## Runtime shared-folder integration
- [x] Connect scanner output to `DreamModRuntime::load_packages` via a public shared-directory entrypoint and preserve diagnostics.
- [ ] Verify integration test and runtime CI.
- [ ] Launcher must supply the actual instance `horizon-mods/` path and surface diagnostics in UI.

- [ ] Verify runtime scanner-to-loader entrypoint after rustfmt correction (tests and Clippy).

- [ ] Confirm scanner-to-runtime entrypoint's latest CI after final whitespace correction.

- [x] Scanner-to-runtime entrypoint verified: runtime CI `44544390519` passed 21 tests, formatting and Clippy.
- [ ] Launcher supplies actual instance `horizon-mods/` path and presents diagnostics in UI.

- [ ] Verify valid-package end-to-end directory test with runtime CI.

- [x] Valid-package end-to-end test verified: runtime CI `44544852796` passed 22 tests, formatting and Clippy.


## DreamMod independent engine target (accepted 2026-11-19)
- [x] Supersede Horizon/HNMods as mandatory target; independent API/host for Rusty-survival.
- [x] Write comprehensive domain/contract architecture in `docs/specs/DREAMMOD_ENGINE_INDEPENDENT_API.md`.
- [ ] Build contract foundation: IDs, API versioning, capability model, handle/error/diagnostic types.
- [ ] Create fake host and engine adapter boundary without dependency on Horizon.
- [ ] Choose source interpreter only after no-bytecode/no-JIT requirement, ARM64, throughput, safety and API binding benchmarks.
- [ ] Implement production script lifecycle, budget metering, event dispatch and fault isolation.
- [ ] End-to-end vertical slice: `.dmod` → interpreted callback → host API → engine, plus disable/reload cleanup.
- [ ] Expand APIs domain-by-domain only as engine support lands.
- [ ] Physics collision channels and vehicle wheel-only collision proof.
- [ ] Engine API compatibility/feature negotiation and automated package compatibility tests.

Horizon native Core/.hmod and the shared Horizon mods-folder scanner are historical/optional experiments, not blockers or dependencies for the new engine.


## Phase 2 — runtime activation and integration status

- [x] Runtime package loading is default-disabled and requires explicit `enable_mod` activation.
- [ ] Verify the new activation regression test in CI (latest checks failed formatting before tests ran).
- [ ] Connect explicit activation to the single package approval/lifecycle authority.
- [ ] Implement mock engine command executor and provider resolver; live native adapters remain unimplemented.


## Verification status update — 2026-11-19
- [x] Root workspace format, tests and Clippy passed at commit `1fb4ed6b6272804027aecb14f40f3ae8a3621b45`.
- [ ] Runtime-specific workflow for `mods/dreammod-runtime` is not yet confirmed.
- [ ] PMCS validator freshness check for the latest source/test commit.


## Runtime fault cleanup
- [x] Clear provider registrations when a runtime mod is auto-disabled due to repeated script failures, both from event and provider callback paths.
- [ ] Verify fault cleanup regression test in DreamMod Runtime CI.


- Fault cleanup behavior is implemented, but runtime CI first stopped at rustfmt; corrected formatting and rerunning.


## Verified runtime lifecycle and cleanup
- [x] Default-disabled runtime activation and regression test: verified in DreamMod Runtime CI (24 tests, formatting and Clippy passed).
- [x] Provider registrations are removed on fault disable; reactivation does not resurrect provider; regression test passed.
- [x] Root Rust core tests, formatting, Clippy and offline engine workflow passed at `6bae66f43f6e51b9c92301beea34224e9bb03ac6`.
- [ ] Unify explicit activation with package ModManager approval lifecycle.
- [ ] Build mock engine host command executor, provider resolver and end-to-end tests.


## Engine host boundary milestone
- [x] Add runtime API that forwards only authorized commands to an engine host in one atomic batch.
- [x] Mock-host tests for authorized delivery and denied-command suppression.
- [x] Verify host-boundary commits in CI: runtime formatting, 26 tests and Clippy passed on 406b8e5; root offline engine test passed on 4d47315.
- [ ] Replace mock with typed engine adapter and actual state/rollback tests.


## Mock block-world vertical slice
- [x] Add a test-only stateful block host with validation-before-commit.
- [x] Exercise successful writes and prove an invalid batch leaves state unchanged.
- [x] Verify CI for the new tests: rustfmt, 27 tests, Clippy passed on c89b038.
- [ ] Add entity adapter tests and cross-domain transaction coverage.

## Cross-domain atomic mock engine
- [x] Extend mock host to stage block and entity-health commands together.
- [x] Add validation for entity handles and finite health values.
- [x] Add test asserting invalid command rolls back staged world and entity mutations.
- [x] Verify CI: fd4109a passed DreamMod Runtime (27 tests, rustfmt, Clippy) and workspace offline tests/Clippy.

## World/block adapter vertical slice
- [x] Add typed block ID, position, mutation structures and host adapter for world.set_block.
- [x] Validate payload schema, IDs, horizontal world border, and reject unsupported command domains.
- [x] Stage all mutations before invoking backend's atomic batch method.
- [ ] Verify runtime/workspace CI and formatting.
- [ ] Connect adapter to engine-owned world/chunk storage (storage not implemented yet).

- [ ] Verify formatted WorldBlockCommandAdapter runtime workflow.

- [ ] Re-run DreamMod Runtime CI after adapter test import correction (106a1c5).


## Hybrid API routing (accepted 2026-11-19)
- [x] Define provider abstraction and per-operation capability declarations.
- [x] Default to Horizon-first preference with DreamMod fallback when the Horizon provider does not advertise an operation.
- [x] Runtime domain preference, provider hot replacement/removal and tests added (CI pending).
- [x] Connect router to host-side runtime API calls and authorization pipeline (enabled-mod check, recognized operation, capability grant).
- [x] Expose event-scoped API requests to .dmod scripts with capability enforcement, routing and next-event result handling (CI pending).
- [ ] Implement and pin concrete Horizon adapter(s); verify actual operations against current public API.
- [ ] Provide DreamMod-native implementations for gaps; add operation-specific integration tests.
- [ ] Define safe migration/snapshot contracts before hot-switching stateful providers.


## Script API bridge design — 2026-11-19
- [x] Define event-scoped request/response design with request IDs, per-mod result isolation, authorization, bounds and cleanup requirements (`docs/specs/DREAMMOD_SCRIPT_API_BRIDGE.md`).
- [x] Implement typed request/result envelopes and strict validation.
- [x] Add per-mod one-shot result queues and event payload injection.
- [x] Route script requests through the same capability checks and provider router as host calls.
- [x] Add end-to-end script → provider → next-event result → command regression test.
- [x] Add capability-denial result test; denial does not disable the mod.
- [x] Add two-mod result-isolation test using same request ID but different input values.
- [x] Preserve scalar event payloads before results are present; test scalar wrapping when results arrive.
- [x] Bound aggregate queued API-result bytes and clear results on disable/fault-disable.
- [ ] Expand result-isolation and provider-error tests; verify runtime format, tests and Clippy in CI.


## Script API bridge contract primitives
- [x] Add serializable request/result envelope types and request-ID/operation validation.
- [x] Add tests for validation and envelope serialization.
- [ ] Verify commit `f484db8` with runtime CI (status/run list currently empty).
- [ ] Implement request processing, authorizer capability enforcement, provider routing, response injection, per-mod queue isolation and cleanup.


## Script API bridge progress
- Source and basic end-to-end test implemented in commits `9934644` and `6ee5b26`; PMCS updated.
- GitHub status/workflow lookup did not expose checks. Treat CI as unverified until Actions results are available.

- Script bridge hardening source/test commits: `db1189d`, `4fc88f8`, `09222b5`, `39d0166`, `26d955b`; Actions still unverified.

- Isolation test commit `8c2f410`; Actions remains unverified.

- Scalar event compatibility commits `037eaed`/`0e40e51`; Actions pending.
