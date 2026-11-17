# Project Changelog

## 2026-11-17 — Refine Android renderer feasibility assessment
- Confirmed wgpu 25 documents Vulkan support on Android and winit 0.30 documents an Android glue/lifecycle path.
- Minelib's pinned manifests do not enable that Android entrypoint/configuration; its renderer remains unproven on Android.
- Updated the backend status to candidate-only and planned an isolated clear-screen POC before selecting the renderer.
- No Android build or device test has been performed.


## 2026-11-17 — Strengthen AI onboarding for specs and audits
- AI_HANDOFF now requires reading relevant version-pinned specifications and audit documents.
- PROJECT_MEMORY identifies Rusty Survival Engine and treats reference fixtures as a source of truth.
- Added an explicit rule against guessing unknown Minecraft Java 26.3 behavior.


## 2026-11-17 — Propose modular Android-first engine architecture
- Rewrote ARCHITECTURE.md as a layered design proposal with narrow interfaces and simulation/render/platform separation.
- Documented logical modules, dependency direction, deterministic scheduling principles, first vertical slice, and open decisions.
- Kept renderer backend, ECS, save format, multiplayer scope, and final crate layout undecided pending Android proof and 26.3 reference evidence.
- No engine code or parity implementation added.


## 2026-11-17 — Minelib renderer, assets and networking audit
- Added docs/audits/MINELIB_RENDER_ASSETS_NET_AUDIT.md.
- Renderer: ADAPT GPU techniques only; wgpu is a candidate but Minelib's renderer is not Android-proven and is coupled to winit and Minelib world/assets.
- Assets: ADAPT directory/ZIP loading abstraction; replace platform path handling and 26.3 pack semantics.
- Networking: REPLACE custom Minelib relay protocol for Minecraft Java 26.3 networking.
- Embedded VoxelArt pack: BLOCKED pending independent provenance and redistribution review.
- Source inspection only; no builds, tests, benchmarks, or security audit performed.

## 2026-11-17 — Establish Minecraft Java 26.3 compatibility baseline
- Added docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md with verified release metadata, release-specific gameplay/technical changes, unknown internals, reference-fixture methodology, and Android-first constraints.
- Recorded data version 5023, protocol 777, data pack 121.0, and resource pack 97.1 with source links.
- Explicitly marked world-height/chunk internals, generation algorithms, light/fluid scheduling, collision constants, storage internals, and full protocol schema as unresolved pending reference evidence.
- No engine implementation or parity verification is claimed.

## 2026-11-17 — PMCS regression suite verified
- GitHub Actions run 44126182263 completed successfully.
- Verified negative cases for missing documentation, implementation changes without required memory updates, and edits to existing session-log lines.
- Verified the positive case for implementation changes accompanied by all required memory updates.
- PMCS regression hardening is now VERIFIED; no engine, Android, Vulkan, or Minecraft parity tests are implied.

## 2026-11-17 — PMCS regression hardening
- Added isolated regression tests for missing required documents, implementation changes without required memory updates, append-only session journal violations, and valid implementation-plus-memory updates.
- Expanded code-change freshness requirements to include CHANGELOG, CURRENT_STATE, ISSUES, ROADMAP, and SESSION_LOG.
- Removed a stale commit SHA from CURRENT_STATE; the live repository head must be resolved from Git.

## 2026-11-17 — Pin Minecraft Java Edition 26.3
- Recorded the previously established project target as Minecraft Java Edition 26.3 in the handoff, project memory, compatibility record, parity matrix, decisions, and roadmap.
- Confirmed Horizon remains architectural inspiration only; no Horizon code or implementation is copied.
- No engine implementation or parity verification is claimed.

## 2026-11-17 — PMCS CI verification
- GitHub Actions run 44121524070 completed successfully for commit 0e1074a9bf6b4599820a7cc92373c76f51f59dad.
- Verified checkout history, Python setup, and PMCS structural/freshness validator step.
- PMCS automation is VERIFIED for its initial push workflow execution.

## 2026-11-17 — PMCS automation and continuity hardening
- Corrected project direction: Minecraft Java Edition is the behavior target; Horizon is inspiration only and no Horizon code is to be copied.
- Added append-only session journal and a standard change-record template.
- Added a standard-library Python validator for required memory files, required sections, duplicate session IDs, and memory freshness when implementation files change.
- Added GitHub Actions workflow for push, pull request, and manual validation.

## 2026-11-17 — Minelib world source audit
- Inspected all nine source files in crates/world/src and the world crate manifest.
- Documented custom 16×16×512 chunk model, fixed custom block IDs/registry, custom terrain/noise, custom RGB/torch lighting, fluid geometry without demonstrated Java-compatible fluid simulation, mesh architecture, and DDA raycast.
- Updated REUSE_MATRIX.md with module-level ADAPT/REPLACE decisions.
- Confirmed source contains unit tests, but they were not executed in this audit.
- No upstream source/assets copied; no Android build or Java parity claim made.

## 2026-11-17 — Preliminary Minelib audit
- Pinned upstream reference to 1cd3faeca7a29acf491b794dda1eacf768d61e41.
- Verified root AGPLv3 license text and noted GitHub metadata NOASSERTION.
- Inspected workspace manifests for all nine crates.
- Inspected platform, render, world, app entry point, audio module, and release workflow.
- Added preliminary REUSE_MATRIX.md.
- Identified world as the strongest candidate for deeper audit; no final KEEP decision.
- Confirmed Android support is not demonstrated by README or CI workflow.
- No upstream source or assets copied; no build or parity claim made.

## 2026-11-17 — Repository bootstrap
- Replaced the previous default-branch file tree with the Minelib Hybrid project bootstrap.
- Added initial PMCS documentation.
- Preserved prior Git history; no old files were inspected or reused.
- No game implementation or build is claimed.
