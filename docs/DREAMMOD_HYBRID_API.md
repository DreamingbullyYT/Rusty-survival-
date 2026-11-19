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
- The router is not yet wired into the script runtime's command/event path.
- No Horizon adapter is linked or tested end-to-end yet.
- The existing command host and world block adapter remain separate until the integration bridge is implemented.

## Reference

Horizon's API portal and stability policy:
- https://playhorizon.dev/docs/api
- https://playhorizon.dev/docs/apis
- https://playhorizon.dev/docs/api/stability
