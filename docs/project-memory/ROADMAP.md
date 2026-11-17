# Roadmap

## Phase 0 — Bootstrap
- [x] Reset repository tree and establish PMCS.
- [ ] Verify bootstrap commit and document final SHA.

## Phase 1 — Minelib audit
- [ ] Record upstream commit/tag and license.
- [ ] Audit each candidate crate: dependencies, portability, Android support, coupling, tests, maintenance.
- [ ] Produce REUSE_MATRIX.md with KEEP / ADAPT / REPLACE / REJECT decisions.
- [ ] Build a minimal Android proof of concept before adopting platform/rendering dependencies.

## Phase 2 — Specification and architecture
- [ ] Confirm exact Minecraft Java target version.
- [ ] Build PARITY_MATRIX.md from observable behavior categories.
- [ ] Finalize crate/module boundaries and interfaces.
- [ ] Define deterministic test harness and reference fixtures.

## Phase 3 — Vertical slice
- [ ] Android app lifecycle and surface.
- [ ] Render a minimal world.
- [ ] Input/touch pipeline.
- [ ] Load/save one test world.
- [ ] Reproducible CI build and device test.

## Phase 4 — Vanilla parity implementation
- [ ] Implement systems in dependency order, with tests.
- [ ] Track each parity feature as NOT_STARTED / IMPLEMENTED / VERIFIED / BLOCKED.
- [ ] No mod API until vanilla foundation is stable, unless the user explicitly changes scope.
