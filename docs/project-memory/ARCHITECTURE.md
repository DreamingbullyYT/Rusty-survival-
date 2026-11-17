# Architecture — Proposed Rusty Survival Engine

Status: HIGH-LEVEL DESIGN PROPOSAL / NOT IMPLEMENTED
Correctness target: Minecraft Java Edition 26.3 observable behavior
Primary platform: Android
Inspiration: Horizon's engineering and modularity ideas only. No Horizon source or implementation is copied.

## 1. Design goals

1. Keep vanilla simulation independent from Android, graphics APIs, and UI.
2. Make Minecraft Java 26.3 behavior testable without a GPU or Android device wherever possible.
3. Keep code small and modular without weakening observable parity.
4. Treat rendering, storage, audio, input, and networking as adapters around versioned game semantics.
5. Make data/resource definitions version-aware and preserve provenance.
6. Avoid premature plugin/mod APIs; the vanilla foundation comes first.
7. Do not commit to wgpu, Vulkan wrappers, storage crates, or ECS until the Android proof and dependency audits justify them.

## 2. Proposed layers and dependency direction

The intended dependency direction is:

Android platform adapter -> application runtime -> vanilla simulation/core
                                         |-> renderer adapter (reads snapshots)
                                         |-> persistence adapter
                                         |-> audio adapter
                                         |-> optional Java-protocol networking adapter

Vanilla simulation must not depend on Android APIs, winit, wgpu, Vulkan, audio devices, filesystem paths, or UI widgets.

Renderer receives immutable/versioned snapshots and emits presentation only. It must never own block rules, authoritative entity state, fluid simulation, or world generation.

Persistence serializes versioned state through explicit schemas/migrations. Networking transports and validates versioned protocol messages but must not silently alter local simulation rules.

## 3. Proposed logical modules

These are logical boundaries, not a final Cargo workspace. Split into crates only when interfaces and ownership are clear.

### platform-android
Owns Android activity/lifecycle integration, native surface handles, touch/controller input, sensors, app-private storage access, audio-device access, permissions, and pause/resume notifications.

It exposes narrow platform capabilities to the runtime. No game rules live here.

### app-runtime
Owns process/application lifecycle, fixed-step scheduling, frame orchestration, task coordination, loading screens, and high-level subsystem wiring.

It translates platform events into engine commands and sends presentation snapshots to the renderer.

### vanilla-data
Owns version-pinned registries and definitions: block/item/entity/biome/dimension identifiers, block states, tags, recipes, loot, data-driven rules, and pack metadata.

Definitions must be keyed by stable namespaced identifiers and validated against the 26.3 data set. Avoid a small fixed u16-only registry as the semantic source of truth.

### world-model
Owns dimensions, chunk coordinates, chunk sections, block-state storage, biome data, heightmaps, block entities, and chunk lifecycle.

Exact height, section format, palettes, and save representation remain OPEN until captured from a legitimate 26.3 reference installation.

### worldgen
Owns seed handling, random sources, noise settings, biome selection, carvers, features, structures, and chunk generation. It must be deterministic against controlled inputs and independently fixture-tested.

### simulation
Owns game ticks, scheduled block/fluid ticks, random ticks, block updates, lighting, weather, entity updates, inventories, recipes, loot, and vanilla gameplay rules.

The simulation is authoritative. Rendering frame rate must not change its tick results.

### physics-interaction
Owns player/entity movement, collision resolution, selection/collision shapes, ray traversal, block/entity interaction, and movement traces. Exact constants and update order must come from 26.3 fixtures.

### renderer-api
Defines renderer-facing world snapshots, camera state, render resources, capabilities, and frame submission without exposing a concrete GPU API to simulation.

### renderer-vulkan (candidate, not selected)
Potential Android renderer backend. Could use wgpu or another maintained abstraction, but only after a real-device Vulkan surface/lifecycle proof. Owns GPU resource lifetime, swapchain, chunk meshing, shaders, transparency, UI composition, and presentation.

