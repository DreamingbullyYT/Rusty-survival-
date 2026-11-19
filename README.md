# DreamMod

A deeply extensible interpreted modding API built for the independent Rusty-survival engine.

- Player mods use `.dmod` packages containing interpreted source, assets and data.
- No player-side native compilation, bytecode compilation, JIT or engine rebuild.
- DreamMod is built into the engine through its own contract/runtime/host adapter crates.
- **No dependency on Horizon, HNMods, `.hmod`, horizon-mixins or an external mod host.**
- API coverage is planned across world/worldgen, blocks/items, entities/AI, physics/vehicles, player/inventory, rendering/assets, audio/UI, networking and saves.
- Deep subsystem replacement uses versioned contracts, engine-native providers, capability checks, budgets and fallback behavior.

This repository owns the API, package format, runtime, host integration, tests and documentation. The launcher only installs/updates the engine and mod packages.

Read [AI_HANDOFF.md](docs/project-memory/AI_HANDOFF.md), [independent API architecture](docs/specs/DREAMMOD_ENGINE_INDEPENDENT_API.md), [engine layout](docs/specs/ENGINE_LAYOUT.md) and [project state](docs/project-memory/CURRENT_STATE.md).

**Status:** architecture is defined and package/runtime foundations are partially implemented. The new engine currently has only coordinate primitives. The contract and registration foundation crates are in progress; the production source interpreter, live engine adapter, full API domains and end-to-end `.dmod` execution are not yet implemented or verified.
