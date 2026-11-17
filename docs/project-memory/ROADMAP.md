# Roadmap

## Phase 0 — Bootstrap and continuity
- [x] Establish PMCS, validator, regression tests and GitHub Actions.
- [x] Verify PMCS failure and success paths.

## Phase 1 — Minelib audit
- [x] Record upstream commit/tag, root license and workspace manifests.
- [x] Inspect every minelib-world source module.
- [x] Inspect renderer, asset loader and networking entry points; record ADAPT/REPLACE/BLOCKED decisions.
- [ ] Run upstream world crate tests in a reproducible environment.
- [ ] Complete legal/provenance review of candidate code and assets.
- [ ] Audit remaining candidate platform/p2p/relay components if their reuse is still being considered.

## Phase 2 — Specification and architecture
- [x] Select Minecraft Java Edition 26.3 as the observable behavior target.
- [x] Create initial version-pinned 26.3 release-feature and technical-metadata baseline.
- [x] Create proposed layered architecture and logical module boundaries.
- [x] Expand parity matrix with 26.3 release behavior domains and fixture format.
- [x] Document Android Vulkan POC plan and candidate wgpu/winit integration path.
- [ ] Verify protocol/data version by inspecting official 26.3 version.json.
- [ ] Capture 26.3 registry/tag/block-state inventory.
- [ ] Specify dimensions, chunk sections, storage, generation, lighting, fluids, collision and tick order with evidence.
- [ ] Define deterministic differential test harness and reference fixtures.

## Phase 3 — Android platform vertical slice (paused per current priority)
- [x] Implement isolated ARM64 NativeActivity + winit + wgpu Vulkan clear-screen POC source.
- [x] Pass GitHub Actions ARM64 native compilation and Gradle APK packaging.
- [ ] Install and test on real device: startup, first present, pause/resume, rotation and repeated surface recreation.
- [ ] Record device/GPU/driver details and smoke-test results.
- [ ] Decide production renderer/platform stack based on evidence.
- [ ] Integrate lifecycle and surface into the engine only after POC passes.

## Phase 4 — Vanilla parity implementation (not started)
- [ ] Implement systems in dependency order, with tests.
- [ ] Track each parity feature as NOT_STARTED / IMPLEMENTED / VERIFIED / BLOCKED.
- [ ] No custom Modding API until the vanilla foundation is stable unless the user explicitly changes scope.

## Exit criteria before engine implementation
- [x] PMCS automation and regression tests pass in GitHub Actions.
- [x] Minecraft Java Edition 26.3 target is selected.
- [x] Initial 26.3 baseline, Minelib audits and proposed architecture are documented.
- [ ] Detailed technical parity specification is evidence-backed.
- [ ] Android lifecycle/surface approach is proven on hardware.
- [ ] Remaining reuse and legal/provenance risks are closed or explicitly accepted.


## Current execution focus — Engine test foundation
- [x] Add minimal Rust core and deterministic coordinate tests.
- [x] Add CI for formatting, tests and Clippy.
- [x] Verify the new Engine Tests workflow: run 44140955601 passed formatting, tests and Clippy; PMCS run 44140955639 passed.
- [ ] Design and implement a version-pinned reference fixture runner; capture 26.3 fixtures from a lawful reference installation.
- [ ] Continue engine implementation only from specified and testable behavior.


## Engine-only execution plan (current priority)
- [x] Define staged, readable crate ownership and dependency direction in docs/specs/ENGINE_LAYOUT.md.
- [x] Define official-source Java 26.3 reference tests in docs/specs/REFERENCE_TEST_PIPELINE.md: ephemeral $RUNNER_TEMP, size/hash verification, no cache/artifact/release, unconditional cleanup.
- [x] Implement fixture schema/validator with synthetic self-tests (not counted as vanilla parity).
- [x] Implement official metadata/artifact resolution and temporary download verification.
- [x] Add manual Actions artifact smoke workflow with unconditional cleanup and deliberate-failure switch.
- [ ] Verify workflow cleanup on both success and deliberate failure.
- [ ] Capture real 26.3 fixtures, then implement dimension/chunk/section foundations.
- [ ] Expand engine crates gradually; no empty placeholder crates.
- [ ] Keep modding entirely out of scope.
