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

## Latest implementation and verification
- Test-only mock host supports block writes plus entity health updates in one atomic staged batch. Invalid batch preserves both maps. Verified on fd4109a: DreamMod Runtime 27 tests, rustfmt and Clippy passed; workspace offline tests and Clippy passed.

## Next steps
1. Verify CI for cross-domain atomic mock.
2. Unify package approval and runtime activation in a coordinator.
3. Begin concrete world/block adapter, then entity and physics.
