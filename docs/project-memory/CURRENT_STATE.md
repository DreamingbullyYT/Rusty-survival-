# Current State

Last updated: 2026-11-17
Status: ENGINE_CORE_TEST_FOUNDATION_ADDED; ANDROID_VULKAN_POC_DEVICE_CRASH_UNRESOLVED
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
- The user installed the APK; it immediately crashes. The supplied log does not contain a conclusive native crash signature, so the cause is unknown. Android debugging is paused by the user's latest direction.
- Artifact: workflow run 44127492869, artifact ID 14347124661, 2,379,313 bytes, SHA-256 d4ec89aca87ae27dbd22f7505965be1017b5f522790323363581cb8ed4fafd61. Extracted APK is 7,726,873 bytes, ARM64-only (`lib/arm64-v8a/libmain.so`), SHA-256 27493418f81bd866dbbfe89cff4bcdc5ff9f4ef058142d127adcc10545538552.
- No Minelib or Horizon source/assets copied.
- Added a dependency-free Cargo workspace and rusty-survival-engine-core.
- Implemented horizontal block-to-chunk mapping using Euclidean division/remainder, preserving valid local X/Z for negative coordinates without defining dimension height.
- Added integration tests for positive/negative boundaries, round trips, integer extremes, and the 16-block horizontal chunk width.
- Added Engine Tests GitHub Actions workflow for formatting, tests, and Clippy.
- Added docs/specs/ENGINE_TEST_STRATEGY.md.
- Engine Tests run 44140955601 completed SUCCESS on code commit `f504ba1f3629702330380c3bb71c44c4a5879150`; formatting, `cargo test --workspace --all-targets`, and Clippy all passed.
- PMCS validation run 44140955639 completed SUCCESS on the same code commit; latest documentation updates were also validated by PMCS push run 44142349002 (SUCCESS).
- The tested engine foundation and its memory updates were fast-forwarded directly to `main` at `8bf795c2959a1ca78e4fe40618b418df015162ae`; no PR review/merge flow was used. GitHub subsequently marked the now-redundant PR #4 closed/merged because its head was already on `main`.
- Main-branch push validation runs 44142921426 (Engine Tests) and 44142921444 (PMCS) were triggered after the direct update; final conclusions are being checked.
- Local Rust tests were not run because rustc and cargo are unavailable in the current execution environment; CI executed them successfully.

## Not implemented / not verified
- APK launch on the user's device failed immediately; successful runtime startup and Vulkan presentation remain unverified.
- Android Vulkan adapter/surface/present, pause/resume, rotation, or swapchain recreation have not been tested on hardware.
- Protocol/data-version values still need confirmation from version.json in a legitimate official 26.3 artifact before networking/save implementation.
- Full registry, world/chunk, tick, physics, generation, lighting and fluid specifications are incomplete.
- No world model beyond the initial coordinate primitive; no sections, heightmaps, block states, registries, ticks, generation, physics, lighting, fluids, persistence, renderer, or gameplay.
- No Minecraft Java 26.3 differential fixtures captured; coordinate tests verify foundational invariants only and do not establish parity.
- Minelib upstream tests, legal/provenance review, and security review remain incomplete.
- wgpu/winit is only a candidate path; production renderer selection remains OPEN.

## Next actions
1. Continue all repository work directly on `main`; do not create feature branches or pull requests unless the user explicitly changes this rule.
2. Design a version-pinned 26.3 reference-fixture harness before adding more world/gameplay behavior.
3. Build deterministic Minecraft Java 26.3 reference fixtures before implementing version-specific world/gameplay rules.
4. Verify 26.3 version metadata and extract registries/tags/block states from a legitimate reference artifact.
5. Specify dimensions, chunk sections, storage, generation, lighting, fluids, collision and tick order with evidence.
6. Keep Android POC crash investigation paused until the user asks to resume it.
