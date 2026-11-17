# Session Log (append-only)

This file is chronological and append-only. Never rewrite or delete earlier session entries. Add a new entry at the end for each meaningful work session. The validator rejects deletion of existing non-empty journal lines in pull requests and pushes when the base commit is available.

## S-2026-11-17-01 — PMCS hardening
Date: 2026-11-17
Commit: pending
Goal: Make project continuity durable across separate AI sessions and context compression.
Changes:
- Added mandatory onboarding and handoff rules.
- Clarified Minecraft Java Edition as behavior target and Horizon as inspiration only, with no copied code.
- Added session journal and change template.
- Added automated structural and memory-freshness validation plus GitHub Actions workflow.
Verification:
- Repository contents inspected at baseline commit 39c5651ba3c032d680f5f918ba4abd50055b4fdd.
- Local validator test: pending.
- GitHub Actions run: pending.
Limitations:
- Automation is not considered VERIFIED until CI runs successfully.
- Exact Minecraft Java Edition release/version remains unselected.
