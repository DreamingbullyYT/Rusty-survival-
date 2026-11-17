# Current State

Last updated: 2026-11-17
Status: ANDROID_VULKAN_POC_CI_PASSED_DEVICE_TEST_PENDING
Repository HEAD: resolve dynamically from the current main branch; do not store a commit SHA here because it becomes stale after every commit.

## Direction
- Correctness target: Minecraft Java Edition 26.3 observable behavior.
- Horizon: architectural and technical inspiration only; no code copying.
- Minelib: hybrid reuse only after component-level audit.
- Platform: Android first.

## Repository
- Repository: DreamingbullyYT/Rusty-survival-
- Default branch: main
- PMCS validator/regression suite verified.
- Initial 26.3 reference baseline, Minelib audits, proposed architecture and parity matrix recorded.

## Implemented
- PMCS continuity docs, validator, regression tests and GitHub Actions workflow.
- Initial Minecraft Java Edition 26.3 release-feature and technical-metadata baseline with evidence links and explicit unknowns.
- Expanded parity matrix into testable domains and fixture requirements.
- Source-level Minelib world, renderer, assets and network audits.
- Proposed layered architecture and Android Vulkan POC plan.
- Isolated Android NativeActivity + winit 0.30.13 + wgpu 25.0.0 Vulkan clear-screen POC source and Gradle packaging.
- GitHub Actions successfully compiled the ARM64 native library and packaged a debug APK.
- Artifact: workflow run 44127492869, artifact ID 14347124661, 2,379,313 bytes, SHA-256 d4ec89aca87ae27dbd22f7505965be1017b5f522790323363581cb8ed4fafd61. Extracted APK is 7,726,873 bytes, ARM64-only (`lib/arm64-v8a/libmain.so`), SHA-256 27493418f81bd866dbbfe89cff4bcdc5ff9f4ef058142d127adcc10545538552.
- No Minelib or Horizon source/assets copied.

## Not implemented / not verified
- APK has not yet been installed or executed on a real device.
- Android Vulkan adapter/surface/present, pause/resume, rotation, or swapchain recreation have not been tested on hardware.
- Protocol/data-version values still need confirmation from version.json in a legitimate official 26.3 artifact before networking/save implementation.
- Full registry, world/chunk, tick, physics, generation, lighting and fluid specifications are incomplete.
- No Rust engine workspace or gameplay code.
- No Minecraft Java 26.3 parity fixtures.
- Minelib upstream tests, legal/provenance review, and security review remain incomplete.
- wgpu/winit is only a candidate path; production renderer selection remains OPEN.

## Next actions
1. Install the generated APK on a real ARM64 Android device; capture logcat.
2. Verify Vulkan adapter, first frame presentation, pause/resume, rotation, and repeated surface recreation.
3. Verify 26.3 version metadata and extract registries/tags/block states from a legitimate reference artifact.
4. Specify chunk/dimension storage, tick ordering, collision, generation, lighting and fluids.
5. Build deterministic differential fixtures and test harness.
6. Begin engine implementation only after the specification, architecture and platform risks are reviewed.
