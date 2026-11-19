# DreamMod Hybrid API

## Goal

DreamMod presents one stable API to player-installed `.dmod` mods. Where Horizon provides a compatible API operation, the Horizon adapter may service it. Missing operations use the DreamMod-native adapter. Mods call the stable DreamMod contract and never bind directly to a provider.

## Routing

- `ApiOperation`: stable domain + operation name.
- `ApiProvider`: provider descriptor advertises supported operations and implementation kind.
- `ApiRouter`: selects a provider for each operation; default preference is Horizon-first, with DreamMod as compatible fallback.
- Domain preference can be changed at runtime. Provider registration by ID replaces the previous provider immediately for subsequent calls; unregistering removes it.
- Provider errors are returned, not automatically retried. Retrying a mutation after a provider error can duplicate a side effect if it failed after partial commit. A later retry policy must require idempotency/transaction guarantees.

## Integration policy

1. Reuse Horizon's documented public API, not private engine internals, when a stable adapter can be built.
2. Use DreamMod-native contracts for unsupported operations or capabilities Horizon does not expose.
3. If both providers support an operation, allow a domain preference/override to select the provider.
4. Do not expose raw Horizon types to .dmod scripts; translate to DreamMod contract values.
5. Capabilities, ownership, validation and operation budgets remain enforced by DreamMod before provider dispatch.
6. Provider support is negotiated per operation. Never advertise a function that has no tested implementation.
7. Hot swap is a provider/route switch, not arbitrary replacement of authoritative state. Stateful subsystems require migration/snapshot/rollback contracts before switching.

## Adapter execution context

Horizon's documented code-mod surface is hook-based: each hook receives a typed host/event context, not a global API handle. Level reads/writes are available through access-specific hosts, and the native mod API is version-pinned/experimental. A live adapter must therefore run within the appropriate Horizon hook and receive that scoped host context.

Current DreamMod `ApiProvider::invoke` does not accept a host context. It is suitable for self-contained providers and test fakes, but cannot safely implement live Horizon world operations without a context contract. Do not use global mutable host pointers, unsafe cross-thread access, or fabricated snapshots. The next architecture change is a scoped provider context (engine-neutral in DreamMod, Horizon-specific in an optional adapter) passed through dispatch/router/provider invocation.

The adapter should remain optional: the base runtime must build and run without Horizon SDK dependencies. Pin adapter compatibility to the Horizon API revision, advertise only operations it actually supports, and route missing operations to DreamMod-native providers.

## Scoped provider context

The design is now specified in `docs/specs/DREAMMOD_SCOPED_API_CONTEXT.md`.

Providers will receive a borrowed operation-scoped host context through a context-aware invocation path. The context is bound to the already-authorized operation, expires when the invocation returns, and cannot be used to dispatch another operation. Existing self-contained providers keep the legacy `invoke` entrypoint through a compatibility default. Context-free calls remain available for those providers; adapters that require a host context must fail explicitly when it is missing.

The context interface is engine-neutral. Router-level context support is implemented (but CI-unverified); runtime dispatch does not yet pass contexts. A Horizon adapter will translate the scoped operation into typed callback host calls without exposing Horizon types to scripts or adding Horizon SDK dependencies to the base runtime. No global mutable handles, unsafe pointers, stale snapshots, or context caching.

Router tests now include a fake context/provider pair for context propagation and mismatched-operation rejection. Runtime-level missing-context/provider-error cases remain. The tests are not yet verified by CI.

## Horizon surface mapping (planned)

| Domain | Horizon reuse | DreamMod-native fallback/extensions |
|---|---|---|
| Lifecycle/events/hooks | Horizon hooks where adapter can translate | DreamMod event catalog and synthetic events |
| Blocks/world | Horizon level host and block helpers where available | additional queries, custom world providers, transactions |
| Registries/content | Horizon registrars/content data | unsupported custom registries and extensions |
| Entities/parts | Horizon entity hooks/Parts/attachments | extra entity behaviors and custom systems |
| Items/inventory | Horizon host/item APIs when exposed | extended item behaviors and custom inventory rules |
| Rendering/shaders | Horizon renderer/shader APIs if supported by adapter | new rendering pipelines/features not available in Horizon |
| UI/HUD | Horizon hooks/host UI surfaces where supported | custom screens, widgets, overlays |
| Audio | Horizon APIs if present and usable | audio extensions |
| Physics | reuse existing Horizon behavior only when compatible | advanced custom physics/providers (e.g. vehicle suspension) |
| Networking/server | Horizon's server-side public hooks/contracts | additional transport and protocol features |

This matrix is a plan, not a claim that every Horizon surface currently exists or is integrated.

## Current implementation

- `mods/dreammod-runtime/src/api_router.rs`: generic provider routing contract, preference switching, provider replacement/removal, tests.
- Router is integrated with host-side runtime calls and the deferred script request bridge: scripts return `api_requests`; results are injected into the requesting mod's next event as `api_results`.
- Script requests use the same operation-to-capability authorizer, request/result byte limits, and provider router. Results are one-shot and mod-isolated.
- No concrete Horizon adapter is linked or tested end-to-end yet.
- The scoped host-context contract is specified, but code integration is not implemented yet; no live world operations are available.
- The typed world/block adapter is not connected to authoritative engine world/chunk storage; no live block reads/writes are implemented.

## Reference

Horizon's API portal and stability policy:
- https://playhorizon.dev/docs/api
- https://playhorizon.dev/docs/apis
- https://playhorizon.dev/docs/api/stability
