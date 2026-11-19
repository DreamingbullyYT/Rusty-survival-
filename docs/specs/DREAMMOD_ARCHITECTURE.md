# DreamMod — Architecture and Integration Plan

Status: CORE SOURCE AND UNIT TESTS VERIFIED; API-ONLY PREBUILT OVERLAY CI HARNESS UNDER REPAIR; RHAI CANDIDATE PROVISIONAL
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
- Keep the current Core API-only: no native.mixin, native.module, native.bridge, native.state, native.loop or internals dependency. The pinned launcher source routes this shape through Route::Api, reads only the prebuilt SDK runtime/API slices, compiles each mod as its own dynamic library, and returns the unchanged prebuilt horizon-client plus runtime/mod libraries.
- Package the official native-build output as a versioned prebuilt game distribution, including native-mods.json. The stock native host validates adjacent library paths, SHA-256 and exact game/Rust identity before loading.
- The source package passed the official pinned cargo-horizon-mod check and produced an inspected .hmod artifact (Actions run 44285877944). A CI-only harness now calls the exact prebuilt::compile(..., Route::Api, ...) function with Horizon's signed SDK client runtime/API slices, writes the native inventory, and validates it through the real horizon-mod-host loader. The first attempt failed because CI overrode HOME and hid Rustup; fixed in 67bc587. Current run 44288685182 is pending. Even after it passes, the artifact is only an engine/native-library overlay, not a complete playable install; the user's licensed Minecraft 26.3 assets remain separate.
- Produce a complete prebuilt Horizon distribution with DreamMod Core included; launcher only copies/installs files.
- Pin Horizon revision, horizon-mixins revision, hooks/API revision, Rust toolchain and native library identity.
- The launcher downloads and installs that finished distribution; it does not compile native .hmod source on the player's device.

Current Horizon documentation describes .hmod native packages as Rust source that the launcher normally compiles for the selected game revision. The native host also has a loader for already-built adjacent dynamic libraries with SHA-256 and build-identity checks. The official native-build CLI requires a prepared Minecraft 26.3 installation; the CI harness therefore calls the same internal API-only prebuilt compiler directly from a pinned Horizon source checkout, using signed SDK slices only. It does not fetch or redistribute Mojang assets. The generated overlay must still be installed beside a matching licensed base game. Current docs call the package .hmod, so do not assume the older .hnmod name is still the active package format. See docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md.

### Runtime

- Load `.dmod` packages dynamically from the installed instance.
- Parse and interpret their source at runtime; no per-mod native compilation or bytecode build step.
- Validate manifests, dependencies, API version and requested capabilities before activation.
- Support enable/disable/reload and deterministic lifecycle transitions where safe.
- Keep package assets/data separate from executable scripts.

## 3. Current format/runtime reality (verified 2026-11-19)
| Format/runtime | Repository status |
|---|---|
| .dmod interpreted source | Intended player format; package validation/manager/lifecycle exists, script execution does not |
| Rhai | Isolated AST-walking prototype; not integrated or selected |
| WASM | No DreamMod runtime/format/adapter. Horizon cold-cache route compiles via child process/Wasmtime; not adopted |
| Native .hmod | Trusted DreamMod Core build-time integration only, CI-built and intended for prebuilt distribution |
| Player native .so/DLL | Not implemented |

Do not describe current API as supporting both WASM and native player mods. WASM would need to meet no-player-compilation and use the capability-checked Host API. Player-native execution requires a separate trust/security decision.

## 4. Reuse the existing Horizon mod platform

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
- Benchmark evidence: the current Rhai AST candidate took 265.955 ms versus 574.49 microseconds in Rust for 50 x 10,000 item updates (462.94x slower) on x86_64 CI. This is synthetic and not an ARM64 result, but it is a strong warning against per-entity/per-vertex script loops.
- Initial script providers must run at coarse event/subsystem boundaries; Rust retains high-frequency iteration and batch storage. Current Rhai prototype limits are per-array, per-map and per-string caps plus operation/call-depth/variable/function limits, not aggregate per-mod heap accounting.

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
- **Sprite (research-only reference):** a small MIT tree-walking interpreter with no bytecode/VM, but its current value model and language surface are far too limited for DreamMod without substantial development. It is not adopted as a dependency.
- **Starlark 0.14.x:** rejected for the strict no-bytecode rule. Its public API parses to AST, but its evaluator compiles the AST to bytecode before execution.

