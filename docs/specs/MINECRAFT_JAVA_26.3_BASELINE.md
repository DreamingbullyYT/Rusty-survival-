# Minecraft Java Edition 26.3 — Reference Baseline

Status: INITIAL BASELINE / NOT A COMPLETE TECHNICAL SPECIFICATION
Target: Minecraft Java Edition 26.3 stable release
Primary goal: match observable vanilla behavior, not reproduce Mojang's internal source architecture.

## 1. Version identity

| Field | Target value | Evidence / confidence |
|---|---:|---|
| Release | 26.3 | Official Mojang release notes |
| Release date | 2026-09-15 | Official Mojang release notes |
| Protocol version | 777 | Community-maintained protocol reference; confirm against the official 26.3 server/client artifact before networking implementation |
| World/data version | 5023 | Community-maintained data-version reference; confirm against the official 26.3 artifact before save-format implementation |
| Data pack format | 121.0 | Official Mojang release notes |
| Resource pack format | 97.1 | Official Mojang release notes |
| Java runtime | Java 25 or newer | Version metadata reference; relevant to desktop reference runtime, not a requirement for our Android Rust app |

Sources:
- Official release notes: https://www.minecraft.net/en-us/article/minecraft-java-edition-26-3
- Official technical changes: https://feedback.minecraft.net/hc/en-us/articles/48913133328013-Minecraft-Java-Edition-26-3
- Protocol version reference (CC BY-SA 3.0): https://minecraft.wiki/w/Protocol_version
- Data version reference (CC BY-SA 3.0): https://minecraft.wiki/w/Data_version
- Cross-check metadata table: https://gofedako.com/mc/java/versions/

The protocol and data-version numbers above are reference metadata, not yet verified by extracting version.json from a legally obtained official 26.3 jar. Do that before implementing network handshake or world-save compatibility.

## 2. 26.3 release-specific behavior that must enter parity coverage

### World content and generation
- Dappled Forest biome, found near cold biomes.
- Poplar trees with red/orange/yellow leaf variants; fallen Poplar trees.
- Shelf Mushrooms, including small/large behavior and growth interaction.
- Red Shrubs and their bone-meal behavior.
- Abandoned Camp structures with biome-specific variants and loot containers.
- Explorer maps for Abandoned Camps and other structures.
- Poplar sapling growth can select any of the three Poplar variants.

### Blocks, items, interaction and player behavior
- Cushions: 16 color variants, sit interaction, placement/support rules, no collision, cannot be piston-moved, break when support is removed.
- Straw Beds: one-use sleep item; do not set spawn; destroyed after use; Nether/End restrictions.
- Wool stairs and slabs for every wool color.
- Concrete stairs and slabs for every concrete color; stonecutter support.
- Shelf Mushrooms have bounce behavior and sound when a living entity bounces on them.
- Red Shrubs can be composted; bone meal can grow an adjacent shrub when space permits.
- Spectator players can interact with portals to teleport.

### Gameplay and UI changes
- Explorer Maps are distinct item types rather than renamed Filled Maps; old-world conversion and map cloning must preserve type.
- Explorer/Buried Treasure maps are no longer zoomable unless their item is in the extendable_maps tag.
- Player map marker consistently shows facing direction.
- Persistent mob random walk/swim behavior deactivates when players are not nearby, like non-persistent mobs.
- Villager trades unlocked by profession level-up update while the player is trading.
- Suspicious Stew and Mushroom Stew recipes accept two of any mushroom.
- Shield raising takes precedence over hoe tilling and shovel path-making.
- Structure location and chunk generation performance changes are not behavior specifications by themselves; measure performance separately.

### Rendering and platform implications
- 26.3 replaces the prior Improved Transparency approach with order-independent transparency (OIT). Treat this as a rendering parity item; do not assume a particular internal algorithm from the release note alone.
- Terrain blocks use MultiDrawIndirect on supported devices. This is an optimization/capability path, not a requirement to force on every Android GPU.
- The desktop Java client changed from GLFW to SDL3 for window/input/platform integration. This is not a reason to adopt SDL3 in the Android Rust engine. Our Android adapter remains independently designed and must expose equivalent input/lifecycle behavior where applicable.
- The release adds post effects as resource-pack-controlled client-side screen effects. Server simulation must not assume a client has applied a visual post effect.

## 3. Core systems requiring deeper specification

The release notes do not define these systems sufficiently. Do not guess their constants or implementation details.

1. Dimension min/max Y, section count, section indexing and out-of-range behavior.
2. Chunk data layout, palettes, packed storage, block entities, biomes, heightmaps and serialization.
3. Block-state registry: identifiers, properties, default states, tags and state transitions.
4. World generation: seed pipeline, random sources, noise parameters, density functions, biome source, carvers, features, structures and placement order.
5. Lighting: sky/block light, propagation, opacity/emission, boundary updates and relighting.
6. Fluids: water/lava state, scheduling, flow, source creation, interactions and tick order.
7. Physics: movement constants, collision boxes, step-up, gravity, drag, swimming, climbing, knockback and ray selection.
8. Tick model: game time, scheduled ticks, random ticks, entity/block update order and pause semantics.
9. Rendering: model baking, culling, AO/light sampling, transparency ordering/OIT, fog, particles and animation.
10. Persistence: level.dat, region/chunk NBT, player data, data version 5023 and migration behavior.
11. Networking: protocol 777 handshake, login/configuration/play states, packet IDs, compression, encryption/authentication and keepalive.
12. Input/platform: pointer capture, keyboard/controller semantics, touch mapping, focus changes, app pause/resume and surface loss.

## 4. Evidence and implementation policy

- Every numeric constant must link to a reference artifact, a reproducible experiment, or a test fixture.
- Separate public observable behavior from internal implementation assumptions.
- Reference fixtures must record exact game version, world seed, settings, player state, action sequence and expected result.
- Differential tests compare our output with the official 26.3 reference client/server; they must not compare against Minelib as a proxy for vanilla.
- Use only lawfully obtained reference artifacts. Do not commit Mojang binaries or assets without a documented redistribution basis.
- Community wiki data used as evidence must retain attribution and comply with its CC BY-SA terms; prefer official artifacts for final constants.
- Mark every claim OPEN until backed by source, reproducible test, or fixture.

## 5. Immediate specification sequence

1. Extract version.json and server/client data from a legitimate official 26.3 installation; verify protocol, data version and pack formats.
2. Capture registries, tags, block states and item/component definitions from the reference artifact.
3. Specify chunk/dimension layout and save representation.
4. Specify block interactions, collision/selection and tick ordering.
5. Specify generation, lighting and fluid systems.
6. Build differential fixtures before implementing the corresponding systems.
