# Roadmap

## Phase 0 — Bootstrap and continuity
- [x] Reset repository tree and establish PMCS.
- [x] Verify bootstrap commit and record SHA.
- [x] Add PMCS structural/freshness validator.
- [x] Add GitHub Actions workflow for pull requests and pushes.
- [x] Add append-only session journal and change template.
- [x] Verify the workflow with a real GitHub Actions run (run 44121524070, success).
- [ ] Exercise validator failure cases (missing docs, stale implementation change, edited journal line).

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
- [ ] Confirm exact Minecraft Java Edition target version.
- [ ] Specify chunk sections, world height, block states, registries, generation, lighting, fluids, collision, and interaction.
- [ ] Build PARITY_MATRIX.md from observable behavior categories.
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
- [x] PMCS automation passes its first CI run.
- [ ] Exact Minecraft Java Edition version is selected.
- [ ] Architecture and initial parity specification are reviewed.
- [ ] Reuse decisions and Android platform risks are documented.
