# DreamMod — Architecture and Integration Plan

Status: ARCHITECTURE PROPOSED; NO DREAMMOD CODE IMPLEMENTED
Date: 2026-11-18
Target engine: Horizon (upstream engine; integrate through supported HNMod/horizon-mixins interfaces)
Repository role: DreamMod runtime, API, package format, tests and integration source

## 1. Mission

Build DreamMod: a deeply extensible runtime modding platform for Horizon. It must let mods add new gameplay and engine systems, customize existing behavior, and replace supported subsystems through explicit runtime extension points.

Player-installed `.dmod` packages must not require Rust compilation, native compilation, bytecode compilation, or a game rebuild. Their source is interpreted at runtime.

DreamMod Core itself is integrated into a pinned Horizon build during development/build time using an HNMod integration package. The launcher distributes the already-built Horizon package containing DreamMod Core. The launcher is a separate project and owns installation, file placement, profiles and updates.

## 2. Build-time versus runtime

### Build-time integration

- Use HNMod/horizon-mixins as a source integration mechanism to add DreamMod Core to a specific pinned Horizon revision.
- Compile/integrate DreamMod Core once in the trusted build pipeline.
- Produce a complete prebuilt Horizon distribution with DreamMod Core embedded.
- Pin Horizon revision, mixins revision, hooks/API revision, Rust toolchain and build metadata.
- The launcher downloads and installs that finished distribution; it does not compile HNMods on the player's device.

HNMod is not assumed to be a precompiled runtime plugin. Current Horizon documentation describes native HNMod packages as source that the launcher builds for the selected game revision. Therefore, distributing an HNMod source archive alone would not meet the no-device-compilation requirement.

### Runtime

- Load `.dmod` packages dynamically from the installed instance.
- Parse and interpret their source at runtime; no per-mod native compilation or bytecode build step.
- Validate manifests, dependencies, API version and requested capabilities before activation.
- Support enable/disable/reload and deterministic lifecycle transitions where safe.
- Keep package assets/data separate from executable scripts.

## 3. High-level architecture

```text
Pinned Horizon build
  └── DreamMod Core (integrated once through HNMod during build)
       ├── Package manager / manifest validation
       ├── Script runtime (interpreted source; engine choice OPEN)
       ├── Capability and sandbox layer
       ├── Lifecycle / events / scheduler
       ├── Stable DreamMod API facade
       ├── Extension-point registry
       ├── Subsystem replacement/override coordinator
       └── Diagnostics / profiling / crash isolation
                |
          .dmod packages
          ├── manifest.json
          ├── scripts/*.dream
          ├── assets/
          └── permissions.json
```

The extension facade is stable and versioned. Horizon-specific integration adapters are isolated behind it so upstream engine changes do not leak through every mod-facing API.

## 4. Extensibility requirements

DreamMod must support:

- Registering new gameplay content and behaviors.
- Listening to and emitting engine/gameplay events.
- Adding new systems and scheduled tasks.
- Replacing selected algorithms or providers.
- Overriding supported engine services.
- Registering complete alternative implementations for designated subsystems, including AI, physics and rendering where the underlying Horizon integration exposes safe replacement boundaries.
- Declaring dependencies, load order, compatibility ranges and conflicts.
- Hot reload for script/data changes when no unsafe state transition is involved.

"Deep access" means broad, documented, versioned engine capabilities and replaceable subsystem contracts. It does not mean handing interpreted scripts arbitrary Rust pointers, memory corruption primitives, or unrestricted native process access. Truly native/unsafe extensions belong to the trusted build-time Core integration, not ordinary player-installed scripts.

A subsystem is replaceable only if DreamMod can define its complete contract: inputs, outputs, ownership, scheduling, state migration, error behavior, determinism and fallback. Rendering and physics are performance-critical and may require coarse-grained/batched calls rather than interpreted per-vertex/per-collision callbacks.

## 5. Runtime performance principles

