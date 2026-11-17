# Minecraft Java Edition Parity Matrix

Target Java Edition version: **26.3**
Overall status: NOT_STARTED
Rule: IMPLEMENTED is not VERIFIED. VERIFIED requires a reproducible comparison against an official 26.3 reference.

| Domain | Required observable behavior / test coverage | Status | Evidence |
|---|---|---|---|
| Version identity | Release metadata, protocol, data version, pack formats | INVESTIGATING | docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md |
| Blocks and states | Registry IDs, properties, default states, placement, breaking, drops, tags | NOT_STARTED | — |
| Items and components | Stack limits, durability, use, components, item conversions | NOT_STARTED | — |
| Player | Movement, jumping, swimming, climbing, damage, knockback, spectator portal use | NOT_STARTED | — |
| World and dimensions | Dimension bounds, chunk sections, block coordinates, heightmaps, biomes | NOT_STARTED | — |
| World generation | Seed determinism, biome source, noise, carvers, features, structures, placement order | NOT_STARTED | — |
| 26.3 world content | Dappled Forest, Poplar variants, fallen trees, Shelf Mushrooms, Red Shrubs, Abandoned Camps | NOT_STARTED | Official 26.3 release notes |
| Lighting | Sky/block light, propagation, opacity/emission, chunk-edge relighting | NOT_STARTED | — |
| Fluids | Water/lava flow, scheduling, source rules, interactions, tick ordering | NOT_STARTED | — |
| Entities and AI | Spawn/despawn, movement, collision, targeting, mob activation and random walk/swim | NOT_STARTED | — |
| Inventory and crafting | Slot rules, recipes, containers, crafting, stonecutter, creative ordering | NOT_STARTED | — |
| 26.3 items/interactions | Cushions, Straw Beds, wool/concrete stairs/slabs, Explorer Maps, recipe changes | NOT_STARTED | Official 26.3 release notes |
| Persistence | level.dat, region/chunk NBT, player data, version_history, migration and round-trip | NOT_STARTED | — |
| Networking | Protocol 777 handshake/login/configuration/play, packet ordering and disconnect behavior | NOT_STARTED | Protocol reference; verify official artifact |
| Rendering | Block models, culling, lighting, particles, fog, animation, OIT and post effects | NOT_STARTED | Official 26.3 release notes |
| Audio | Sound events, attenuation, spatialization, subtitles and entity interactions | NOT_STARTED | — |
| Input/platform | Android lifecycle, touch, controller, focus, pause/resume, surface recreation | NOT_STARTED | Android Vulkan POC pending device test |
| Performance | Chunk generation, frame time, memory, thermal behavior and GPU capability fallbacks | NOT_STARTED | — |

## Required test fixture format

Each fixture must include:
- Fixture ID and domain
- Exact reference version (26.3 stable)
- Seed, dimension, coordinates and relevant settings
- Initial world/player/entity state
- Exact action sequence and timing/tick count
- Expected observable result (state, NBT, packet, image or trace)
- Reference capture method and provenance
- Tolerance rules, if any
- Test command and result

No parity row may be marked VERIFIED from visual resemblance alone if the domain requires state, timing, persistence, protocol or physics equivalence.
