# DreamMod — Architecture and Integration Plan

Status: CORE BOOTSTRAP VERIFIED IN CI; INTERPRETER CANDIDATE PROTOTYPE ADDED; PREBUILT DISTRIBUTION NOT VERIFIED
Date: 2026-11-18
Target engine: Horizon (upstream engine; reuse its .hmod/native mod API and host, with horizon-mixins only where required)
Repository role: DreamMod runtime, API, package format, tests and integration source

## 1. Mission

Build DreamMod: a deeply extensible runtime modding platform for Horizon. It must let mods add new gameplay and engine systems, customize existing behavior, and replace supported subsystems through explicit runtime extension points.

Player-installed `.dmod` packages must not require Rust compilation, native compilation, bytecode compilation, or a game rebuild. Their source is interpreted at runtime.

DreamMod Core is built once as a trusted native .hmod mod against a pinned Horizon SDK. The CI-built native library and matching manifest are shipped with the prebuilt Horizon distribution. The launcher distributes the already-built Horizon package containing DreamMod Core. The launcher is a separate project and owns installation, file placement, profiles and updates.

## 2. Build-time versus runtime

### Build-time integration

- Build DreamMod Core as a trusted native Horizon mod against a specific pinned Horizon revision, using the current .hmod/native route and horizon-mixins only where native source transformations are required. Current Horizon already has a substantial public mod API and native host; DreamMod must reuse and extend these rather than duplicate them.
- Compile DreamMod Core once in the trusted CI/build pipeline against the pinned Horizon SDK.
- Verify whether the stock native host can load the resulting prebuilt library, its SHA-256/build identity metadata and matching package manifest directly from the shipped distribution. The native host visibly supports loading adjacent prebuilt libraries, but the full CI-to-distribution path is not yet proven.
- Produce a complete prebuilt Horizon distribution with DreamMod Core included; launcher only copies/installs files.
- Pin Horizon revision, horizon-mixins revision, hooks/API revision, Rust toolchain and native library identity.
- The launcher downloads and installs that finished distribution; it does not compile native .hmod source on the player's device.

Current Horizon documentation describes .hmod native packages as Rust source that the launcher normally compiles for the selected game revision. The native host also has a loader for already-built adjacent dynamic libraries with SHA-256 and build-identity checks. Our intended route is to build DreamMod Core in CI and verify that prebuilt-library path end-to-end; a source .hmod archive alone is not enough. Current docs call the package .hmod, so do not assume the older .hnmod name is still the active package format. See docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md.

### Runtime

- Load `.dmod` packages dynamically from the installed instance.
- Parse and interpret their source at runtime; no per-mod native compilation or bytecode build step.
- Validate manifests, dependencies, API version and requested capabilities before activation.
- Support enable/disable/reload and deterministic lifecycle transitions where safe.
- Keep package assets/data separate from executable scripts.

## 3. Reuse the existing Horizon mod platform

Horizon already supplies a typed mod API and host: Hooks, Host/ServerHost/ClientHost, Registrar, Parts, attachments, capabilities, dependency ordering and typed API injection points. Native-only mixins, bridges, loop slots and support modules reach private game code where necessary. DreamMod should adapt interpreted scripts to these existing contracts and extend only proven gaps; it must not create a parallel event bus, registry or dependency resolver. The current WASM runtime performs module compilation through a child process and Wasmtime when its cache is cold, so it does not satisfy the strict no-player-compilation requirement. Ordinary native .hmod source is also compiled by the launcher. See docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md for pinned source evidence and unresolved feasibility checks.

## 4. High-level architecture

