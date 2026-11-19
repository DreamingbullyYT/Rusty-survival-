# DreamMod — Independent Engine API Architecture

Status: ACTIVE DESIGN for the Rusty-survival engine; Horizon/HNMod is optional legacy integration only.
Date: 2026-11-19

## Mission

DreamMod is the engine's built-in, language-neutral mod platform. The new Rusty-survival engine must not depend on Horizon, HNMods, .hmod, horizon-mixins, or any external mod host. DreamMod is linked into the engine as its own set of crates/modules and exposes a stable Host API to interpreted .dmod packages.

Player mods contain source, manifest, configuration, and assets. They do not contain native executable libraries and do not trigger player-side native compilation, bytecode compilation, JIT, or game rebuild. Parsing source into an AST is permitted; execution must be a true source/AST interpreter unless a future user-approved decision changes this constraint.

## Layering

```
Rusty-survival engine
  engine-core / world / data / worldgen / physics / simulation / assets / render / audio / runtime
                          |
                    DreamMod Host
        (built-in engine integration, stable contracts)
          |       |       |       |       |
       events  systems  registries  providers  resource API
                          |
                 DreamMod Runtime
      package validation / interpreter / sandbox / budgets
                          |
                    *.dmod packages
```

DreamMod must be optional at engine build/configuration time where practical, but the base engine must not depend on Horizon or an external mod framework. The mod host is wired through explicit engine-owned adapter interfaces, not invasive patching/mixins. Keep mod-specific code out of vanilla gameplay modules except documented integration calls.

## Crate ownership (introduce gradually; no empty crates)

- `dreammod-contract`: stable IDs, version/range types, errors, capabilities, contract descriptors; no engine dependencies.
- `dreammod-package`: manifest/archive validation, dependencies/conflicts, lifecycle and package data; already partially implemented.
- `dreammod-api`: mod-facing API types, declarations, events, commands, read/write requests and extension contracts; no engine implementation dependencies.
- `dreammod-runtime`: source interpreter adapter, scheduler/dispatch, resource budgets, fault isolation, hot reload; package loader currently exists, execution is not production-ready.
- `dreammod-host`: engine adapter and bridge from API requests to engine-owned systems; only this crate depends on engine domain crates.
- `dreammod-sdk` (later): generated docs, stubs, examples, author tooling; not needed in player runtime.
- `dreammod-testkit`: deterministic fake host, mod fixtures, API contract tests.

The workspace must not contain empty placeholder crates. Split only when code and ownership are ready.

## Engine adapter contract

The host owns an adapter set for domains that are actually implemented by the engine. A mod must not receive direct Rust references, pointers, internal locks, engine-owned mutable collections, or unrestricted filesystem/process/network access.

Each adapter exposes:
- stable versioned operations with typed arguments/results;
- explicit read/write authority and capability check;
- ownership and lifetime rules (owned snapshots, handles with generation IDs, or bounded transaction objects);
- deterministic ordering where gameplay depends on it;
- batch operations and limits;
- validation and structured errors;
- thread/tick affinity;
- transaction/rollback rules where mutation is multi-step;
- observable diagnostics and API compatibility metadata.

Host calls validate handles, namespace ownership, capability, enabled state, API version, limits and thread context before touching the engine. A handle is an opaque ID, not a pointer; stale handles fail.

## Public API domains

### Core and lifecycle
Mod identity/version, dependency graph, API compatibility, load/enable/disable/reload, callbacks, error state, priority, namespaced IDs, per-mod diagnostics.

### Events
Typed engine events; subscribe/unsubscribe; priorities; cancellation only for cancellable events; event phases (before/after/observe); deterministic listener order; recursion limits; snapshot semantics; no event callbacks in inner per-block/per-vertex loops unless batched.

### Systems and scheduler
Register mod systems into declared stages. Stages and ordering constraints are explicit. System execution is budgeted, cancellable at safe points, and fault-isolated. No unrestricted thread creation. Optional parallel systems require declared access sets and deterministic-safe contracts.

### Registries/content
Blocks, items, fluids, entities, biomes, dimensions, recipes, tags, sounds, particles, effects, commands, components, materials, models and resource identifiers. Registrations are namespace-owned, validated, batch-transactional, conflict-aware and removed/rebuilt safely on disable/reload.

### World/worldgen
Read snapshots, bounded queries, chunk requests, block/biome writes through transactions, feature/structure providers, dimension descriptors, generator/noise providers, seed determinism, generation stage contracts, cancellation and quotas. Never expose whole unbounded worlds to scripts.

### Physics
Collision shape providers, layers/masks, queries, materials, body creation/configuration, forces/impulses, constraints/joints, solver/provider replacement at coarse boundaries. High-frequency collision/solver work remains engine-native. Custom script callbacks are not invoked once per contact unless a measured, bounded extension contract requires it.

### Vehicles
Vehicle definitions, chassis/body, wheel configuration, suspension geometry, tire/contact model provider, drivetrain, steering, brake, telemetry. Wheel-only collision semantics use explicit collision layers/masks and dedicated wheel query channels; ordinary player/entity queries must not see wheel-only surfaces. Provide native batch physics support.

### AI/entity
Entity component schemas, lifecycle, goals/behavior trees/state machines, sensors, pathfinding provider, navigation data, spawn and despawn, target/decision events. Bulk entity access through bounded query batches.

