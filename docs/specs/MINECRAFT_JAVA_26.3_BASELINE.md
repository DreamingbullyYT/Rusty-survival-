# Minecraft Java Edition 26.3 — Compatibility Baseline

Status: INITIAL BASELINE / NOT A COMPLETE IMPLEMENTATION SPECIFICATION
Target: Minecraft Java Edition 26.3 (Wilderness Bound)
Primary purpose: define the version-pinned observable behavior target for the independent Rusty Survival engine.

## 1. Version identity

| Property | Target value | Evidence |
|---|---:|---|
| Edition | Java Edition | Official release notes |
| Release | 26.3 | Official release notes |
| Release name | Wilderness Bound | Official release notes |
| Release date | 2026-09-15 | Official release notes |
| Data version | 5023 | `/version` reference and version metadata |
| Network protocol | 777 (0x309) | `/version` reference |
| Data pack format | 121.0 | Official release notes |
| Resource pack format | 97.1 | Official release notes |

Primary sources:
- Official 26.3 release notes: https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3
- Official 26.3 feedback/release technical notes: https://feedback.minecraft.net/hc/en-us/articles/48913133328013-Minecraft-Java-Edition-26-3
- Version metadata reference: https://minecraft.wiki/w/Commands/version
- Protocol version reference: https://minecraft.wiki/w/Protocol_version
- Pack format reference: https://minecraft.wiki/w/Data_pack_version

The official release notes are authoritative for announced features and changes. Secondary version tables are used only for metadata cross-checking. Where sources disagree, record the discrepancy and verify against a legitimate 26.3 installation's version metadata before implementation.

## 2. Compatibility contract

The engine targets observable Minecraft Java Edition 26.3 behavior, not source-code or internal-architecture equivalence.

Observable parity includes, when the corresponding subsystem is implemented:
- state transitions and game rules;
- world generation and saved-world behavior;
- block states, shapes, collision and interaction;
- fluids, light, weather and environmental simulation;
- entities, AI, movement, damage and inventories;
- crafting, recipes, loot, structures and progression;
- client presentation, rendering order, particles, audio and UI behavior;
- data/resource pack loading and version compatibility;
- network protocol behavior if multiplayer is included in the supported scope.

A feature is not VERIFIED merely because it looks similar. It needs reproducible reference cases and matching outcomes. Record intentional exclusions and platform-driven presentation differences separately; do not silently call them parity.

## 3. 26.3 release-specific feature and behavior inventory

This inventory is a minimum version-specific checklist, not a complete list of every vanilla feature inherited from earlier releases.

### World generation and blocks
- Dappled Forest biome, generated near cold biomes.
- Poplar trees and fallen Poplar trees, with red/orange/yellow leaf variants.
- Poplar logs and wood family.
- Shelf Mushrooms in small and large forms, including growth behavior and placement on Poplar trees/fallen trees.
- Red Shrubs and their bone-meal behavior.
- Abandoned Camp structure and biome variants.
- New Explorer Map content and map item identities.
- Wool stairs and slabs for all wool colors.
- Concrete stairs and slabs for all concrete colors.

### Items and gameplay
- Cushions: no collision, cannot overlap other cushions, break when support is removed, cannot be moved by pistons.
- Straw Beds: one-use sleep item, does not set spawn, destroyed after use, dimension-specific failure behavior in Nether and End.
- Explorer Maps are separate item types rather than renamed Filled Maps; old-world conversion and cloning behavior must be tested.
- Spectator players can interact with portals to teleport.
- Suspicious Stew and Mushroom Stew crafting accepts two of any mushroom.
- Shield raising takes precedence over Hoe tilling and Shovel path-making.
- Ender Pearl teleportation resets the teleported player's momentum.
- Endermen/Shulkers no longer teleport onto Bedrock; projectile damage behavior for mounted Endermen changed.

### Entities and environment
- Persistent mobs' random walk/swim behavior deactivates when no players are nearby, like non-persistent mobs.
- New passive/neutral mob spawn set for Dappled Forest, using cold variants where applicable.
- Poplar leaf ambience, particles and sound behavior must be represented.
- New/changed tags affecting fluid flow, teleport destinations, heightmaps, fire spread, washed-away blocks, and other gameplay rules must be loaded as data, not duplicated as scattered hard-coded lists.

### Rendering and client presentation
- 26.3 introduces an order-independent transparency algorithm as the replacement for the prior Improved Transparency approach; release notes describe it as an approximation with some remaining artifacts and a higher expected performance cost.
- Terrain blocks use MultiDrawIndirect on supported devices.
- Terrain and post-process shader contracts changed to support multi-draw.
- Player map icon now shows facing direction.
- Debug overlay adds player speed and display refresh rate and supports its own GUI scale.
- Subtitle direction targets the most recent sound in range.
- 26.3 changes UI and input behavior on desktop, including SDL3, physical-key scancodes, and relative mouse mode. Android must provide equivalent game actions through touch/controller input where applicable; desktop window management itself is not an Android requirement.

