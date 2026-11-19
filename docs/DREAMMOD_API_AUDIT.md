# DreamMod API Audit (2026-11-19)

Audited repository: DreamingbullyYT/Rusty-survival-
Branch: main
Commit inspected: 0d78b4dc3940c068227ee33ef60eede6066357e5

## Verified existing pieces

- `crates/engine-core`: currently small platform-independent primitives, including coordinate/address types. It is not yet a complete game engine.
- `mods/dreammod-core`: currently Horizon-native bootstrap/hooks and atlas registration; `engine_bridge.rs` only bridges coordinate primitives.
- `mods/dreammod-package`: validates manifest and .dmod ZIP in bounded memory; path/type/size/compression checks; dependency load-order resolver; lifecycle states (awaiting approval, ready, enabled, disabled, failed); explicit capability gate; resource and atlas registries; bounded owned player snapshots and batches. Does not execute scripts.
- `mods/dreammod-runtime`: .dmod scanner/loader, manifest validation, dependency/API version checks, Rhai AST interpreter, event dispatch, JSON command envelopes, capability checks, provider registrations (Replace/Before/After), per-mod failure threshold, and disabled/unload operations.
- `mods/dreammod-runtime-prototype`: Rhai runtime experiments and coarse batch-physics test; candidate prototype, not production runtime.

## Gaps

1. Engine adapters are not implemented for world, entity, physics, render, audio, input, UI, AI or world generation. The current bridge is coordinate-only.
2. Runtime dispatch returns authorized command envelopes; no verified engine-side executor consumes them end-to-end.
3. Provider registrations are collected, but the engine-facing scheduler/resolver for replacement, chaining, conflict resolution, priorities, fallback and safe teardown is not complete/verified.
4. Event dispatch is callable, but engine event production/subscription integration is not verified.
5. Package/runtime lifecycle designs coexist. They must be reconciled into one player-facing lifecycle and one runtime path rather than two parallel systems.
6. The current loaded runtime path sets mods enabled during package loading. This must be reconciled with the explicit approval/activation lifecycle model; default-deny activation should be the player-facing rule.
7. Permissions are only meaningful when every engine adapter enforces them at the call boundary. No raw engine handles should be exposed to scripts.
8. Prototype batch physics demonstrates a direction but is not a live physics adapter or proof of performance.

## Decisions

- Keep the existing Rhai AST interpreter; do not add bytecode/native compilation to player devices.
- Retain prototype as test/benchmark material, not a second production runtime.
- Make the engine own a DreamMod host/runtime and expose language-neutral contracts.
- Keep mod scripts away from direct mutable engine objects. Use opaque handles, validated commands, snapshots and coarse-grained batch calls.
- Provide scoped replace/override: providers can replace a specific subsystem or operation, not implicitly take over the whole engine.
- Every replaceable subsystem needs a deterministic provider resolver, fallback to vanilla, conflict diagnostics, capability authorization and teardown.
- Prioritize engine-neutral API contract and a mock adapter/test harness before live subsystem adapters.

## Proposed implementation order

1. Consolidate lifecycle and ensure default-disabled, explicit approval + enable; align package manager and runtime semantics.
2. Define typed API request/response contracts and capability matrix for domains.
3. Implement engine-owned host, event queue and deterministic scheduler with provider resolver.
4. Build mock engine adapters and end-to-end tests for package -> interpreter -> authorized batch -> adapter.
5. Implement adapters in order: world/entities, assets/registry, physics, input/UI, renderer/audio, AI/worldgen.
6. Performance tests: batch crossing, allocation counts, event latency, script operation limits, disabled-mod overhead.

## Acceptance tests

- Unapproved mod cannot execute privileged commands; approved-but-disabled mod cannot run gameplay callbacks.
- Unknown commands/providers are denied.
- Replace conflict resolution is deterministic; vanilla fallback remains available.
- Unloading/disabling a mod removes its providers, registrations and queued work.
- Invalid or over-budget batch rejected before native execution; a batch is atomic or has explicitly documented partial semantics.
- Package dependency/version failure does not partially activate the load set.
- End-to-end tests use a mock engine first; live engine integration must be reported separately.

## Contract mismatches found during deeper inspection

- Package crate capability whitelist currently includes a smaller legacy set (assets.read, entities.read/write, events.emit/listen, ui.register, world.read/write) than the runtime Capability enum (world/entity, physics, AI, render, audio, UI, network, registry, commands).
- Package manifest uses `entrypoint` and API min/max; runtime manifest uses `entry` and `dreammod_api` range. These are incompatible schemas and need a canonical schema plus migration/adapter.
- Package lifecycle explicitly requires approval then enable; runtime loading currently creates loaded mods with `enabled: true`. This discrepancy must be fixed before player-facing use.
- Provider modes and priority are accepted in runtime response but no complete arbitration/scheduling behavior is verified yet.
