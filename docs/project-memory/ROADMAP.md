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
