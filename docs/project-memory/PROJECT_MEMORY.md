# Project Memory

## Identity
Working title: Minelib Hybrid
Repository: DreamingbullyYT/Rusty-survival-
Primary platform: Android
Technical strategy: Hybrid — selective reuse after audit, independent implementation where needed.
Primary goal: high-fidelity observable parity with Minecraft Java Edition.

## Scope
Build a modular voxel game runtime with independent systems for platform integration, world simulation, rendering, resources/assets, audio, input, persistence, and networking. Android is the first implementation target. Desktop is not a current delivery target unless explicitly approved.

## Ground rules
- Correctness/parity before feature breadth or benchmark optimization.
- Do not assume Minelib is Minecraft-compatible merely because it supports voxel worlds or Java-world import.
- Reuse is conditional, not mandatory.
- Record provenance, licenses, and modifications for every imported component and asset.
- No undocumented architectural coupling.
- No fabricated test results or completion claims.
- Update project memory with each meaningful implementation change.

## Source of truth
1. Current source and build configuration.
2. Git history and commit references.
3. Reproducible tests and captured logs.
4. These documents as an indexed project state, updated alongside code.
