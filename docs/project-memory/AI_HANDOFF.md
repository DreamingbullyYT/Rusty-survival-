
- `dreammod-api` crate now exists with an owner-scoped registration registry (commands/events/systems/content/providers/resources), batch atomicity, namespace checks, deterministic ordering, feature requirements and cleanup. It does not dispatch or call the engine yet. Check latest Actions before claiming tests pass.


# Mandatory Entry Point

Read this handoff, then `PROJECT_MEMORY.md`, `CURRENT_STATE.md`, `DECISIONS.md`, and the append-only `SESSION_LOG.md` before continuing.

# Horizon

Horizon/HNMods are legacy experiments only. The active target is the independent Rusty-survival engine and engine-owned DreamMod host; do not make Horizon a dependency.

# IMPLEMENTED is not VERIFIED

Runtime source changes and tests are implemented but must not be reported verified until formatting, Rust tests, Clippy and relevant CI pass.

## Latest change

DreamMod runtime now loads packages disabled and exposes explicit `enable_mod`. The lifecycle test ensures inactive mods do not dispatch events. Commit `b2d6f04` introduced the fix; `30a4edabbc2dfb237650d57f8fbffeaa65bea456` corrected its test assertion. Latest Actions were failing on rustfmt and PMCS markers; runtime tests did not run yet.
