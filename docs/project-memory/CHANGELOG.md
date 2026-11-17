# Project Changelog

## 2026-11-17 — PMCS automation and continuity hardening
- Corrected project direction: Minecraft Java Edition is the behavior target; Horizon is inspiration only and no Horizon code is to be copied.
- Added append-only session journal and a standard change-record template.
- Added a standard-library Python validator for required memory files, required sections, duplicate session IDs, and memory freshness when implementation files change.
- Added GitHub Actions workflow for push, pull request, and manual validation.
- Updated current state and roadmap; automation remains unverified until an Actions run succeeds.

## 2026-11-17 — Minelib world source audit
- Inspected all nine source files in `crates/world/src` and the world crate manifest.
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
