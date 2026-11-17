# Project Memory

## Identity
Project: Rusty Survival Engine
Repository: DreamingbullyYT/Rusty-survival-
Primary platform: Android
Correctness target: Minecraft Java Edition 26.3 observable behavior
Architectural inspiration: Horizon, ideas only; no code copying
Technical strategy: Hybrid — selective Minelib reuse after audit, independent implementation where needed.

## Mission
Build an independent, modular, Android-first voxel game engine/runtime that aims for high-fidelity observable parity with Minecraft Java Edition 26.3. Learn from Horizon's architecture and engineering ideas without copying its code or implementation. Audit Minelib components and reuse only those that are technically, legally, and architecturally suitable.

## Scope
Independent systems for platform integration, runtime, vanilla gameplay, world simulation, rendering, resources/assets, audio, input, persistence, and networking. Android is the first implementation target. Desktop is not a current delivery target unless explicitly approved.

## Ground rules
- Correctness/parity before feature breadth or benchmark optimization.
- Do not assume Minelib is Minecraft-compatible merely because it supports voxel worlds or Java-world import.
- Horizon is inspiration only. No copied Horizon source, assets, or implementation.
- Reuse is conditional, not mandatory.
- Record provenance, licenses, and modifications for every imported component and asset.
- No undocumented architectural coupling.
- Keep headless engine logic independent of Android, graphics APIs, audio devices, UI, and filesystem paths.
- Run Rust formatting, unit/integration tests, and Clippy in CI for engine changes.
- No fabricated test results or completion claims.
- Update project memory with each meaningful implementation change.
- Keep historical session records append-only.
- Do not guess version-specific Minecraft Java 26.3 constants or mechanics; use reference fixtures and mark unknowns OPEN.

## Source of truth
1. Current source and build configuration.
2. Git history and commit references.
3. Reproducible tests and captured logs.
4. Version-pinned Minecraft Java 26.3 reference fixtures and their provenance.
5. These documents as an indexed project state, updated alongside code.

## Continuity system
PMCS consists of AI_HANDOFF, PROJECT_MEMORY, CURRENT_STATE, ARCHITECTURE, DECISIONS, ISSUES, ROADMAP, COMPATIBILITY, PARITY_MATRIX, REUSE_MATRIX, CHANGELOG, SESSION_LOG, and CHANGE_TEMPLATE, plus version-pinned specifications in docs/specs/ and audit evidence in docs/audits/. A GitHub Actions workflow validates required documents and checks that implementation changes include memory updates. The validator is a guardrail, not a substitute for truthful human/AI updates.


## Engine test foundation
The initial Cargo workspace and rusty-survival-engine-core crate provide only platform-independent horizontal block-to-chunk coordinate mapping. The Engine Tests workflow runs formatting, tests and Clippy. These checks are foundational invariants only, not a Minecraft Java 26.3 parity claim. See docs/specs/ENGINE_TEST_STRATEGY.md.
