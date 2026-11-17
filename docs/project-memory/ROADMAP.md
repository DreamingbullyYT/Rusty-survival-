# Roadmap

## Phase 0 — Bootstrap and continuity
- [x] Reset repository tree and establish PMCS.
- [x] Verify bootstrap commit and record SHA.
- [x] Add PMCS structural/freshness validator.
- [x] Add GitHub Actions workflow for pull requests and pushes.
- [x] Add append-only session journal and change template.
- [x] Verify PMCS workflow and regression tests in GitHub Actions.
- [x] Exercise validator failure cases and the valid update path.
- [x] Remove the stale hard-coded repository HEAD SHA from CURRENT_STATE.

## Phase 1 — Minelib audit
- [x] Record upstream commit/tag and root license.
- [x] Inspect workspace manifests and release workflow.
- [x] Produce preliminary reuse matrix.
- [x] Inspect all minelib-world source modules.
- [x] Record world/chunk/block/generation/light/fluid/mesh/raycast incompatibilities.
- [x] Inspect renderer, asset loader and networking source entry points; record ADAPT/REPLACE/BLOCKED decisions.
- [ ] Run upstream world crate tests in a reproducible environment.
- [ ] Complete legal/provenance review of candidate code and assets.
- [ ] Build a minimal Android proof of concept before adopting platform/rendering dependencies.
- [ ] Audit remaining candidate platform/p2p/relay components if their reuse is still being considered.

## Phase 2 — Specification and architecture
- [x] Select Minecraft Java Edition 26.3 as the observable behavior target.
- [x] Create an initial version-pinned 26.3 release-feature and technical-metadata baseline.
- [ ] Capture official 26.3 version metadata and build a complete registry/tag inventory.
- [ ] Specify dimensions, chunk sections, storage, block states, generation, lighting, fluids, collision and tick order with evidence.
- [ ] Build PARITY_MATRIX.md from detailed 26.3 observable behavior cases.
- [ ] Finalize module boundaries and interfaces.
- [ ] Define deterministic differential test harness and reference fixtures.
- [ ] Record Horizon-inspired architectural ideas as independent design requirements, not copied code.

## Phase 3 — Vertical slice
- [ ] Android app lifecycle and surface.
- [ ] Render a minimal world through a validated Vulkan path.
- [ ] Touch/input pipeline.
- [ ] Load/save one test world.
- [ ] Reproducible CI build and device test.

## Phase 4 — Vanilla parity implementation
- [ ] Implement systems in dependency order, with tests.
- [ ] Track each parity feature as NOT_STARTED / IMPLEMENTED / VERIFIED / BLOCKED.
- [ ] No custom Modding API until the vanilla foundation is stable unless the user explicitly changes scope.

## Exit criteria before engine implementation
- [x] PMCS automation and regression tests pass in GitHub Actions.
- [x] Minecraft Java Edition 26.3 target is selected.
- [x] Initial version-pinned 26.3 baseline is written.
- [x] Minelib renderer/assets/network entry points have source-level reuse decisions.
- [ ] Detailed technical parity specification is evidence-backed.
- [ ] Architecture and module boundaries are reviewed.
- [ ] Remaining reuse decisions and Android platform risks are documented.
