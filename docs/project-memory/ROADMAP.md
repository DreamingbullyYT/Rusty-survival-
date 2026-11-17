# Roadmap

## Phase 0 — Bootstrap and continuity
- [x] Reset repository tree and establish PMCS.
- [x] Verify bootstrap commit and record SHA.
- [x] Add PMCS structural/freshness validator.
- [x] Add GitHub Actions workflow for pull requests and pushes.
- [x] Add append-only session journal and change template.
- [x] Verify the workflow with real GitHub Actions runs 44121524070, 44121703417, and 44122028665.
- [x] Exercise validator failure cases: missing documents, implementation changes without memory updates, and edited session-log content.
- [x] Verify the positive path for implementation changes accompanied by all required memory updates (run 44126182263).
- [x] Remove the stale hard-coded repository HEAD SHA from CURRENT_STATE; resolve HEAD from Git when needed.

## Phase 1 — Minelib audit
- [x] Record upstream commit/tag and root license.
- [x] Inspect workspace manifests and release workflow.
- [x] Produce preliminary reuse matrix.
- [x] Inspect all `minelib-world` source modules.
- [x] Record world/chunk/block/generation/light/fluid/mesh/raycast incompatibilities.
- [ ] Run upstream world crate tests in a reproducible environment.
- [ ] Complete legal/provenance review of candidate code and assets.
- [ ] Build a minimal Android proof of concept before adopting platform/rendering dependencies.
- [ ] Audit remaining candidate renderer/assets/network components in depth.

## Phase 2 — Specification and architecture
- [x] Select Minecraft Java Edition 26.3 as the observable behavior target.
- [ ] Specify chunk sections, world height, block states, registries, generation, lighting, fluids, collision, and interaction for 26.3.
- [ ] Build PARITY_MATRIX.md from detailed 26.3 observable behavior cases.
- [ ] Finalize module boundaries and interfaces.
- [ ] Define deterministic test harness and reference fixtures.
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
- [ ] No custom Modding API until vanilla foundation is stable unless the user explicitly changes scope.

## Exit criteria before engine implementation
- [x] PMCS automation passes its first CI runs.
- [x] PMCS regression tests cover failure and success paths and pass in GitHub Actions.
- [x] Minecraft Java Edition 26.3 target is selected.
- [ ] Architecture and initial parity specification are reviewed.
- [ ] Reuse decisions and Android platform risks are documented.
