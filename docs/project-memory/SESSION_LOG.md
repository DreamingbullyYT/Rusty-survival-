

## S-2026-11-18-52 — Increase cold host-loader test budget
Date: 2026-11-18
Commit: `a3d3eef30b8f3a30cad5273874518b8d499839f5`
Goal: Let the real Horizon native host-loader test finish compiling on a cold GitHub Actions runner.
Changes:
- Inspected failed overlay run `44365336841`: API-only overlay build and per-library `$ORIGIN` RUNPATH setup passed.
- Host test hit exit 124 at 180 seconds while Cargo was still compiling the Horizon host dependency graph; no loader assertion had failed.
- Raised that step's timeout to 900 seconds while retaining the workflow's 90-minute job cap.
Verification:
- New overlay run `44368761333` queued on commit `a3d3eef30b`.
- Loader success, archive inspection and artifact upload are not yet verified.
Limitations:
- A successful CI host-loader test does not prove the production launcher resolves the overlay libraries in its actual runtime environment.
