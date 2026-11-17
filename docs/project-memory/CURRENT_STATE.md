# Current State

Last updated: 2026-11-17
Status: ANDROID_RENDERER_POC_PLANNED
Repository HEAD: resolve dynamically from the current main branch; do not store a commit SHA here because it becomes stale after every commit.

## Direction
- Correctness target: Minecraft Java Edition 26.3 observable behavior.
- Horizon: architectural and technical inspiration only; no code copying.
- Minelib: hybrid reuse only after component-level audit.
- Platform: Android first.

## Repository
- Repository: DreamingbullyYT/Rusty-survival-
- Default branch: main
- PMCS validator and regression suite are verified; run 44126182263 passed.
- Initial 26.3 compatibility baseline and Minelib renderer/assets/network audit are recorded.

## Implemented
- PMCS continuity docs, validator, regression suite and GitHub Actions workflow.
- Initial Minecraft Java Edition 26.3 release-feature/metadata baseline.
- Source-level Minelib world audit.
- Source-level Minelib renderer/assets/network audit with component reuse decisions.
- Proposed layered engine architecture in ARCHITECTURE.md.
- No Horizon or Minelib code/assets copied.

## Not implemented / not verified
- No Rust workspace or game code.
- No Android application or APK.
- No renderer integration or Vulkan device/surface test.
- No Minecraft Java 26.3 parity fixtures.
- No Android lifecycle proof; wgpu 25 + winit 0.30 is a candidate combination, not yet built or tested.
- Minelib upstream tests, legal/provenance review, and security review remain incomplete.
- Architecture is a proposal; final crate boundaries and renderer backend are not selected.
- 26.3 internal world/simulation details remain unresolved pending reference capture.

## Next actions
1. Build and test the isolated wgpu 25 + winit 0.30 Android Vulkan clear-screen proof of concept.
2. Capture 26.3 metadata, registry/tag inventory, dimension/chunk semantics and reference fixtures.
3. Complete remaining Minelib legal/provenance and upstream test work.
4. Review the architecture and parity matrix against evidence.
5. Start engine implementation only after architecture, Android risks and initial parity spec are reviewed.
