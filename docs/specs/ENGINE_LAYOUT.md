# Engine Layout and Module Ownership

Status: PROPOSED IMPLEMENTATION MAP; only `engine-core` exists today.
Target: Minecraft Java Edition 26.3 observable behavior.
Primary runtime target: Android (ARM64 first).
Horizon and Minelib are references for engineering ideas, not code templates.

## 1. Rules for readable code

- A file has one clear owner and one reason to change.
- A crate owns one domain; it is not a bucket for unrelated helpers.
- Public APIs are small, documented, and named after game concepts.
- Data flow is explicit: no hidden singletons, global mutable state, or cyclic dependencies.
- Simulation is authoritative. Rendering, audio, UI and platform code observe it; they do not decide gameplay.
- Prefer simple structs and explicit systems. Do not add an ECS, job framework or abstraction without a concrete requirement and measurement.
- Split into a Cargo crate when ownership is stable and independent testing/building benefits.
- Never create empty placeholder crates just to make the tree look complete.

## 2. Staged target layout

This is the intended destination, not a claim that these crates already exist.

```text
Rusty-survival-/
├── Cargo.toml
├── crates/
│   ├── engine-core/              # coordinates, IDs, math and deterministic primitives
│   ├── engine-data/              # version-pinned registries, tags and vanilla definitions
│   ├── engine-world/             # dimensions, chunks, sections, palettes, block entities
│   ├── engine-worldgen/          # seeded noise, biomes, terrain, features, structures
│   ├── engine-physics/           # collision, movement, ray traversal and interactions
│   ├── engine-simulation/        # tick scheduler, blocks, fluids, lighting, entities, inventory
│   ├── engine-persistence/       # NBT, region files, save/load, migration and recovery
│   ├── engine-assets/            # data/resource packs, models, textures and sounds
│   ├── engine-render-api/        # renderer-neutral snapshots and frame contract
│   ├── engine-render-wgpu/       # GPU backend, chunk meshing, shaders and presentation
│   ├── engine-audio/             # sound events, mixing and spatial playback
│   ├── engine-runtime/           # fixed tick loop, task orchestration and subsystem wiring
│   ├── platform-android/         # activity lifecycle, surface, touch, sensors and storage
│   └── testkit/                  # deterministic fixtures and comparison helpers
├── app/android/                  # Android shell and UI; no Minecraft rules
├── tests/
│   ├── fixtures/                 # small authored fixtures; no proprietary jars/assets
│   └── reference/                # normalized outputs with provenance
├── tools/reference/              # development-only Java/Python harness drivers
└── docs/{audits,project-memory,specs}/
```

Introduce crates gradually. Some domains may begin as modules inside an existing crate until the boundary is proven.

## 3. Dependency direction

```text
engine-core
  ↑
engine-data ───── engine-world
                    ↑
              engine-worldgen
                    ↑
              engine-physics
                    ↑
             engine-simulation
                ↑       ↑
        persistence   assets
                ↑       ↑
             engine-runtime ── engine-render-api → engine-render-wgpu
                ↑                         ↑
        platform-android ─────────────────┘
                ↑
             app/android

testkit may depend on engine crates; shipped engine crates never depend on testkit/tools.
```

Constraints:
- `engine-core` has no engine-crate, platform, GPU or UI dependency.
- `engine-world` owns world representation, not generation or rendering.
- `engine-worldgen` produces world data through explicit interfaces.
- `engine-physics` queries world state and returns results.
- `engine-simulation` owns authoritative game rules and tick ordering.
- `engine-persistence` serializes explicit state; it does not own gameplay policy.
- `engine-assets` resolves pack resources and definitions; it does not invent game rules.
- `engine-render-api` exposes immutable snapshots; GPU backend only presents them.
- `platform-android` provides lifecycle, surface, input and storage capabilities.
- `engine-runtime` wires systems and schedules work; it does not absorb their implementations.
- No dependency cycles. Enforce the graph in CI once multiple crates exist.

## 4. Readable layout inside a domain

Example for `engine-world`:

```text
src/
├── lib.rs                 # public API and exports only
├── dimension.rs           # dimension identity and bounds
├── chunk/
│   ├── mod.rs             # Chunk public type
│   ├── position.rs
│   ├── section.rs
│   ├── palette.rs
│   ├── heightmap.rs
│   └── lifecycle.rs
├── block_state.rs
├── biome.rs
└── block_entity.rs
```

Use one concept per file. `lib.rs` and `mod.rs` declare/re-export; keep implementation elsewhere. Put private unit tests beside implementation and public contract tests in `tests/`. Document coordinate spaces, units, ownership, mutation and failure behavior. Avoid `misc.rs`, `utils2.rs`, and giant manager modules.

## 5. What we learn from the references

### Horizon: adopt ideas
- Portable game logic separated from platform code.
- Explicit dependency/layer policy.
- Small probes and deterministic differential harnesses against the official game.
- Implemented is not the same as verified.
- Keep runtime separate from launcher/application concerns.
- Measure performance against matched vanilla workloads.

### Minelib: evaluate selectively
- Euclidean chunk-coordinate conversion for negative coordinates.
- DDA voxel traversal as a candidate algorithm, after boundary/tie-case tests.
- Visible-face extraction, neighborhood-aware meshing and AO as techniques to re-evaluate.
- wgpu as a candidate GPU backend, not an automatic Android decision.

### Do not inherit
- Minelib's zero-based 512-block storage, custom block IDs/registry, custom RLE, custom generator, flame gameplay or relay protocol as Minecraft semantics.
- Minelib desktop paths/window assumptions as Android architecture.
- Any Horizon source, assets, or copied implementation.
- Benchmark claims without reproducible same-world/same-settings comparisons.

Minelib's root license is AGPLv3. No Minelib code is copied at present. Any future source reuse requires file-level provenance and legal review first.

## 6. Scope boundary

This phase is the Rust vanilla engine only. Do not design or implement mod loaders, mod APIs, hooks, mixins, scripting runtimes, mod registries or launcher modding features. Revisit only if the user explicitly changes scope.

Android is the intended first platform, but current work remains focused on headless engine foundations and reference tests. Android device work stays paused unless explicitly resumed.

## 7. Current reality

Implemented today:
- `crates/engine-core`: `BlockPos`, `ChunkPos`, `ChunkAddress`, horizontal chunk mapping.
- Five coordinate integration tests.
- Formatting, workspace tests and Clippy workflow.

Not implemented:
- All other crates/modules in the target tree.
- Java 26.3 reference fixture capture or comparison.
- World, simulation, world generation, persistence, physics, renderer, audio, runtime, Android integration or a playable game.

Do not describe the target tree as implemented. Add each component only with code, tests and a project-memory record.
