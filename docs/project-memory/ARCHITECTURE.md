# Architecture — DreamMod

Status: NATIVE CORE SOURCE PACKAGE ADDED; OFFICIAL PACKAGE CI PENDING; RHAI CANDIDATE TESTS/CLIPPY VERIFIED; PREBUILT DISTRIBUTION NOT VERIFIED.
Target engine: Horizon.

## Build and distribution layers
- Upstream Horizon remains the engine.
- DreamMod Core is integrated once into a pinned Horizon build through the current `.hmod` native-mod route; use horizon-mixins only for required native source transformations.
- The build pipeline emits a prebuilt game distribution containing DreamMod Core. Current API-only Core is eligible for Horizon's source-verified Route::Api: the game executable remains unchanged and Core is compiled into adjacent dynamic libraries.
- The separate launcher installs and updates that distribution.

## Runtime layers
1. Package discovery and validation.
2. Candidate source interpreter (Rhai prototype; final language OPEN).
3. Interpreted source runtime (language decision OPEN).
4. Sandbox, capability checks, resource budgets and fault isolation.
5. Stable, versioned DreamMod API facade.
6. Event/lifecycle scheduler and batched engine calls.
7. Extension-point registry and subsystem provider/override coordinator.
8. Horizon integration adapters pinned to exact upstream revisions.

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

The package crate now has a pure lifecycle policy helper: validation alone does not activate a mod; explicit approvals produce a Ready state, and a separate enable action is required. Disable and failure block capability checks. A generic HostAdapter::invoke boundary now checks lifecycle and capability before invoking a supplied host callback. It is not yet wired to concrete Horizon host calls and is not a sandbox.

## Dependency and compatibility rules
- Pin Horizon, horizon-mixins, hooks/API, Rust toolchain and build metadata.
- Keep upstream-specific code in adapters.
- Version public DreamMod API namespaces.
- Ordinary .dmod updates must not require rebuilding Horizon.
- Core integration changes require a new prebuilt game distribution.

## Current implementation notes
- `mods/dreammod-core` now has a native trait mod manifest pinned to Nightly 2026-11-18 (`0b14f8a`) and mixins `e59da90`; it uses API 0.2, declares no extra capabilities and has no mixin/internals/bridge/loop requirements. The new official package validation/build workflow is pending (44284751206).
- `mods/dreammod-runtime-prototype` is an isolated Rhai AST-evaluator candidate harness; it is not yet integrated into Core and does not establish a final language choice.
- Pinned Horizon source at the current Nightly confirms an API-only native build route that compiles mod dylibs against prebuilt SDK runtime/API slices without rebuilding the engine. CI output packaging and clean startup are still unverified.
- The prebuilt native library distribution path, `.dmod` loader and production security layer remain unverified/unimplemented.

## Historical architecture
The former independent vanilla-engine architecture remains historical. Its Android, Minelib and Minecraft Java 26.3 decisions are not active DreamMod requirements.


## Owned host-value boundary
The first typed bridge contract is `bridge::PlayerSnapshot` in the package-contract crate.
It contains only owned UUID/name/dimension/position values and rejects malformed identifiers
and invalid coordinates. `HostAdapter::read_player_snapshot` requires an enabled lifecycle
and the explicit `entities.read` grant before invoking the supplied Horizon read closure.
This keeps the policy and data contract independently testable without linking the ZIP
inspector into DreamMod Core.

The concrete Core-side `ServerHost` reader is intentionally a separate next step: Core must
copy Horizon handle data into the DTO while the hook's host borrow is valid, then drop every
Horizon handle before any interpreted script receives the value. Do not make
`dreammod-package` a Core dependency solely to reuse this policy primitive; the package
inspector and runtime host should remain separable to avoid pulling ZIP/manifest machinery
into the native engine unnecessarily.


## Bounded player query batches
Player reads cross the host boundary as a `PlayerSnapshotBatch`, capped at 256 unique UUIDs. One `entities.read` authorization check protects one host callback for the whole batch. This avoids one native/interpreter crossing per player and provides a hard per-call work/memory bound. A server with more than 256 online players must use explicit paging rather than silently truncating or exceeding the batch cap. The Core collector must preserve Horizon's `server_players()` connection order and copy all data before the hook's host borrow ends.


