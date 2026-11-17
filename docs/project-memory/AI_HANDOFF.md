# AI Handoff — Mandatory Entry Point

Last updated: 2026-11-17
Project: Rusty Survival Engine
Repository: DreamingbullyYT/Rusty-survival-
Primary platform: Android
Correctness target: Minecraft Java Edition 26.3
Architectural inspiration: Horizon and Minelib (ideas only; no Horizon code copying)

## Mandatory onboarding
1. Read this file, then PROJECT_MEMORY.md, CURRENT_STATE.md, ARCHITECTURE.md, DECISIONS.md, ISSUES.md, ROADMAP.md, COMPATIBILITY.md, PARITY_MATRIX.md, REUSE_MATRIX.md, and SESSION_LOG.md.
2. Read docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md, docs/specs/ENGINE_TEST_STRATEGY.md, docs/specs/ENGINE_LAYOUT.md, docs/specs/REFERENCE_TEST_PIPELINE.md, and relevant documents in docs/specs/ and docs/audits/. The `rusty-survival-testkit` crate owns fixture parsing/validation; synthetic fixtures are harness tests, never Java parity evidence. The official artifact resolver is `tools/fetch_mojang_server.py`; offline unit tests live under `tools/tests/`, and the manual artifact-only workflow is `.github/workflows/reference-artifact-smoke.yml`. It does not yet run a Java oracle or claim parity.
3. Inspect current branch, HEAD commit, working tree, and recent history. Repository state is authoritative over stale prose.
4. Work directly on `main`; do not create feature branches or pull requests unless the user explicitly changes this rule.
5. Read the latest session-log entry and unresolved issues before proposing work.
6. Treat source code, Git history, reproducible test output, pinned upstream references, and reference fixtures as ground truth. Never invent progress.
7. Do not import/copy Horizon code. Horizon is architectural/technical inspiration only.
8. Do not import Minelib code until its license and component-level suitability have been reviewed and recorded.
9. Do not claim Android support, parity, performance, or test success without reproducible evidence.
10. Before implementation, state intended change, affected modules, risks, and verification plan.
11. After every meaningful change, update project memory in the same logical change set. Append to SESSION_LOG; do not rewrite old entries.
12. Run PMCS validation and regression tests where available.
13. For Rust changes run cargo fmt, cargo test --workspace --all-targets, and cargo clippy --workspace --all-targets -- -D warnings in CI.

## Non-negotiable project direction
- Android-first final platform intent; current active work is engine foundations.
- Minecraft Java Edition 26.3 observable behavior is the correctness target.
- Build an independent Rust engine; Horizon is inspiration, not a code base.
- Minelib reuse is optional and conditional on technical, license, provenance and platform review. Its root license is AGPLv3; no Minelib code has been copied.
- Modular architecture with explicit interfaces and minimal cross-system coupling.
- Keep code as small as practical without sacrificing required behavior.
- Preserve asset provenance. Never assume Mojang assets may be redistributed.
- Keep platform-specific code out of simulation/gameplay modules.
- **Current scope is the vanilla Rust engine only. Do not design or implement modding systems, mod loaders, APIs, hooks, mixins, scripting or related product features.**
- Never guess unknown 26.3 behavior; capture a reference fixture or leave it OPEN.
- Official Minecraft reference artifacts used in CI must remain temporary, verified and unpublished. The public workflow must be transparent; never disguise downloads.

## Change record requirements
Every substantial change records: date, commit, files, motivation, root cause if known, implementation, exact tests/results, risks and follow-up. Use docs/project-memory/CHANGE_TEMPLATE.md.

## Status vocabulary
OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED. IMPLEMENTED is not VERIFIED.

## Handoff rule
At the end of a work session, update CURRENT_STATE.md, ISSUES.md, ROADMAP.md, CHANGELOG.md and this handoff when relevant. Append a dated SESSION_LOG.md entry with exact work and verification. Clearly state what was and was not tested.
