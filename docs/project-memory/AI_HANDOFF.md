# DreamMod AI Handoff

## Mandatory Entry Point
Read this handoff, then `PROJECT_MEMORY.md`, `CURRENT_STATE.md`, `DECISIONS.md` and append-only `SESSION_LOG.md` before continuing.

## Target
Rusty-survival engine with DreamMod. Reuse Horizon's documented API where compatible through optional adapters; DreamMod-native providers fill missing operations. Keep the mod-facing contract engine/provider-neutral and hot-swappable. Horizon/HNMods must not be mandatory dependencies.

## Ground rules
- Player-installed .dmod mods use interpreted source/assets; no player-device compilation.
- Engine APIs are broad and engine-neutral; native engine adapters own heavy work.
- Use owned snapshots, opaque typed handles, batches and explicit capability checks.
- PMCS is persistent project context. Append session history and refresh current-state snapshots after substantial work.
- IMPLEMENTED is not VERIFIED: do not claim checks passed without CI evidence.

## Verified historical milestones
- Runtime CI passed on `406b8e5`: format, 26 tests and Clippy with warnings denied.
- Runtime CI passed on `c89b038`: format, 27 tests and Clippy.
- Runtime CI passed on `fd4109a`: 27 tests, format and Clippy; root offline engine tests/Clippy also passed.
- This is historical verification; it does not verify current bridge code.

## Current implementation
- Runtime packages load disabled and require explicit activation; provider registrations and pending API results are cleared on fault-disable.
- EngineCommandHost boundary and test-only transactional mock are implemented. No actual world/entity engine adapter/storage is connected.
- Hybrid ApiRouter: Horizon-first default, provider fallback by advertised operation, per-domain preference, replacement/removal. No concrete Horizon adapter yet.
- Script bridge: `.dmod` returns `api_requests`; requests are validated, operation authorizer/capability checked, then routed. Results appear once on the same mod's next callback as `event.api_results`.
- Request count, per-request input/result sizes, and aggregate queued result bytes are bounded. Errors are structured; expected denial does not disable the mod. Scalar event payloads remain unchanged before results and are wrapped under `event_payload` when results are injected.
- Regression tests cover script request→provider→next event→command, capability denial, two-mod isolation using identical request IDs, and scalar payload compatibility.

## Verification status
- Latest bridge commits: `9934644`, `6ee5b26`, `db1189d`, `4fc88f8`, `09222b5`, `39d0166`, `26d955b`, `8c2f410`, `037eaed`, `0e40e51`, `7750687`.
- GitHub status/workflow lookups returned empty for latest code; local Cargo/Rust tools are unavailable. Do not claim latest tests/format/Clippy passed.
- Check CI or run `cargo fmt --manifest-path mods/dreammod-runtime/Cargo.toml -- --check`, `cargo test --all-targets --manifest-path mods/dreammod-runtime/Cargo.toml`, and `cargo clippy --all-targets --manifest-path mods/dreammod-runtime/Cargo.toml -- -D warnings` before marking verified.

## Horizon adapter blocker
- Official Horizon code mods implement `mod_api::Hooks` and receive typed hook-specific host/event contexts; Level reads and writes use access-specific host types. Native API is experimental and revision-pinned.
- DreamMod's current `ApiProvider::invoke` has no scoped host-context argument. Do not implement live Horizon world operations using global mutable state, unsafe host pointers or fabricated snapshots.
- Next: design a scoped engine-neutral provider context and pass it through dispatch/router/provider invocation, with Horizon-specific optional adapter and no SDK dependency in base runtime. Research: https://playhorizon.dev/docs/mod-code and https://playhorizon.dev/docs/api/stability.

## Next steps
1. Verify latest runtime CI and fix any compilation/rustfmt/Clippy failures; remaining test gaps include request-count bounds and disabled/unloaded cleanup.
2. Design scoped provider execution context and test it with context-aware mock providers.
3. Implement a small optional, version-pinned Horizon adapter vertical slice and advertise only tested operations.
4. Add DreamMod-native fallback providers and connect typed world/block adapter to real engine storage.
5. Unify package approval and runtime activation.