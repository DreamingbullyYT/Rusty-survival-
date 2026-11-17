# Current State

Last updated: 2026-11-17
Status: ANDROID_VULKAN_POC_IMPLEMENTED_AWAITING_CI
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
- 26.3 baseline, Minelib audits, and proposed architecture recorded.

## Implemented
- PMCS continuity docs, validator, regression suite and GitHub Actions workflow.
- Initial Minecraft Java Edition 26.3 compatibility baseline.
- Source-level Minelib world, renderer, assets and network audits.
- Proposed layered architecture and Android Vulkan POC plan.
- Isolated Android NativeActivity + winit 0.30.13 + wgpu 25.0.0 Vulkan clear-screen POC source and Gradle packaging.
- Dedicated GitHub Actions workflow to compile and package an ARM64 debug APK.
- No Minelib or Horizon source/assets copied.

## Not implemented / not verified
- Initial POC workflow failed before Rust compilation because setup-android v3 attempted to install the removed SDK 'tools' package. Workflow is being corrected to setup-android v4; no Rust compile result or APK is claimed yet.
- No APK has yet been verified as installable on a real device.
- No Android Vulkan adapter/surface/present, pause/resume, rotation, or swapchain recreation test has been performed on hardware.
- No Rust engine workspace or gameplay code.
- No Minecraft Java 26.3 parity fixtures.
- Minelib upstream tests, legal/provenance review, and security review remain incomplete.
- wgpu/winit is only a candidate path; production renderer selection remains OPEN.

## Next actions
1. Fix any Android POC CI build errors and produce a debug APK artifact.
2. Install/test the APK on a real ARM64 Android device; capture logcat and lifecycle behavior.
3. Continue 26.3 reference metadata, registry/tag and world/chunk specification.
4. Finalize architecture and differential test harness.
5. Begin engine implementation only after the specification, architecture and platform risks are reviewed.
