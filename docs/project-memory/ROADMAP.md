# Roadmap

## Phase 0 — Bootstrap
- [x] Reset repository tree and establish PMCS.
- [x] Verify bootstrap commit and record SHA.

## Phase 1 — Minelib audit
- [x] Record upstream commit/tag and root license.
- [x] Inspect workspace manifests and release workflow.
- [x] Produce preliminary reuse matrix.
- [x] Inspect all `minelib-world` source modules.
- [x] Record world/chunk/block/generation/light/fluid/mesh/raycast incompatibilities.
- [ ] Run upstream world crate tests in a reproducible local/CI environment.
- [ ] Complete legal/provenance review of candidate code and assets.
- [ ] Build a minimal Android proof of concept before adopting platform/rendering dependencies.

## Phase 2 — Specification and architecture
- [ ] Confirm exact Minecraft Java target version.
- [ ] Specify chunk sections, world height, block states, registries, generation, lighting, fluids, collision and interaction.
- [ ] Build PARITY_MATRIX.md from observable behavior categories.
- [ ] Finalize crate/module boundaries and interfaces.
- [ ] Define deterministic test harness and reference fixtures.

## Phase 3 — Vertical slice
- [ ] Android app lifecycle and surface.
- [ ] Render a minimal world through Vulkan.
- [ ] Touch/input pipeline.
- [ ] Load/save one test world.
- [ ] Reproducible CI build and device test.

## Phase 4 — Vanilla parity implementation
- [ ] Implement systems in dependency order, with tests.
- [ ] Track each parity feature as NOT_STARTED / IMPLEMENTED / VERIFIED / BLOCKED.
- [ ] No mod API until vanilla foundation is stable, unless the user explicitly changes scope.