### Data, resources and persistence
- Data pack format: 121.0.
- Resource pack format: 97.1.
- `level.dat` includes a `version_history` list with prior data versions.
- Block-state providers are standalone registry-backed types under `worldgen/block_state_provider`; inline object shorthand is supported for simple providers.
- World-generation provider identifiers and cache types changed; removed provider types must not be assumed present.
- 26.3 introduces/changes many block, entity, fluid, structure, potion, damage, and biome tags. Load tag definitions from the versioned data set.
- Item model elements add `shade_direction_override`; older `shade` behavior is removed.
- Palette texture locations and metadata changed.
- Map color tint source was removed.
- The `blocks.json` data generation report no longer contains its `definition` field.

### Networking and server behavior
- Protocol version target: 777 (0x309).
- Minecraft Server Management Protocol notes version 3.1.0 and additional tag-driven behavior.
- Server `white-list` property defaults to true in 26.3.
- Multiplayer implementation must use the 26.3 packet schema and state machine, not an earlier protocol version. Exact packet IDs, codecs, login/configuration/play transitions, compression, encryption, and edge cases require a separate protocol spec and reference tests.

## 4. What is deliberately not assumed yet

The release notes and metadata above do NOT establish every internal value or algorithm. Do not guess or copy values from Minelib.

The following require direct 26.3 reference capture or additional primary technical evidence before implementation:
- exact minimum/maximum build height and dimension-specific height ranges;
- chunk section layout, palette encoding, light arrays and serialization details;
- block-state registry contents and canonical property ordering;
- generation noise settings, random sources, feature placement order and seed behavior;
- light propagation, skylight, block light and update scheduling;
- fluid tick scheduling, source creation, flow decay and cross-chunk edge behavior;
- collision boxes, step height, movement constants and tick ordering;
- entity AI scheduling, pathfinding and random tick behavior;
- region file, NBT, data-fixer and migration behavior for all relevant saved data;
- full 26.3 network packet schema;
- renderer shader inputs, vertex formats, blending and depth rules.

These items remain OPEN until supported by reproducible fixtures. Do not put guessed constants into production code.

## 5. Reference and differential-test method

Use a legally obtained, unmodified Minecraft Java Edition 26.3 installation as a black-box reference. Do not redistribute its jars or assets. Record the exact version metadata used for every fixture.

For each fixture, store:
- fixture ID and target subsystem;
- exact 26.3 build/version metadata;
- world seed and world creation options, when relevant;
- initial world/player/entity state;
- exact input/action sequence and tick count;
- expected observable result;
- capture method and known nondeterminism;
- engine result and pass/fail status.

Prefer deterministic fixtures. For inherently random behavior, record seeds and control all available random sources; otherwise compare distributions and mark the test probabilistic rather than deterministic.

Golden data may include normalized state snapshots, NBT summaries, block/biome samples, movement traces, packet traces, screenshots, and audio-event traces. Do not include Mojang assets or copyrighted game binaries in the repository unless a documented lawful basis permits it.

## 6. Required parity test groups

- Version and data identity.
- Dimension/world bounds and chunk lifecycle.
- Block registry, states, tags, shapes and updates.
- World generation, structures, biomes and feature placement.
- Lighting, fluids, weather and random ticks.
- Player movement, collision, raycasting and interaction.
- Entity simulation, AI, damage and status effects.
- Items, inventories, crafting, recipes and loot.
- Persistence, loading, saving and migration.
- Rendering, transparency, particles, maps and UI.
- Audio and subtitles.
- Networking and server behavior, if included.

Every test group starts NOT_STARTED. A subsystem may become IMPLEMENTED before it is VERIFIED. Only passing reference fixtures can move it to VERIFIED.

## 7. Android-first engineering constraints

- Android lifecycle, surfaces, touch, sensors, storage and audio belong in the platform adapter, not vanilla simulation.
- Vulkan is the preferred Android rendering backend, but do not assume desktop Vulkan behavior maps directly to Android. Prove surface creation, swapchain lifecycle, format support, synchronization and device-loss handling on real devices.
- MultiDrawIndirect and order-independent transparency are 26.3 reference behaviors, not mandatory implementation techniques. Use a different Android GPU path if necessary, but match visible output within documented tolerances.
- The simulation must be renderer-independent and testable headlessly.
- Rendering quality/performance trade-offs must not change game rules or authoritative world state.

## 8. Next specification work

1. Capture version.json and relevant data metadata from an unmodified 26.3 client/server installation.
2. Build a 26.3 registry and tag inventory from legitimate version data.
3. Specify dimensions, chunk sections, storage and block states with evidence.
4. Specify world generation, light, fluids, collision and tick ordering with fixtures.
5. Create a minimal differential test harness before implementing those systems.
6. Write the separate 26.3 network protocol specification if multiplayer remains in the initial product scope.

This document is a baseline inventory. It is not permission to begin implementing guessed internals, and it does not claim engine parity.
