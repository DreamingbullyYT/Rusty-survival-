# Current State

Last updated: 2026-11-17
Status: SPECIFICATION_IN_PROGRESS
Repository HEAD: resolve dynamically from the current `main` branch; do not store a commit SHA here because it becomes stale after every commit.

## Direction
- Correctness target: Minecraft Java Edition 26.3 observable behavior.
- Horizon: architectural and technical inspiration only; no code copying.
- Minelib: hybrid reuse only after component-level audit.
- Platform: Android first.

## Repository
- Repository: DreamingbullyYT/Rusty-survival-
- Default branch: main
- PMCS validator, regression tests, and GitHub Actions workflow are implemented and verified.
- PMCS regression suite run 44126182263: SUCCESS.

## Implemented
- Clean project bootstrap and project-memory documentation.
- Preliminary Minelib crate/license/dependency audit.
- Source-level inspection of every module in `crates/world/src`.
- Expanded reuse matrix with per-module findings.
- PMCS mandatory handoff rules, append-only session journal, and change template.
- Python structural/freshness validator and automated regression suite.
- GitHub Actions workflow for push, pull request, and manual PMCS validation.
- Minecraft Java Edition 26.3 recorded as the selected parity target.
- Horizon recorded as inspiration only, with independent implementation and no code copying.
- Initial version-pinned 26.3 compatibility baseline specification in `docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md`.

## Not implemented / not verified
- No Rust workspace or game code.
- No Android application or APK.
- No renderer integration.
- No Minelib or Horizon source copied.
- No Minecraft asset bundle imported.
- No Minecraft Java 26.3 parity tests or reference fixtures.
- No successful Android build or Vulkan device/surface test.
- Minelib's upstream tests have not been executed.
- The 26.3 baseline is an initial release-feature inventory, not a complete technical spec for world internals, simulation, or networking.

## World audit findings
- Minelib chunks use a custom 16×16×512 model with Y starting at zero; not directly compatible with modern Java world-height/section semantics.
- Block IDs and registry represent Minelib-specific content/states.
- Terrain generation and noise are custom and not Java-compatible by evidence.
- World light logic is custom, including radius-limited RGB torch lights and chemical torches.
- Water/lava shapes and meshing exist, but Java-compatible fluid tick/spread behavior is not demonstrated.
- Meshing and DDA raycast contain potentially reusable general algorithms, but need semantic extraction and tests.

## Next actions
1. Capture 26.3 version metadata and build a data/registry inventory.
2. Specify world dimensions, chunk sections, block states, storage and generation with reference evidence.
3. Continue Minelib renderer/assets/network audits and Android Vulkan/lifecycle proof.
4. Finalize module boundaries and differential test harness.
5. Begin engine implementation only after specification, architecture and platform risks are reviewed.