```text
Pinned Horizon build
  └── DreamMod Core (CI-built native library loaded by the Horizon mod host)
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

## 5. Extensibility requirements

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

## 6. Runtime performance principles

- Horizon/Rust owns hot loops and data-oriented bulk operations by default.
- Script callbacks are event-driven and coarse-grained; avoid one interpreter crossing per block/entity/vertex every frame.
- Provide batch APIs for entity, world, physics and render operations.
- Permit a mod to replace a subsystem through a provider interface, but benchmark interpreted implementations against realistic workloads.
- Provide native built-in fallback implementations and explicit priority/override resolution.
- Zero or near-zero script dispatch overhead when no mods subscribe to a hook.
- Add per-mod execution budgets, memory limits, cancellation and fault reporting.
- Never silently claim native-level performance for interpreted code.

## 7. Security and trust

- `.dmod` scripts are sandboxed by default.
- Capabilities are explicit, narrow, declared in the manifest and granted by the user/instance policy.
- Filesystem, network, process, native-library and external-device access are denied unless a specific safe capability exists.
- A script exception or budget overrun must not corrupt engine state or crash unrelated mods.
- DreamMod Core's native library runs outside the script sandbox and can have the user's process privileges. They are trusted build-time components and must not be treated as safe untrusted downloads.
- Validate package paths, archive limits, hashes/signatures where available, dependency cycles and API compatibility.

## 8. Script language decision

OPEN — no final language has been selected.

Initial shortlist:
- **Rhai 1.27.0:** Rust-native embedding, source parser produces an AST which is evaluated by the AST walker. The optional Rhai Grain bytecode VM is deliberately not enabled in DreamMod. The prototype also disables AST optimization so we can measure the interpreter path rather than optimized-away script logic.
- **Starlark 0.14.x:** Rust interpreter with an explicit parse-to-AST and evaluator API; useful comparison for deterministic behavior and a constrained language surface.

The first Rhai evaluation prototype now exists at `mods/dreammod-runtime-prototype`. It covers event-style function calls, a coarse-grained batch host query, a batched physics-provider function, and an execution-operation budget. Its existence is not a final language decision and it is not yet integrated into DreamMod Core.

Important terminology: parsing source into an AST is required by ordinary interpreters and is not native-code or bytecode compilation. The player must never run a native compiler, bytecode compiler/VM preparation step, JIT, or game rebuild for a `.dmod`. The prototype must keep Rhai's Grain feature disabled and must not serialize compiled executable artifacts.

Before selection, compare:
- true source interpretation and whether AST parsing meets the product requirement,
- ability to expose large typed APIs safely,
- runtime throughput and allocation behavior,
- sandboxing/capability model and resource metering,
- Android/ARM64 support,
- debugging, diagnostics and hot reload,
- batched API calls and subsystem-provider workloads,
- maintenance, dependency footprint and license.

Do not select a language based only on syntax or a microbenchmark. Benchmark event dispatch, entity batches, world queries, and a coarse subsystem replacement on desktop and representative ARM64 devices.

## 9. Package contract (draft)

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

## 10. Compatibility and update model

- Every DreamMod Core build records exact Horizon, horizon-mixins and hook/API revisions.
- Each .dmod declares its compatible DreamMod API range.
- Stable API namespaces are versioned; deprecated APIs have a migration window.
- Runtime package updates must not require rebuilding Horizon unless the Core integration itself changes.
- Changes to Core's native Horizon integration require a new prebuilt game distribution.
- The launcher handles downloading/installing that distribution and package files; DreamMod does not implement launcher UI or account logic.

## 11. Development phases

1. Audit and reuse the existing Horizon mod API/host, then pin a baseline.
2. Build the smallest trusted native .hmod Core and prove CI-built library installation without launcher-side compilation.
3. Prototype and benchmark candidate interpreted runtimes; select one with evidence.
4. Define manifest, loader, lifecycle, capability model and diagnostics.
5. Expose stable gameplay/event APIs and test with a minimal .dmod.
6. Add system registration and replaceable provider contracts.
7. Add advanced subsystem replacement (AI, physics, rendering) incrementally, with performance and safety tests.
8. Add compatibility tests against new Horizon revisions and build a reproducible prebuilt distribution pipeline.

## 12. Non-goals

- No per-mod Rust compilation on player devices.
- No per-mod bytecode compilation as a hidden prerequisite.
- No claim that an ordinary .dmod can safely execute arbitrary native Rust.
- No assumption that every Horizon internal can be replaced without a designed interface.
- No launcher implementation in this repository.
- No copying Horizon source into this repository; integrate against pinned upstream extension points and keep DreamMod-owned code distinct.

## 13. Current status

- The minimal native `.hmod` Core bootstrap is implemented and its `cargo check` against pinned Horizon source passed in GitHub Actions run `44249522292` (commit `d6ca63ecea51facfaeda46e37e15e0664d188814`). This verifies API-level compilation only; it does not prove native library packaging or game startup.
- `mods/dreammod-runtime-prototype` contains an initial Rhai AST-interpreter evaluation harness with tests and a coarse batch benchmark. It is a candidate prototype, not the production runtime and not yet part of DreamMod Core.
- The runtime prototype CI workflow has been added; its result is pending.
- No `.dmod` manifest/parser/loader, package lifecycle, production sandbox, capability enforcement, stable public API, subsystem registry, prebuilt native distribution or clean game startup proof exists yet.
- No language decision has been made; Rhai and Starlark remain candidates.
- No Android/ARM64 runtime benchmark has been performed.
