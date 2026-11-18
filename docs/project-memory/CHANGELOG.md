

### Overlay host-loader cold-build timeout — 2026-11-18
- Run `44365336841` built the API-only overlay and applied `$ORIGIN` RUNPATH successfully, but the 180-second host-test timeout expired during dependency compilation (exit 124); this was not a loader failure.
- Commit `a3d3eef30b8f3a30cad5273874518b8d499839f5` increases the host-test timeout to 900 seconds. Run `44368761333` queued; verification pending.