### Player/input/inventory
Input mapping and action contexts, movement providers, player state snapshots, interaction hooks, inventories/containers, item transfer transactions, crafting and status effects.

### Rendering/assets
Resource declarations and ownership, models, textures, materials, animation, shader/material providers, render feature/pass declarations, UI rendering and renderer-independent data. GPU resources are created/managed by engine; scripts submit coarse declarations/commands, never raw GPU pointers. Render provider replacements must preserve frame contract and have fallback.

### Audio/UI/network/save
Audio event and emitter APIs; UI screens/HUD/widgets with ownership; networking with validated mod channels and server authority; persistent namespaced mod data with schema versions/migrations and transactional save boundaries.

## Replaceable subsystems

Replacement is not unrestricted monkey-patching. Each provider contract specifies:
- interface and version;
- lifecycle and registration;
- inputs/outputs, state ownership;
- tick/schedule/thread;
- determinism requirements;
- performance budget and batch shape;
- error/fallback behavior;
- save migration;
- priority/conflict resolution;
- test/benchmark suite.

Engine-native default provider remains. Mod provider activation is atomic: validate, construct, then swap at a safe boundary; on failure retain fallback/previous provider. Only domains with a real engine integration point can be replaced. This is an implementation dependency: API contracts cannot promise providers for systems that do not yet exist.

## Interpreter/runtime decision requirements

Candidate language remains OPEN. The requirement is source interpretation, no bytecode/JIT/native compile on player device. Evaluate:
- actual interpreter semantics and AST parsing;
- mobile ARM64 footprint and startup;
- execution throughput and allocation;
- bounded memory/CPU/fuel, recursion and call depth;
- cancellation/safe points and fault isolation;
- typed API binding ergonomics;
- debugging/diagnostics/hot reload;
- license, maintenance and dependency risk.

Avoid per-entity/per-block/per-vertex callbacks. Scripts operate on events, snapshots, transactions and batches. Rust engine handles hot loops. Benchmark desktop and Android ARM64 before selecting language.

## Package format

Draft `.dmod` archive:
- `manifest.json`: id, version, display name, API range, entrypoint, dependencies, conflicts, load order, capabilities, declared content.
- `scripts/`: interpreted source.
- `assets/`: content.
- `data/`: definitions/config.
- optional `permissions.json`: user-facing capability request list.

No native libraries, bytecode, JIT artifacts or required engine rebuild. Validate archive path traversal, duplicate/case collisions, symlinks, compression bombs, CRC, file/entry limits, dependency cycles, namespace ownership, API compatibility and signatures (when distribution supports signing). Never extract by default; keep validated assets in bounded memory or safe virtual package access.

## Performance principles

- Zero/near-zero cost for unused mod hooks.
- One dispatch snapshot per event; no repeated host lookups per subscriber.
- Batch world/entity/resource operations.
- Engine-native loops for physics, meshing, rendering and world simulation.
- Bounded queues and work budgets; no unbounded mod tasks.
- Per-mod CPU/allocations/host-call telemetry.
- Benchmark vanilla with no mods and with inert mods to catch overhead regressions.

## Security

Default-deny capabilities. Capabilities are granular (domain.operation, scoped to namespaces/entities/world regions where possible). Scripts have no filesystem, network, process, native-library, OS, or arbitrary reflection access. Mod-owned data is isolated. Host validates every call. Faults disable/quarantine only the offending mod and clean owned registrations; failure cleanup must be idempotent.

## Compatibility

- DreamMod API has semantic versions and feature negotiation.
- Engine adapter reports supported contract IDs and versions.
- Package declares an API range and optional feature requirements.
- Engine build identity and game data version are separate from DreamMod API version.
- Breaking contract changes use versioned namespaces/contracts and migration documentation.
- Mods fail with actionable diagnostics when required capability/API features are unavailable.

## Implementation order / acceptance gates

1. **Reset integration target:** make Horizon/HNMod optional and non-required. Document the independent Rusty engine adapter. (design gate)
2. **Contract foundation:** IDs, API versions, errors, capabilities, handle rules, structured diagnostics.
3. **Host adapter skeleton:** fake host and engine host interfaces; no concrete unsupported domains.
4. **Runtime core:** manifest/package lifecycle, interpreted runtime decision, budgets, dispatch, fault isolation.
5. **Vertical slice:** one mod adds a custom command/event and namespaced data; run from package through runtime into engine, test disable/reload cleanup.
6. **Content APIs:** registries, assets, block/item definitions, data-driven behaviors.
7. **World & systems:** batch query/transaction APIs, scheduler and worldgen providers.
8. **Entity/AI/player/inventory.**
9. **Physics API:** collision channels, body/constraint contracts and test harness.
10. **Vehicle proof:** wheel-only collision channel and suspension provider with automated test.
11. **Render/audio/UI/network/save APIs.**
12. **Deep provider replacement:** only after subsystem exists and benchmark/fallback tests pass.
13. **SDK/tooling:** docs, templates, validator, compatibility inspector, mod diagnostics.

Exit criteria: an ordinary .dmod package runs interpreted source in the new engine, can use a documented API, is sandboxed and budgeted, can be disabled/reloaded without leaks, and works without HNMods or player-side compilation.

## Explicit status truth
This document is architecture, not implementation. Current engine is early: only engine-core coordinate primitives and tests exist. DreamMod package/runtime foundations are partially implemented. There is no production interpreter, full engine API, live engine host, or complete engine to bind all domains to yet.
