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
2. Build a concrete version-pinned Horizon adapter for a small API slice, with contract-type translation and integration tests.
4. Implement DreamMod-native fallbacks for gaps; integrate real world/block storage and entity/physics adapters.
5. Unify package approval and runtime activation in a coordinator.

## Current vertical slice (verification pending)
- Typed block command adapter in runtime crate is implemented; native world backend must fulfill atomicity. Tests added; CI pending. The engine still has no actual chunk/block storage adapter.


## Hybrid API routing — implemented, verification pending
- `ApiRouter` integrated in `DreamModRuntime`: Horizon-first provider selection, DreamMod fallback by advertised operation, per-domain preference, hot replacement/removal.
- `call_api` requires a loaded and enabled mod, an authorizer-recognized operation and granted capability; validates operation IDs and limits serialized request/response sizes. Runtime tests added; CI pending.
- Important limitation: scripts cannot call this host API directly yet; implement a safe script bridge and result path before claiming script API usability.
- Architecture and API surface mapping: `docs/DREAMMOD_HYBRID_API.md`.
- No concrete Horizon adapter/end-to-end API exists yet. Don't claim Horizon integration until tested.


## Script API bridge design
- Read `docs/specs/DREAMMOD_SCRIPT_API_BRIDGE.md` before implementation.
- Safe initial design is event-scoped deferred request/response: scripts emit `api_requests`; runtime routes/authorizes; responses are delivered in the next event under `api_results` and correlated by mod-local IDs.
- Design only: no bridge code, queue, script schema, or integration tests exist yet. Host-side `call_api` is still the only entry point.


## Bridge contract primitives (verification pending)
- Added typed request/result envelope types in `api_router.rs` and public exports. Tests cover validation and serialization.
- No request execution pipeline yet. Do not claim scripts can call API; next implement `api_requests` response schema, per-mod result queue, event injection, and same authorization/routing.
- Commit `f484db8`; Actions/status currently returned no checks/runs, so verification is pending.


- Follow-up compile fix: `ApiOperation` now derives serde traits for request/result envelopes; commit `1ffd548`. CI remains unconfirmed.
