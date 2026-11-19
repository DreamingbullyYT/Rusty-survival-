# DreamMod Engine Host Contract (Draft)

Status: partially implemented (generic authorized-command host boundary); concrete engine adapters remain draft.
Date: 2026-11-19

Implementation note: DreamMod Runtime now provides EngineCommandHost and dispatch_event_and_execute. The runtime forwards only its authorized command list; the host must apply it atomically. Mock-host tests exercise the boundary, but do not establish native engine integration.
Date: 2026-11-19

## Ownership

The engine owns a single `DreamModHost` that contains the runtime, permission policy, event queue, scheduler, provider registry, diagnostics, and adapters. Scripts remain engine-agnostic.

```
Engine Tick / Lifecycle
  -> DreamModHost.emit(EventSnapshot)
  -> Runtime interprets subscribed handlers
  -> Validate ScriptResponse (commands + provider registrations)
  -> Capability authorization
  -> EngineHostAdapter validates command batch
  -> Stage/commit native mutations
  -> Scheduler resolves provider chain
  -> Native engine executes
```

## Boundary rules

- No raw engine pointers/references/objects exposed to script.
- Use opaque typed handles with generation/version validation.
- Event payloads are owned snapshots, not borrowed engine memory.
- Script responses are data only; all commands are validated and authorized at host boundary.
- No direct filesystem/network/process access in the interpreter.
- Domain operations should be coarse-grained and batch-oriented.
- Per-mod quotas apply to event frequency, operations, response bytes, commands, asset bytes, and retained state.
- Capability is checked at execution/registration, not just at package load.
- Validation must finish before a mutation batch commits; batches should be atomic unless a domain explicitly documents otherwise.
- Each native subsystem remains responsible for authoritative invariants.

## Canonical mod lifecycle

`Discovered -> Validated -> AwaitingApproval -> Ready -> Enabled`

Optional terminal states: `Disabled`, `Failed`, `Unloaded`.

- Discovery/validation never executes source.
- Approval only grants requested permissions; it does not activate.
- Activation is explicit.
- Disabled mods cannot receive gameplay events or invoke providers.
- Disable/unload removes providers, event subscriptions, queued callbacks, owned registrations, and temporary resources.
- A failure threshold disables the mod and runs cleanup; no automatic re-enable.

## Canonical capability identifiers

Use one shared registry/source for identifiers; package validation and runtime authorization must not maintain separate lists.

Initial domain capabilities (proposed; exact granularity must be finalized before implementation):
- `world.read`, `world.write`
- `entity.read`, `entity.write`
- `physics.read`, `physics.modify`, `physics.override`
- `render.read`, `render.register`, `render.override`
- `assets.read`, `assets.register`
- `audio.read`, `audio.control`
- `input.read`, `input.register`
- `ui.register`, `ui.control`
- `ai.read`, `ai.modify`, `ai.override`
- `worldgen.register`, `worldgen.override`
- `events.listen`, `events.emit`
- `commands.register`
- `registry.write`
- `network.access` (only if/when networking exists)

Capabilities are scoped to operation/domain, not a blanket unrestricted "engine access" grant. User approval UI should show understandable descriptions and risk.

## Provider resolution

Every replaceable subsystem defines a stable subsystem ID, input/output schema version, vanilla implementation, allowed provider modes, and required capability.

- `Replace`: one winner per subsystem. Sort by priority descending; tie-break by mod ID ascending. Losing registrations are reported as conflicts; never silently depend on load order.
- `Before`/`After`: ordered middleware chain around the selected vanilla/replacement provider, with deterministic priority then mod-ID ordering. Their contracts must define whether input/output may be transformed, vetoed, or only observed.
- A provider is selected only if the mod is enabled and granted the required capability.
- Failure, disable, unload, invalid response, or incompatibility returns control to vanilla/fallback; no subsystem remains ownerless.
- Resolver returns an immutable snapshot so tick-time lookup is cheap.

## Event and scheduler

- Engine emits typed event snapshots; event names and payload schema are versioned.
- Dispatch only to enabled mods that declared/subscribed to that event and match client/server side.
- Stable dispatch order: dependency order, then explicit priority, then mod ID (final precedence to be chosen consistently).
- Tick handlers have budgets and may not block engine thread.
- Work that can be deferred is queued with limits and cancellation by mod owner.

## Command and batch API

Commands are typed (not arbitrary string/payload where possible), versioned, and grouped into a bounded batch. JSON envelope may remain as the interpreter interchange format, but adapter converts it into typed commands.

Proposed processing:
1. Decode and validate all command IDs and payload schemas.
2. Validate handles, ranges, world/chunk state and capabilities.
3. Estimate/charge budget.
4. Stage mutations.
5. Commit batch atomically; return owned result values/handles.
6. Emit diagnostics for rejected commands.

For high-throughput domains (physics, entities, block operations), provide batch methods and avoid per-item interpreter/native crossings.

## Domain adapters and phases

1. World/chunk/block + entity snapshots and mutations
2. Assets, content registries and resource ownership
3. Physics queries and batch modifications; provider slots per physics subsystem
4. Input and UI
5. Render registration/provider hooks
6. Audio
7. AI and navigation
8. World generation
9. Networking only after a real engine network layer exists

## Required test coverage

- lifecycle: no execution before explicit approval and activation
- capability: denied command/provider never reaches adapter
- atomicity: invalid item in batch leaves no partial mutations
- handle validation: stale/forged handle rejected
- resolver: repeatable winner and chain order; vanilla fallback
- teardown: disable/unload removes all mod-owned state and queued work
- package/API compatibility: invalid dependency or API range aborts staging without partial activation
- quotas: over-budget event/batch safely rejected
- end-to-end mock: .dmod -> parse -> event -> command -> authorization -> mock adapter -> state assertion
- benchmark: baseline engine tick with zero mods, disabled mods, enabled idle mods, and batch workload

