# DreamMod AI Handoff

## Mandatory Entry Point
Read this handoff, then PROJECT_MEMORY.md, CURRENT_STATE.md, DECISIONS.md and the append-only SESSION_LOG.md before continuing.

## Target
Rusty-survival engine with DreamMod. Reuse Horizon's documented API where compatible through optional adapters; DreamMod-native providers fill missing operations. Keep the mod-facing contract engine/provider-neutral and hot-swappable. Horizon/HNMods must not be mandatory dependencies.

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
1. Verify CI for hybrid ApiRouter (format, tests, Clippy).
2. Connect ApiRouter to runtime dispatch and capability checks.
3. Build a concrete version-pinned Horizon adapter for a small API slice, with contract-type translation and integration tests.
4. Implement DreamMod-native fallbacks for gaps; integrate real world/block storage and entity/physics adapters.
5. Unify package approval and runtime activation in a coordinator.

## Current vertical slice (verification pending)
- Typed block command adapter in runtime crate is implemented; native world backend must fulfill atomicity. Tests added; CI pending. The engine still has no actual chunk/block storage adapter.


## Hybrid API routing — implemented, verification pending
- Generic `ApiRouter` with Horizon-first default, fallback to compatible DreamMod provider, per-domain preference, replacement/unregister and tests.
- Architecture and API surface mapping: `docs/DREAMMOD_HYBRID_API.md`.
- Still not wired to the .dmod runtime and no concrete Horizon adapter exists. Don't claim integration until tested.
