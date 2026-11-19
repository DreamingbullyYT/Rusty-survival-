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

## Latest implementation and verification
- Runtime packages load disabled and require explicit enable_mod.
- Provider registrations are cleared on fault-disable.
- Runtime exposes EngineCommandHost and dispatch_event_and_execute. Only authorized commands reach a host; it is contractually atomic. Empty batches skip the host.
- Mock-host tests verify command delivery and denied-command suppression.
- Verified commit 406b8e5: rustfmt passed, 26 runtime tests passed, Clippy passed with warnings denied. Root offline engine tests passed on 4d47315.
- This is only a generic host boundary; no concrete engine adapter or actual world/entity/physics mutations exist.

## Next steps
1. Check PMCS validation on latest memory revision.
2. Add a typed mock engine adapter with state mutation, full batch validation and rollback tests.
3. Unify package approval and runtime activation in a coordinator.
4. Start concrete domain adapters: world/block, then entities and physics.
