

### Cold host-loader compile timeout — 2026-11-18
- Overlay run `44365336841`: API-only build and ELF `$ORIGIN` RUNPATH setup passed; host-loader test exited 124 at its 180-second timeout while Cargo was still compiling Horizon dependencies (last observed: `horizon-mod-package`). This is a timeout, not evidence of a loader failure.
- Workflow commit `a3d3eef30b8f3a30cad5273874518b8d499839f5` raises the host-test timeout to 900 seconds (job cap remains 90 minutes) to permit a cold dependency build. Run `44368761333` queued; loader result and artifact upload are still unverified.
- Actual launcher runtime library search behavior remains a separate verification requirement.
