# Minelib Reuse Matrix — Source-Level World Audit

Audit date: 2026-11-17  
Upstream: https://github.com/Viendax/minelib  
Pinned reference: 1cd3faeca7a29acf491b794dda1eacf768d61e41  
Audit depth: workspace manifests and source-level inspection of every file in `crates/world/src`. No upstream tests were executed in this audit.

Decision meanings:
- KEEP: suitable to reuse with minimal changes after license review.
- ADAPT: useful, but requires isolation, refactoring, or semantic/platform work.
- REPLACE: implement independently; source may be used only as non-copying behavioral inspiration where appropriate.
- REJECT: incompatible or unacceptable risk.

## Crate-level decisions

| Component | Decision | Evidence / reason | Required next check |
|---|---|---|---|
| `minelib-world::Chunk` and `ChunkPos` | ADAPT | Simple 16×16 columns, block array, negative-coordinate floor division, top/bottom bounds and RLE serialization. However Y is 0..511 and there is no negative build height. | Compare target Java chunk section/world-height model; design versioned storage and palette/block-state representation. |
| `minelib-world::block` | REPLACE | Fixed custom `u16` IDs and a short hardcoded base list; not Java block-state semantics. | Design registry/state representation for the pinned Java version. |
| `minelib-world::registry` | REPLACE | Large custom registry with Minelib-specific block shapes, properties, texture sources, and custom content. | Rebuild from target-version behavior/specification, with stable names and explicit state properties. |
| `minelib-world::gen` | REPLACE | Explicitly Minelib procedural generator: continent/hill/ridge noise, custom caves and tree placement. Not Java terrain generation. | Implement target Java generation algorithms and deterministic seed behavior after version selection. |
| `minelib-world::noise` | REPLACE | Custom value noise/fBm and hash; deterministic but no evidence of Java-compatible random/noise algorithms. | Identify exact target random/noise algorithms and test against reference outputs. |
| `minelib-world::mesh` | ADAPT candidate | Contains useful general voxel-meshing techniques: visible-face extraction, AO, chunk neighborhood, cutout masks, partial shapes, fluid surfaces, light sampling. Large (about 104 KB) and tightly coupled to Minelib registry/world/light data and custom packed vertex format. | Extract algorithms behind neutral interfaces; compare block model, face culling, AO, transparency, light and chunk-section behavior to Java. Benchmark on Android GPU. |
| `minelib-world::raycast` | ADAPT candidate | Uses Amanatides & Woo DDA and shape-aware hitboxes for partial blocks. | Verify boundary/tie handling, negative coordinates, fluids, collision vs selection shapes, and exact Java interaction semantics. |
| `minelib-world::world` | ADAPT candidate | Useful chunk map, edits journal, imported chunk bases, and light-source queries. But it owns custom torch chemistry/clock behavior and assumes custom world/chunk semantics. | Split storage/chunk cache from gameplay; independently implement Java lighting, fluids, ticks, and block updates. |
| `minelib-world::flame` | REPLACE | Chemical torch-coloring gameplay is Minelib-specific. | None for vanilla parity. |

## Source-level findings

### Chunk and coordinates
- `CHUNK_SIZE = 16`; `CHUNK_HEIGHT = 512`.
- Chunk Y storage is indexed from zero; `World::get` returns air outside the supported vertical range.
- Chunk serialization uses a custom RLE representation; tests cover round-trip, old-format decoding, malformed input rejection, and negative X/Z chunk coordinates.
- This is not a drop-in model for Java world-height/section storage. The exact required height depends on the Java version selected by the user.

### Blocks and registry
- `BlockId` is a `u16`; first 32 IDs are reserved for Minelib's own base blocks and later IDs are appended.
- Registry includes custom shapes (torches, panels, stairs, posts, boxes, cross plants, fluids), custom texture-source logic, and legacy ID/meta mapping.
- Java's block states and registries are not represented by this fixed ID scheme. A compatibility layer would be substantial.

### Terrain and randomness
- Generator is named `Procedural`, with separate `Flat` and `Void` modes.
- Procedural terrain combines custom 2D fBm continent/hill/ridge terms, 3D value-noise cave tests, and custom tree placement.
- Noise uses a project-specific integer hash and smooth value noise. Determinism alone does not establish Java parity.

### Lighting and fluids
- World light query code gathers block light emitters and Minelib torches within a fixed radius (14).
- Mesh code builds a custom local light grid and stores sky/RGB values in packed vertex bits.
- Registry and mesher contain water/lava block levels and fluid surface geometry. The inspected world simulation API does not demonstrate Java-compatible fluid tick/spread behavior.
- The custom colored-chemical torch system is explicitly Minelib gameplay, not vanilla Java behavior.

### Meshing and interaction
- Mesher handles opaque, reflective, translucent geometry, ambient occlusion, partial block shapes, and custom lighting.
- Raycast uses DDA for voxel traversal and tests actual boxes for non-full blocks.
- These are useful algorithmic references, not proof of matching Java's renderer, selection/collision shapes, or update semantics.

### Tests and verification
- Source contains unit tests for deterministic noise, chunk serialization/coordinates, terrain invariants, world behavior, registry properties, and raycast cases.
- Tests were not run: this audit used GitHub source inspection and did not clone/build the upstream workspace.
- No Android build, Vulkan surface test, benchmark, or parity test has been performed.

## Cross-cutting findings
- Root workspace contains nine crates: platform, render, world, assets, net, relay, p2p, app, launcher.
- Workspace declares Rust 1.85 / edition 2021, winit 0.30, wgpu 25.
- README presents Windows and Linux as available; Android/iOS are described as in preparation.
- Current release workflow builds Windows executables only.
- Root LICENSE is AGPLv3. GitHub metadata reports NOASSERTION; the P2P package separately declares MIT OR Apache-2.0. Resolve license scope and contribution provenance before reuse.
- No Minelib code or assets have been copied into this repository.

## Recommendation
Do not adopt `minelib-world` wholesale. Reuse only general-purpose ideas after extracting and testing them behind our own interfaces. Independently implement Java-compatible block states, world height, generation, lighting, fluid simulation, and game rules. The most promising implementation candidate is the meshing/raycasting algorithmic approach, but both require semantic adaptation and Android benchmarks.
