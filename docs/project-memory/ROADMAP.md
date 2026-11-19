# Roadmap

## Phase 0 — Project continuity and research
- [x] Maintain PMCS continuity files and validation workflow.
- [ ] Keep PMCS regression tests green and append session history.

## Phase 1 — Package safety and lifecycle
- [x] Strict .dmod archive/manifest validation.
- [x] Explicit capabilities, approvals, lifecycle and deterministic dependencies.
- [x] Asset and entrypoint access gates.
- [x] Resource ID ownership bookkeeping.

## Phase 2 — Runtime foundation
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
- [x] Add exported validation/snapshot helper for mod-owned atlas declarations, enabled-owner filtering, deterministic ordering, duplicate rejection and 32-entry cap.
- [x] Package CI run `44467917911` passed.
- [ ] Integrate declarations into ModManager and the direct batched Core-facing API; submit on every verified Horizon resource reload hook.

## Exit criteria before engine implementation
- Confirm package/runtime safety architecture, extension contracts, host integration route, and representative compatibility tests before expanding engine implementation.
