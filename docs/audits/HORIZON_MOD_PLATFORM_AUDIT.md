# Horizon Mod Platform Audit

Audit date: 2026-11-18
Inspected Horizon commit: 1c45cf95d8431a7b49d94df007f7e75a890370c9
Inspected horizon-mixins commit: e59da9094f0c2fdda6c627fe9eff22dec6a906f0
Status: SOURCE/DOCUMENTATION INSPECTION ONLY; no Horizon code built or modified.

## 1. Existing platform is substantial

Horizon already separates engine, platform and Horizon-specific product layers. Its documented platform layer owns the mod API, mod formats, native mod host, shader APIs and other extension services.

Relevant existing components:
- `horizon-mod-api`: public mod-facing Hooks, Host/ServerHost/ClientHost, Registrar, Parts, attachments, Wire types and typed API.
- `horizon-mod-vocab`: hook declarations, host areas, events, handles, capabilities and shared API vocabulary.
- `horizon-mod-points`: generated injection-point registration and dispatch.
- `horizon-mod-runtime`: runtime state, attachment indexes and registries.
- `horizon-mod-package`: .hmod package/manifest/dependency validation.
- `horizon-mod-host`: native library loading, package checks and unified startup.
- `horizon-mod-wasm`: sandboxed WASM execution.
- `horizon-mod-source`: source inspection and Wire/API metadata tooling.
- `horizon-engine-slots` and `horizon-mod-internals`: native integration support.

Source links pinned to the inspected Horizon commit:
- [Layer boundaries](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/docs/layers.md)
- [Modding introduction](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/docs/modding/src/introduction.md)
- [Mod API crate](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-api/src/lib.rs)
- [Native host loader](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-host/src/native_libraries.rs)
- [Native host startup](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-host/src/loading.rs)

## 2. Existing extension mechanisms

The official mod API already offers:
- Named Hooks with typed events and side-specific Host interfaces.
- Parts attached to block, block-entity, item, menu, fluid and entity types.
- Attachments for mod-owned state.
- Capabilities and provider registration, including provider precedence and fallbacks.
- Typed `[[inject]]` points with head, return and replace forms.
- Native-only `[[native.mixin]]`, `[[native.loop]]`, `[[native.bridge]]` and `[[native.module]]` for private game types, function bodies and hot loops.
- Dependency resolution, load ordering, API layout checks and package refusal diagnostics.

Decision: DreamMod must reuse and extend these facilities. Do not create a parallel event bus, registry, package dependency resolver or Host API unless a documented gap requires it.

## 3. Why the existing WASM route does not meet the strict requirement

Horizon documents WASM mods as Rust source compiled by the mod author to `mod.wasm`. More importantly, the inspected game runtime has a separate child-process path named `--compile-wasm-module`, calls `wasmtime::Module::new`, and caches the compiled result. The WASM runtime therefore performs compilation on the player's machine at module load when the cache is absent.

Evidence:
- [WASM mod guide](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/docs/modding/src/how-to/wasm-mods.md)
- [WASM compiler child](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-wasm/src/compile_child.rs)
- [WASM linker](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-wasm/src/link.rs)

Decision: do not use the existing WASM runtime as the DreamMod script runtime while the requirement remains interpreted source with no player-side compilation.

## 4. Why the ordinary native-mod install path does not meet the requirement

The native-mod guide says the launcher compiles Rust source on the player's machine against the exact prebuilt game build. This is not acceptable for ordinary DreamMod packages.

The native host does, however, load already-built dynamic libraries from the executable's adjacent native-mod inventory. It validates library paths, SHA-256, game build identity and Rust compiler identity, then loads the library and obtains its exported mod entries. It subsequently checks matching native mod package manifests.

Evidence:
- [Native mod build and shipping](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/docs/modding/src/how-to/native-mods.md)
- [Native library loader](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-host/src/native_libraries.rs)
- [Native package runtime](https://github.com/horizon-rust/horizon/blob/1c45cf95d8431a7b49d94df007f7e75a890370c9/crates/horizon-mod-package/src/runtime.rs)

Potential route to verify: build DreamMod Core as a trusted native Horizon mod in CI against the pinned Horizon SDK, then ship the resulting prebuilt library, its checked inventory metadata and matching package manifest inside the prebuilt game distribution. The launcher would only install/copy the files. This is a hypothesis pending a reproducible package-build-and-launch proof; do not claim it works until verified.

## 5. Native route and mixins

The current documentation calls the package a `.hmod` and describes the native route through `[native]`, `[[native.mixin]]`, `[[native.loop]]`, `[[native.bridge]]` and `[[native.module]]`. The launcher normally compiles the native mod against one exact Horizon build; horizon-mixins applies source-level transformations to function copies without rebuilding the entire game.

Pinned mixins source inspected: [horizon-mixins](https://github.com/horizon-rust/horizon-mixins/tree/e59da9094f0c2fdda6c627fe9eff22dec6a906f0).

The older term HNMod must not be treated as proof that the current public package format is still `.hnmod`. Current inspected docs describe `.hmod` and the `horizon-mod.toml` manifest. Confirm the precise tool/format before implementation.

## 6. Recommended DreamMod integration shape

1. Implement DreamMod as an interpreted-source runtime embedded in one trusted DreamMod Core native mod.
2. Build that Core in CI against an exact Horizon and mixins revision.
3. Verify whether the existing native host can load the prebuilt Core library plus manifest from the shipped distribution without invoking launcher compilation.
4. Let DreamMod scripts call the existing Horizon mod API through a typed adapter. Reuse Hooks, Host, Parts, attachments, capabilities, API injection points and dependency metadata where sufficient.
5. Use native mixins/bridges/loop slots only for gaps that require private internals or coarse-grained subsystem replacement.
6. Keep .dmod script packages as source/data packages loaded by the Core interpreter. They must not contain native libraries or required bytecode.
7. If the existing native host cannot accept the prebuilt artifact arrangement, add the smallest build-time integration needed; do not fork or duplicate the entire mod platform.

## 7. Open technical questions

- Can a CI-built native .hmod produce a library/inventory/package layout that the stock pinned Horizon host loads unchanged?
- Does the prebuilt game distribution include the native host entrypoint and support the intended native inventory on each target platform?
- What is the minimum bridge from the script interpreter into `mod_api::Hooks`, `Host`, `Registrar` and native extension points?
- Which APIs are already available to WASM but require native access for performance or internals?
- Which engine systems expose safe replaceable function cells/slots today, and which need additional native integration?
- Can the interpreter run script source with zero bytecode/JIT compilation and predictable resource limits?

## 8. Limits of this audit

This was source/documentation inspection only. No Horizon checkout was compiled, no .hmod was built, no prebuilt native library was assembled, no runtime was launched, and no script interpreter was benchmarked. All integration feasibility claims remain OPEN until a working proof exists.
