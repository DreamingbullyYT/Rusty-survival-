# Current State

Last updated: 2026-11-17
Status: ENGINE_CORE_FIXTURE_CONTRACT_AND_ARTIFACT_ACQUISITION_IMPLEMENTED; WORKFLOW_VERIFICATION_PENDING; JAVA_ORACLE_NOT_IMPLEMENTED; ANDROID_VULKAN_POC_DEVICE_CRASH_UNRESOLVED
Repository HEAD: resolve dynamically from current main; do not store a stale SHA here.

## Direction
- Correctness target: Minecraft Java Edition 26.3 observable behavior.
- Build an independent Rust vanilla engine.
- Horizon: architecture and engineering inspiration only; no code copying.
- Minelib: selectively reuse only after component, license and provenance review.
- Android is the final first-class platform target; current work is engine foundations.
- Modding systems are explicitly out of scope.

## Repository
- Repository: DreamingbullyYT/Rusty-survival-
- Default branch: main; all work directly on main, no PRs/feature branches.
- PMCS validator/regression suite and initial engine Rust CI were previously verified.
- Initial 26.3 baseline, Minelib audits, architecture and parity matrix are recorded.

## Implemented
- PMCS continuity docs, validator, regression tests and GitHub Actions workflow.
- Initial Minecraft Java Edition 26.3 release-feature and technical-metadata baseline.
- Source-level Minelib world, renderer, assets and network audits; no source copied.
- Android Vulkan POC source and packaging workflow; APK built in CI but user reported immediate device crash. Android debugging is paused.
- Dependency-free Cargo workspace and `rusty-survival-engine-core`.
- `BlockPos`, `ChunkPos`, `ChunkAddress`, horizontal block-to-chunk mapping using Euclidean division/remainder.
- Five coordinate integration tests; these are foundational invariants, not Java parity.
- Engine Tests workflow for formatting, tests and Clippy.
- Added `docs/specs/ENGINE_TEST_STRATEGY.md`.
- Added `docs/specs/ENGINE_LAYOUT.md`: staged, readable crate ownership map and dependency direction. Only engine-core exists; no placeholder crates.
- Added `docs/specs/REFERENCE_TEST_PIPELINE.md`: official-source reference acquisition policy using ephemeral `$RUNNER_TEMP`, hash/size checks, no cache/artifact/release and unconditional cleanup.
- Docs commits: ENGINE_LAYOUT `dcbc7d0c858918c6a52769a54f0e6a9a7d2b8934`; REFERENCE_TEST_PIPELINE `6e6ef01402b8bdc4cf1d17a2ffe9f23ff225acf7`.
- Added `rusty-survival-testkit` as a separate workspace crate with serde-based fixture contract, validation, and five synthetic self-tests. The only checked-in fixture is explicitly labelled synthetic and is not parity evidence.
- Added `tools/fetch_mojang_server.py`: resolves the pinned Java 26.3 release from Mojang's official v2 manifest, verifies manifest metadata SHA-1, rejects unapproved hosts/redirects, and streams the server jar while checking exact size and SHA-1.
- Added offline Python unit tests for URL policy and artifact verification.
- Added `.github/workflows/reference-tool-tests.yml` for offline tests and `.github/workflows/reference-artifact-smoke.yml` for manual temporary artifact acquisition with unconditional cleanup and an optional deliberate-failure path.

## Not implemented / not verified
- Fixture schema/parser/validator and synthetic self-tests are implemented. Downloader and manual artifact-only workflow are implemented but their Actions runs are not yet verified. No Java 26.3 differential fixture, Java oracle, Rust probe comparator or parity runner exists.
- No world sections, heightmaps, block states, registries, ticks, generation, physics, lighting, fluids, persistence, renderer, audio, runtime or playable game.
- No Mojang binary or asset has been downloaded or committed as part of the latest documentation work.
- Minelib upstream tests and complete legal/provenance review remain incomplete.
- Android POC device crash, Vulkan adapter/surface/present and lifecycle remain unresolved and paused.
- wgpu/winit is only a candidate; production renderer is not selected.

## Next actions
1. Continue directly on main; no PRs or feature branches.
2. Implement official metadata/artifact resolution with size/hash checks into `$RUNNER_TEMP`.
3. Add a manual Actions reference workflow; verify cleanup on success and failure; never cache or upload Mojang files.
4. Capture real Java 26.3 fixtures, then implement world/dimension/chunk foundations in dependency order.
6. Keep all modding work out of scope.
7. Keep Android POC crash investigation paused unless explicitly requested.
