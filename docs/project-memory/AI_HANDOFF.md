# DreamMod AI Handoff

## Mandatory Entry Point
Read this handoff, then PROJECT_MEMORY.md, CURRENT_STATE.md, DECISIONS.md and the append-only SESSION_LOG.md before continuing.

## Target
Rusty-survival independent engine with engine-owned DreamMod. Horizon/HNMods are legacy and must not become dependencies.

## Ground rules
- Player-installed .dmod mods use interpreted source/assets; no player-device compilation.
- Engine APIs are broad and engine-neutral; native engine adapters own heavy work.
- Use owned snapshots, opaque typed handles, batches and explicit capability checks.
- PMCS is persistent project context. Append session history and refresh current-state snapshots after substantial work.
- IMPLEMENTED is not VERIFIED: do not claim checks passed without CI evidence.

## Latest implementation
- Runtime packages load disabled and require explicit enable_mod.
- Provider registrations are cleared on fault-disable.
- Runtime now exposes EngineCommandHost and dispatch_event_and_execute. The host receives only capability-authorized commands and must apply the batch atomically. Mock tests cover delivery and denied-operation suppression.
- Latest host boundary code commits: 6b914d9 and 463abba. CI verification is pending.

## Next steps
1. Check CI on latest code commit and fix format/test/Clippy failures.
2. Add typed mock-engine state adapter with atomic batch validation/rollback.
3. Connect package ModManager approval and runtime activation in one coordinator.
4. Then implement actual world/entity/physics domain adapters, starting with a minimal vertical slice.