The 26.3 release's MultiDrawIndirect and order-independent transparency are observable/technical reference points; the engine may use different GPU techniques if output remains equivalent.

### resource-packs
Owns data/resource pack discovery, safe archive/directory access, pack priority/overlays, version compatibility, schema validation, texture/model/font/sound resource resolution, and provenance records.

Android storage paths are injected by platform-android; this module does not guess home-directory paths.

### persistence
Owns world/player/options save and load, versioned schemas, region/chunk storage, atomic writes, recovery, and data migration. Exact Java save compatibility must be specified separately from internal runtime storage.

### audio
Owns sound event resolution, playback, spatialization, mixing, streaming, and subtitle events. Simulation emits semantic sound events; platform adapter provides the output device.

### java-protocol (optional scope)
Owns Minecraft Java Edition 26.3 packet codecs, connection state machine, encryption/compression and client/server synchronization if multiplayer is part of the initial target. It must be based on the 26.3 protocol specification and fixtures, never Minelib's relay protocol.

### testkit
Owns deterministic fixtures, reference traces, snapshot normalization, parity assertions, golden image/audio/packet comparisons, and headless simulation harnesses.

## 4. Cross-module contracts

- Simulation reads/writes world-model through explicit state operations and emits events.
- Worldgen produces chunk data for world-model; it does not call renderer APIs.
- Physics queries world-model shapes and returns movement/interaction results.
- Renderer consumes read-only snapshots and versioned render resources.
- Resource-packs resolves data definitions and presentation assets separately; assets cannot redefine hard-coded simulation behavior unless the 26.3 data model permits it.
- Persistence receives explicit snapshots/commands and owns no gameplay policy.
- Networking converts wire messages into validated commands/events; authoritative simulation decides their effect.
- Platform adapters provide capabilities and events; they do not become global mutable state.

## 5. Scheduling and threading principles

- Keep deterministic game-tick execution separate from variable-rate rendering.
- Treat asynchronous chunk generation, resource decoding, and GPU uploads as jobs with explicit ownership and cancellation.
- Commit world changes at controlled simulation boundaries; do not let worker threads mutate authoritative state concurrently.
- Make pause/resume and surface recreation explicit lifecycle events.
- Record seeds, tick counts, and input order in parity fixtures.
- Select job system, ECS, and synchronization primitives only after profiling and API review; no framework is mandated now.

## 6. Initial vertical slice target

The first engine vertical slice should prove architecture, not claim broad parity:

1. Android app starts and handles pause/resume.
2. Android surface is created and recreated safely.
3. GPU backend clears and presents a frame.
4. Touch input reaches runtime through a narrow interface.
5. Headless simulation test runs independently of Android/GPU.
6. A minimal test world can be saved and loaded through an injected storage provider.

Only after this is stable should chunk rendering and progressively richer vanilla behavior be added.

## 7. Open decisions

- wgpu versus direct Vulkan or another backend: OPEN pending Android proof.
- Winit versus native Android activity integration: OPEN; do not assume desktop Window API is the right boundary.
- ECS versus explicit data-oriented systems: OPEN pending simulation requirements and benchmarks.
- Chunk storage and palette representation: OPEN pending 26.3 reference capture.
- Internal save format versus Java-compatible region/NBT format: OPEN; define product compatibility requirements before selecting.
- Multiplayer in first release: OPEN; if included, Java protocol 777 must be implemented independently.
- Exact Rust crate boundaries and dependencies: OPEN pending vertical-slice proof and license audit.

## 8. Explicit non-goals for this phase

- No Horizon code reuse or fork.
- No wholesale Minelib crate adoption.
- No custom Modding API before vanilla foundations are stable.
- No renderer-first rewrite that embeds gameplay rules in shaders.
- No guessed world-height, generation, lighting, fluid, or movement constants.
- No promise of 1:1 parity until reference fixtures pass.
