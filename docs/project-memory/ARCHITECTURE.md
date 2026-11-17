# Architecture — Proposed, Not Yet Implemented

## Principles
- Simulation and gameplay must not depend on Android APIs or renderer internals.
- Platform adapters expose capabilities through narrow interfaces.
- Renderer consumes immutable or versioned world snapshots; it does not own game rules.
- Asset/resource loading is centralized and provenance-aware.
- Persistence uses versioned schemas and explicit migrations.
- Networking must not change authoritative simulation semantics.
- Android-specific lifecycle, input, storage, audio, and surface handling stay in the Android platform layer.

## Proposed logical modules
- platform-android: Android lifecycle, surface, touch, sensors, storage, permissions.
- runtime: application lifecycle, fixed-step loop, scheduling, orchestration.
- vanilla-core: blocks, items, entities, recipes, gameplay rules.
- world: chunks, generation, lighting, fluids, collision queries.
- physics: movement, collision, interaction raycasts.
- render: GPU abstraction, Vulkan-capable backend, shaders, chunk meshing.
- resources: registries, textures, models, fonts, resource packs.
- audio: decoding, mixing, spatial audio, device output.
- persistence: worlds, player data, options, migrations.
- networking: protocol, client/server transport, synchronization.
- tests/parity: deterministic fixtures and observable behavior tests.

These are logical boundaries, not a finalized crate layout. Final crate boundaries depend on the Minelib audit and Android toolchain proof.

## Data flow
Android platform -> runtime/input -> simulation -> world snapshot -> renderer.
Persistence and networking communicate through explicit interfaces/events.
