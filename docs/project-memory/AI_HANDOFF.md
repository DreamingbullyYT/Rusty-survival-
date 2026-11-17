# AI Handoff — Mandatory Entry Point

Last updated: 2026-11-17
Project: Minelib Hybrid (working title)
Repository: DreamingbullyYT/Rusty-survival-
Primary platform: Android
Correctness target: Minecraft Java Edition 26.3
Architectural inspiration: Horizon (ideas only; no code copying)

## Mandatory onboarding
1. Read this file, then PROJECT_MEMORY.md, CURRENT_STATE.md, ARCHITECTURE.md, DECISIONS.md, ISSUES.md, ROADMAP.md, COMPATIBILITY.md, PARITY_MATRIX.md, REUSE_MATRIX.md, and SESSION_LOG.md.
2. Inspect the current branch, HEAD commit, working tree, and recent history. Repository state is authoritative over stale prose.
3. Read the latest session-log entry and unresolved issues before proposing work.
4. Treat source code, Git history, reproducible test output, and pinned upstream references as ground truth. Never invent progress.
5. Do not import/copy Horizon code. Horizon is architectural/technical inspiration only; independently design and implement our systems.
6. Do not import Minelib code until its license and component-level suitability have been reviewed and recorded.
7. Do not claim Android support, parity, performance, or test success without reproducible evidence.
8. Before implementation, state the intended change, affected modules, risks, and verification plan.
9. After every meaningful change, update the memory documents in the same commit. Append a session-log entry; do not rewrite old entries.
10. Run `python scripts/validate_project_memory.py --base <base-sha>` when a base commit is available, and run it without --base for a structural check.
11. Run `python -m unittest discover -s tests -v` to exercise PMCS validator regression cases.

## Non-negotiable project direction
- Android-first.
- Minecraft Java Edition 26.3 observable behavior is the primary correctness criterion.
- Horizon is inspiration for architectural ideas and engineering approach only; no source code, assets, or copied implementation.
- Modular architecture with explicit interfaces and minimal cross-system coupling.
- Hybrid Minelib reuse only after code-quality, dependency, license, and Android checks.
- Keep code as small as practical without sacrificing required behavior.
- Preserve asset provenance and license records. Never assume Minecraft assets may be redistributed.
- Keep platform-specific code out of simulation/gameplay modules.
- No custom Modding API until the vanilla foundation is stable unless the user explicitly changes scope.

## Change record requirements
Every substantial change must record:
- Change ID and date
- Commit SHA (fill after commit, or link the entry to the commit)
- Files/modules affected
- Motivation and observed problem
- Root cause (if known)
- Implementation
- Tests/commands and exact results
- Remaining risks and follow-up work

Use `docs/project-memory/CHANGE_TEMPLATE.md`.

## Status vocabulary
OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.
IMPLEMENTED is not VERIFIED.

## Handoff rule
At the end of a work session, update CURRENT_STATE.md, ISSUES.md, ROADMAP.md, CHANGELOG.md, and this handoff when relevant. Append a dated SESSION_LOG.md entry with exact work and verification results. Clearly state what was and was not tested.