## Performance architecture update (2026-11-18)
- Player batch duplicate validation now uses a bounded contiguous UUID scratch vector and sort/adjacent comparison instead of a tree set. This removes tree-node allocation overhead while preserving player snapshot order and the 256-item bound.
- Script-facing APIs remain batch-first. The future dispatcher should collect once per relevant event and share immutable owned results with scripts participating in that dispatch, rather than re-querying Horizon per script/player.
- Do not add persistent caches until invalidation, event ordering, host-thread affinity and re-entrancy are specified.
- Package ZIP inspection accepts a monotonic deadline and defaults to 60 seconds; the future complete loader must carry the same deadline through all later stages.


## Allocation-free snapshot validation (2026-11-18)
Duplicate UUID validation for the bounded 256-player snapshot batch now uses a fixed stack array of 256 u128 values and sort_unstable. This avoids heap allocation and preserves source order. Do not generalize the fixed buffer beyond the documented batch bound. The future dispatcher should reuse immutable snapshots within one event dispatch instead of repeating host reads, subject to thread/re-entrancy and invalidation rules.


## Shared immutable batch ownership (2026-11-18)
PlayerSnapshotBatch now owns Arc<[PlayerSnapshot]> in both Core and package contract. This permits cheap immutable sharing among authorized consumers without deep-copying names/dimensions/positions. It does not itself introduce a cache or dispatcher. Every recipient must pass its own lifecycle/capability check before access; no host handles or mutable snapshot references are shared.

## Shared authorization contract (2026-11-18)
The per-mod authorization boundary now has a small shared crate at `mods/dreammod-contract`. It contains only the `PlayerReadAuthorization` trait and `PLAYER_READ_CAPABILITY` constant, with no third-party dependencies. The package crate implements the trait for `ModLifecycle`; Core consumes the trait in its gated player collector. This avoids a Core → package-inspector dependency edge and keeps ZIP parsing outside the trusted native HNMod. The integration is compile-time only until a future runtime manager owns `ModLifecycle` instances and passes the matching instance to each Core operation.

## ModManager orchestration layer (2026-11-18)
`dreammod-package::manager::ModManager` owns each validated package and its corresponding `ModLifecycle`, keyed by validated mod ID. Construction validates the complete dependency graph using the existing deterministic resolver before returning a manager. The manager keeps the dependency map and topological load order, accepts explicit per-mod approvals, requires dependencies to be Enabled before enabling a dependent, prevents disabling a dependency with an enabled direct dependent, and cascades a runtime failure to transitive dependents in reverse load order. This layer owns package bytes but does not parse or execute script source, register host resources, create threads, or retain Horizon handles. A later runtime coordinator must bridge the manager's lifecycle references into trusted Core calls.

## Authorization boundary — packaging-compatible callback
The real Horizon prebuilt compiler stages the HNMod source package but does not automatically stage arbitrary local Cargo path dependencies. The attempted `dreammod-contract` dependency failed with `unresolved import` in overlay run `44362776804`, so that crate was removed. Core now accepts an authorization closure in `with_player_read_authorization` / `collect_online_players`; the closure runs before any host access. The package-side `ModManager::require_capability(mod_id, capability)` checks the matching lifecycle and explicit grant and is designed to be passed directly as that callback. This avoids non-SDK dependencies in the HNMod source package. A live coordinator/hook call site remains future work.

### Callback bridge and failure-state behavior
`ModManager::require_capability` is the package-side callback target for Core's one-shot authorization closure. The manager regression suite covers the callback across approval/activation/grant states. During failure propagation, already-Failed intermediate dependents are recorded as failed without attempting an invalid second Failed transition; their descendants are still failed in topological order.

## In-memory asset access
`ModManager::read_asset(mod_id, path)` provides a narrow read-only asset API over validated package files. It accepts only paths under `assets/`, reuses the archive's portable path validation, checks the requesting mod's Enabled state and `assets.read` grant before lookup, and returns a borrowed immutable byte slice. No extraction or filesystem write occurs. The interpreter/host adapter must still impose per-call byte/CPU budgets and decide how resource formats are decoded; this method only exposes validated raw bytes.
