# Roadmap

## Phase 0 — Bootstrap and continuity
- [x] Reset repository tree and establish PMCS.
- [x] Add PMCS structural/freshness validator, regression tests, session journal and GitHub Actions workflow.
- [x] Verify PMCS regression suite in GitHub Actions.

## Phase 1 — Minelib audit
- [x] Record upstream commit/tag, root license and workspace manifests.
- [x] Inspect every minelib-world source module.
- [x] Inspect renderer, asset loader and networking source entry points; record ADAPT/REPLACE/BLOCKED decisions.
- [ ] Run upstream world crate tests in a reproducible environment.
- [ ] Complete legal/provenance review of candidate code and assets.
- [ ] Build a minimal Android wgpu 25 + winit 0.30 Vulkan clear-screen proof of concept; test lifecycle and swapchain recreation.
- [ ] Audit remaining candidate platform/p2p/relay components if their reuse is still being considered.

## Phase 2 — Specification and architecture
- [x] Select Minecraft Java Edition 26.3 as the observable behavior target.
- [x] Create initial version-pinned 26.3 release-feature and technical-metadata baseline.
- [x] Create a proposed layered architecture and logical module boundaries.
- [ ] Capture 26.3 version metadata and complete registry/tag inventory.
- [ ] Specify dimensions, chunk sections, storage, block states, generation, lighting, fluids, collision and tick order with evidence.
- [ ] Build PARITY_MATRIX.md from detailed 26.3 observable behavior cases.
- [ ] Define deterministic differential test harness and reference fixtures.
- [ ] Review and finalize module boundaries after Android proof and technical specification.

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
- [x] Minelib world, renderer, assets and networking have source-level reuse decisions.
- [x] A proposed layered architecture is documented.
- [ ] Detailed technical parity specification is evidence-backed.
- [ ] Android lifecycle/surface approach is proven.
- [ ] Remaining reuse decisions and legal/provenance risks are closed or explicitly accepted.