- Horizon/Rust owns hot loops and data-oriented bulk operations by default.
- Script callbacks are event-driven and coarse-grained; avoid one interpreter crossing per block/entity/vertex every frame.
- Provide batch APIs for entity, world, physics and render operations.
- Permit a mod to replace a subsystem through a provider interface, but benchmark interpreted implementations against realistic workloads.
- Provide native built-in fallback implementations and explicit priority/override resolution.
- Zero or near-zero script dispatch overhead when no mods subscribe to a hook.
- Add per-mod execution budgets, memory limits, cancellation and fault reporting.
- Never silently claim native-level performance for interpreted code.

## 6. Security and trust

- `.dmod` scripts are sandboxed by default.
- Capabilities are explicit, narrow, declared in the manifest and granted by the user/instance policy.
- Filesystem, network, process, native-library and external-device access are denied unless a specific safe capability exists.
- A script exception or budget overrun must not corrupt engine state or crash unrelated mods.
- Native HNMods run outside the script sandbox and can have the user's process privileges. They are trusted build-time components and must not be treated as safe untrusted downloads.
- Validate package paths, archive limits, hashes/signatures where available, dependency cycles and API compatibility.

## 7. Script language decision

OPEN. Rhai is an initial Rust-native interpreted-language candidate, not an approved final choice. Before locking it, compare at least:
- true source interpretation (no required bytecode compilation),
- ability to expose large typed APIs safely,
- runtime throughput and allocation behavior,
- sandboxing/capability model,
- Android/ARM64 support,
- debugging, diagnostics and hot reload,
- ability to call batched Rust APIs efficiently,
- long-term maintenance and license.

Do not select a language based only on syntax or a microbenchmark. Build a small representative prototype and benchmark event dispatch, entity batches, world queries, and a coarse subsystem replacement.

## 8. Package contract (draft)

```text
example.dmod/
  manifest.json
  permissions.json
  scripts/
    main.dream
    systems/
  assets/
```

The manifest must identify package ID, version, display name, entry points, required DreamMod API range, dependencies, conflicts, load priority and declared capabilities. Exact schema and extension name are OPEN. A package must not include native executable code.

## 9. Compatibility and update model

- Every DreamMod Core build records exact Horizon, horizon-mixins and hook/API revisions.
- Each .dmod declares its compatible DreamMod API range.
- Stable API namespaces are versioned; deprecated APIs have a migration window.
- Runtime package updates must not require rebuilding Horizon unless the Core integration itself changes.
- Changes to Core's native Horizon integration require a new prebuilt game distribution.
- The launcher handles downloading/installing that distribution and package files; DreamMod does not implement launcher UI or account logic.

## 10. Development phases

1. Verify current Horizon native-mod/HNMod integration points and pin a baseline.
2. Build the smallest HNMod integration that boots DreamMod Core without changing unrelated engine behavior.
3. Prototype and benchmark candidate interpreted runtimes; select one with evidence.
4. Define manifest, loader, lifecycle, capability model and diagnostics.
5. Expose stable gameplay/event APIs and test with a minimal .dmod.
6. Add system registration and replaceable provider contracts.
7. Add advanced subsystem replacement (AI, physics, rendering) incrementally, with performance and safety tests.
8. Add compatibility tests against new Horizon revisions and build a reproducible prebuilt distribution pipeline.

## 11. Non-goals

- No per-mod Rust compilation on player devices.
- No per-mod bytecode compilation as a hidden prerequisite.
- No claim that an ordinary .dmod can safely execute arbitrary native Rust.
- No assumption that every Horizon internal can be replaced without a designed interface.
- No launcher implementation in this repository.
- No copying Horizon source into this repository; integrate against pinned upstream extension points and keep DreamMod-owned code distinct.

## 12. Current status

Only this architecture proposal exists. No HNMod, interpreter, loader, manifest parser, API, package, benchmark, compatibility test or prebuilt distribution has been implemented or verified.