The first Rhai evaluation prototype exists at mods/dreammod-runtime-prototype. Formatting, behavior tests, Clippy and per-value resource-limit tests passed in Actions runs 44284795812 and 44285379104. The comparable synthetic x86_64 benchmark measured Rhai AST 265.955 ms vs Rust 574.49 microseconds for 50 x 10,000 item updates (462.94x). This is not Android data and warns against per-entity/per-vertex interpreted loops. Its array/map/string limits are per-value caps, not aggregate heap quotas. Rhai is not the final language and is not integrated into Core.

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
2. Build the smallest trusted API-only native .hmod Core through Horizon's Route::Api and prove CI-built library installation without launcher-side compilation.
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

- Official pinned cargo-horizon-mod check and source .hmod package build passed in Actions run 44285877944. The artifact was downloaded and inspected; it contains horizon-mod.toml, Cargo.toml, src/lib.rs and uses.json, pinned to Horizon 0b14f8a/API 0.2.
- The exact API-only Route::Api prebuilt path is source-verified in Horizon. The current CI-only harness calls prebuilt::compile with signed SDK client runtime/API slices and then tests the real horizon-mod-host native library loader. First run 44288437986 failed because CI overrode HOME and hid Rustup; fixed in 67bc587. Current run 44288685182 is pending.
- A successful overlay will contain horizon-client, adjacent runtime/mod libraries and native-mods.json. It will not contain Mojang assets; a complete playable install still needs the user's matching licensed Minecraft 26.3 assets.
- mods/dreammod-runtime-prototype contains a provisional Rhai AST evaluator candidate with tests and per-value/operation limits. The synthetic 10k-item loop was 462.94x slower than Rust on x86_64 CI. Rhai is not selected and is not integrated into Core.
- No production .dmod manifest/parser/loader, package lifecycle, production sandbox, capability enforcement, stable public API, event bus, subsystem registry or AI/physics/rendering replacement implementation exists yet.
- No aggregate per-mod heap accounting or Android/ARM64 runtime benchmark has been implemented.
- No complete playable game launch with DreamMod Core has been verified.


## Verification refresh — 2026-11-18
- Core run 44322745253 passed pinned Horizon native/API/host-WASM validation, formatting, unit tests and source packaging on HEAD 741f7ff15bb22a2b16b8a5a667fe471d3ee3ddc9.
- Overlay run 44322745328 failed before artifact creation because the injected test used the UI-facing picker Route rather than prebuilt::Route; the patch now qualifies prebuilt::Route::Api. A fresh run is required.
- The real native host-loader test did not execute in the failed overlay run. Do not claim a verified overlay or playable distribution.
- Package Arc-backed batch CI run 44318172781 and archive-inspection deadline CI run 44317125357 passed. There is no measured performance gain, dispatcher/cache, or ARM64 benchmark.


## SDK compiler compatibility — 2026-11-18
- The signed SDK for Horizon commit 0b14f8acfd97fa2c750e42795249dcc981178ca4 was produced using the compiler version declared in that source's tools/ci/rust-version: 1.99.1.
- Overlay run 44324363090 used Rust 1.95.0 and failed with E0514 for SDK crate mod_api. DreamMod's overlay workflow now uses Rust 1.99.1 and records it in metadata (commit 96d2e74071afb75f5657b0c2fe88caf2e529adc7).
- Run 44328650675 is the verification rerun. The generated crate::hooks diagnostic is not yet classified as an independent source bug; wait for matching-toolchain evidence.

## Player package directory contract (accepted 2026-11-19)
Player `.dmod` packages live in the instance's existing `horizon-mods/` directory alongside Horizon `.hmod` files. Extension-based dispatch is mandatory: DreamMod scans and handles only `.dmod`; Horizon's loader remains responsible for `.hmod`. Do not rename or intercept Horizon packages. The trusted DreamMod Core `.hmod` remains launcher/build integration, not a player package. This convention is accepted, but scanner integration is still pending.


### Shared Horizon mods folder scanner
DreamMod player packages use the existing `horizon-mods/` directory. The runtime scanner examines direct children only, dispatches `.dmod` and ignores `.hmod` (Horizon-owned), and ignores other extensions. It reports invalid package candidates and symlinks, rejects every candidate sharing a duplicate package ID, and defaults to bounded limits of 256 candidates, 256 MiB total archive input and 512 MiB retained uncompressed bytes. This scanner is library-level discovery; launcher instance-path/UI integration is pending.
