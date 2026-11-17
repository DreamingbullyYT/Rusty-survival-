# AI Handoff — Mandatory Entry Point

Last updated: 2026-11-17
Project: Minelib Hybrid (working title)
Repository: DreamingbullyYT/Rusty-survival-
Primary target: Android
Priority: Minecraft Java Edition observable parity

## Before doing any work
1. Read this file, then PROJECT_MEMORY.md, CURRENT_STATE.md, ARCHITECTURE.md, DECISIONS.md, ISSUES.md, ROADMAP.md, COMPATIBILITY.md, and PARITY_MATRIX.md.
2. Inspect the current Git branch, latest commit, and working tree.
3. Treat source code, Git history, reproducible test output, and pinned upstream references as ground truth. Never invent progress.
4. Do not import or copy Minelib code until its license and component-level suitability have been reviewed and recorded.
5. Do not claim Android support, parity, performance, or test success without reproducible evidence.
6. After every meaningful change, update the relevant project-memory documents in the same change/commit.

## Non-negotiable project direction
- Android-first.
- Minecraft Java observable behavior is the primary correctness target.
- Modular architecture with explicit interfaces and minimal cross-system coupling.
- Hybrid reuse: reuse only components that pass code-quality, dependency, license, and Android checks.
- Keep code as small as practical without sacrificing required behavior.
- Preserve asset provenance and license records. Never assume Minecraft assets may be redistributed.
- Keep platform-specific code out of simulation/gameplay modules.

## Change record format
Every substantial change must record:
- Change ID
- Date
- Commit SHA
- Files/modules affected
- Motivation and observed problem
- Root cause (if known)
- Implementation
- Tests/commands and exact results
- Remaining risks and follow-up work

## Status vocabulary
OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.
IMPLEMENTED is not VERIFIED.

## Handoff rule
At the end of a work session, update CURRENT_STATE.md, ISSUES.md, ROADMAP.md, and this handoff when relevant. State exactly what was and was not tested.
