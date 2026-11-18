# Architecture — DreamMod

Status: PROPOSED; implementation not started.
Target engine: Horizon.

## Build and distribution layers
- Upstream Horizon remains the engine.
- DreamMod Core is integrated once into a pinned Horizon build through HNMod/horizon-mixins.
- The build pipeline emits a prebuilt game distribution containing DreamMod Core.
- The separate launcher installs and updates that distribution.

## Runtime layers
1. Package discovery and validation.
2. Interpreted source runtime (language decision OPEN).
3. Sandbox, capability checks, resource budgets and fault isolation.
4. Stable, versioned DreamMod API facade.
5. Event/lifecycle scheduler and batched engine calls.
6. Extension-point registry and subsystem provider/override coordinator.
7. Horizon integration adapters pinned to exact upstream revisions.

## Mod package
Draft .dmod structure:
- manifest.json
- permissions.json
- scripts/
- assets/

Packages contain source and resources, not native executable code or required bytecode.

## Extensibility
DreamMod should allow mods to register new systems and replace designated subsystem providers. Replacement contracts must define data ownership, scheduling, inputs/outputs, error handling, deterministic behavior, migration and fallback. AI, physics and rendering are target domains, but each needs explicit engine integration boundaries and performance tests.

## Performance
Keep hot loops and bulk data processing in native engine code by default. Expose coarse-grained and batch APIs; avoid per-entity/per-vertex interpreter calls. Provide native fallback providers and near-zero dispatch cost when no script uses a hook.

## Security
Scripts run sandboxed by default. Grant narrow capabilities explicitly. Do not expose arbitrary Rust pointers or native process privileges to ordinary scripts. HNMod/native integration code is trusted build-time code.

## Dependency and compatibility rules
- Pin Horizon, horizon-mixins, hooks/API, Rust toolchain and build metadata.
- Keep upstream-specific code in adapters.
- Version public DreamMod API namespaces.
- Ordinary .dmod updates must not require rebuilding Horizon.
- Core integration changes require a new prebuilt game distribution.

## Historical architecture
The former independent vanilla-engine architecture remains historical. Its Android, Minelib and Minecraft Java 26.3 decisions are not active DreamMod requirements.
