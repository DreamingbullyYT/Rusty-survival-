# Project Memory

## Identity
Project: DreamMod
Repository: DreamingbullyYT/Rusty-survival-
Target engine: Horizon
Integration: pinned Horizon .hmod/native integration with the existing mod API and host
Runtime model: interpreted .dmod source, no per-mod compilation on player devices

## Mission
Build a powerful, deeply extensible runtime modding platform for Horizon. Mods can add gameplay and engine systems, customize existing behavior, and replace designated subsystems through versioned contracts. Ordinary .dmod packages are loaded and interpreted at runtime without Rust/native compilation, bytecode compilation or a game rebuild on the player's device.

## Distribution model
1. DreamMod Core is built in CI as a trusted native .hmod mod against a pinned Horizon SDK; use the existing mod API/host and native route. Use horizon-mixins only where an extension point is missing.
2. The resulting prebuilt Horizon distribution contains DreamMod Core.
3. The separate launcher installs and updates that prebuilt distribution.
4. Player-installed .dmod packages contain interpreted source and assets; they are dynamically loaded by DreamMod.

Current Horizon docs describe .hmod as the package format. Native source is normally compiled by the launcher; the native host can also load prebuilt adjacent libraries, a path we must prove end-to-end. See docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md.

## Ground rules
- Integrate with Horizon through pinned upstream extension points; do not copy Horizon source into this repository.
- Work directly on main; no PRs or feature branches unless the user changes this rule.
- No per-mod compilation on player devices, including hidden bytecode compilation.
- Do not finalize the script language before comparative evaluation and representative benchmarks. Rhai is a candidate only.
- Keep performance-critical loops in Rust/native engine code by default; expose batched/coarse-grained script APIs.
- Design explicit contracts for subsystem replacement, state ownership, scheduling, failure and migration.
- Deep script access must not expose arbitrary unsafe pointers or unrestricted native process privileges.
- Sandbox scripts by default. Capabilities must be explicit and narrow.
- Treat DreamMod Core's prebuilt native library as trusted code with native privileges, not as a safe sandbox.
- Version every public API and record exact Horizon, mixins and hooks/API revisions.
- No fabricated test results or completion claims.
- Keep session history append-only.
- Launcher implementation is separate and out of scope.
- Existing Minelib/vanilla-engine work is historical, not the active product direction.

## Architecture
The active architecture is specified in docs/specs/DREAMMOD_ARCHITECTURE.md. The official pinned cargo-horizon-mod check and .hmod source package passed in Actions run 44285877944; the artifact was downloaded and inspected. A CI-only harness is testing the exact signed-SDK Route::Api prebuilt compiler and the real Horizon native host loader (run 44288685182 in progress). Rhai 1.27.0 AST evaluation remains a provisional candidate; tests and per-value limits passed, but a 10k-item script loop was 462.94x slower than Rust on x86_64 CI. No production .dmod loader or sandbox exists yet. The previous engine architecture and Minecraft Java 26.3 parity work are superseded for active development, but remain in Git history and existing records for traceability.

## Continuity system
PMCS documents live in docs/project-memory/. Read AI_HANDOFF.md first. Update project memory with every meaningful implementation or architecture change.


## Performance and load-budget rules (2026-11-18)
- Keep hot gameplay work in trusted Rust/Horizon code; interpreted scripts use coarse-grained, bounded batch operations and event-driven callbacks.
- Do not run one script callback per entity/item/block/vertex. Reuse one owned snapshot batch within a dispatch where safe; do not invent a tick cache before lifecycle/thread ownership is defined.
- Benchmark every optimization against the pinned Horizon baseline. No FPS or slowdown target is considered achieved without representative ARM64 measurements.


## Allocation-free bounded validation (2026-11-18)
- For player snapshot batches (maximum 256), duplicate UUID validation uses a fixed stack array of 256 u128 values and sort_unstable. It allocates no heap memory for duplicate detection and preserves original snapshot order.
- This is a bounded implementation detail, not a measured FPS claim. Keep hot-path script APIs batch-first and benchmark on ARM64.


## Shared immutable snapshots (2026-11-18)
- PlayerSnapshotBatch stores its immutable player slice behind Arc<[PlayerSnapshot]>. Cloning a batch now increments the Arc reference count instead of deep-cloning player strings/records, enabling safe sharing within a dispatch after each mod's capability gate is checked.
- No dispatcher/cache is implemented yet; do not share data with a mod before authorization.

## Shared authorization contract (2026-11-18)
- Added `mods/dreammod-contract`, a dependency-light crate with no third-party dependencies, defining the `PlayerReadAuthorization` trait and canonical `entities.read` capability identifier.
- Both DreamMod Core and the package/lifecycle crate depend on this contract instead of defining unrelated authorization traits. The package `ModLifecycle` implements the shared trait by checking its Enabled state and explicit `entities.read` grant.
- This establishes a compile-time seam without linking ZIP/JSON/semver package-inspection dependencies into trusted Horizon Core. It does not yet instantiate a runtime manager or execute `.dmod` scripts.
- Verification is pending in Core, package and overlay Actions triggered by commits `28d55c1` and `7a9356a`.

## Deterministic ModManager (2026-11-18)
- Added `mods/dreammod-package/src/manager.rs` and exported `manager::ModManager`.
- The manager owns validated `.dmod` package contents and one `ModLifecycle` per mod ID. It validates the full dependency graph up front and retains the deterministic dependency-first load order from `resolve_load_order`.
- Approval remains per-mod and does not activate. Enable requires every declared dependency to be Enabled. Disable refuses while a direct dependent remains Enabled. A failure propagates fail-closed through transitive dependents in reverse load order.
- Regression tests cover dependency-first ordering, package retention, enable ordering, disable protection, transitive failure propagation and invalid graphs.
- This is lifecycle/dependency orchestration only: no interpreter, script scheduling, host registration or Horizon runtime manager exists yet. Package workflow run `44363310681` and PMCS run `44363323071` are pending.

### ModManager cascade-order correction (2026-11-18)
- Corrected failure propagation to walk the dependency-first topological order, so a failed root is propagated to its intermediate dependent before that dependent's own dependents are visited. Reverse order was incorrect for a multi-level chain.
- The transitive cascade regression test is the required guard. The first package run `44363310681` passed but did not include manager tests because the manager module export was committed afterward; the export-triggered run must be inspected, and the corrected run must show the manager tests executing.

### Verification update — ModManager and Core authorization (2026-11-18)
- Package run `44363502581` passed: rustfmt, 41 unit tests (five manager tests included), and Clippy `-D warnings`. The transitive failure cascade test passed after the topological-order correction.
- Core run `44362777239` passed all pinned Horizon tooling/API/host-WASM checks, HNMod engine source parity, formatting, Core tests, source bundle packaging and upload.
- Overlay run `44362776804` remains in progress at API-only compilation; no claim is made about real launcher loading from this run.
- PMCS run `44363588460` is pending.

## Core/package authorization bridge — packaging-compatible callback design (2026-11-18)
- The first trait-based shared crate failed the real Horizon prebuilt compile: Horizon stages the HNMod package source but does not stage arbitrary local Cargo path dependencies. Overlay run `44362776804` exposed `unresolved import dreammod_contract`; therefore the external contract crate was removed rather than masking the error in CI.
- Core's `with_player_read_authorization` and `collect_online_players` now accept a one-shot authorization closure. This avoids any non-SDK Cargo dependency in the HNMod source package.
- `ModManager::require_capability(mod_id, capability)` checks the selected mod's lifecycle and explicit grant and can be passed directly as that closure, e.g. `collect_online_players(host, || manager.require_capability(mod_id, PLAYER_READ_CAPABILITY))`.
- This is a callback boundary, not a completed live call site: the runtime coordinator still needs to own the manager and invoke the Core collector from the correct Horizon hook/context. Core/package/overlay verification after this correction is pending.

### Callback bridge hardening follow-up (2026-11-18)
- Added `ModManager::require_capability(mod_id, capability)` as the per-mod authorization callback target for Core. A regression test covers disabled, approved-but-not-enabled, enabled+granted, and denied capability states.
- Added a manager regression test for a transitive failure cascade where an intermediate dependent was already Failed before its dependency failed. The cascade now skips re-failing an already-failed dependent while still propagating the failure to its dependents.
- Package run `44364113419` passed format/tests/Clippy before the last cascade hardening commit; new package run for `4103668` is pending. Core run `44363904945` is still building pinned Horizon tooling; overlay run `44363904976` is still compiling API-only route.

### Latest verification snapshot (2026-11-18)
- Package run `44364198594` on commit `4103668fee`: PASS — rustfmt, 42 tests (all seven ModManager tests included), and Clippy `-D warnings`.
- PMCS run `44364312080` on the updated documentation head: PASS — six validator regression tests and structural/freshness validation.
- Core run `44363904945` and API-only overlay run `44363904976` are still in progress at Horizon tooling/API-only compilation; no post-callback Core or host-loader success is claimed yet.

## Capability-gated package asset access (2026-11-18)
- Added `ModManager::read_asset(mod_id, path)` to expose validated `assets/` files from retained package memory without filesystem extraction.
- The method checks the mod's Enabled state and explicit `assets.read` grant before looking up bytes, validates the requested path with the same archive-path rules used during inspection, and returns a borrowed immutable byte slice.
- Tests cover awaiting approval, missing grant, successful byte read, traversal rejection and missing assets. Package workflow after commit `f90d0b1` is pending.

### Asset API verification — 2026-11-18
- Package run `44364618050` on commit `f90d0b1dfe`: PASS — all 43 tests, including the new capability-gated asset read test, rustfmt and Clippy.
- PMCS run `44364715327` has completed its validator regression tests and structural/freshness validation successfully; post-job cleanup is finishing.
- Core run `44363904945` has passed pinned Horizon checker build, native/API/host-WASM checks, mirror parity and formatting; Core unit tests are running. Overlay run `44363904976` remains in API-only prebuilt compilation.

### Core callback refactor verification — 2026-11-18
- Core run `44363904945` completed SUCCESS after the closure-based authorization refactor: pinned `cargo-horizon-mod`, native manifest/source/API/host-WASM validation, HNMod engine mirror parity, formatting, Core tests, source packaging and upload all passed.
- Overlay run `44363904976` has passed API-only prebuilt compilation and Linux sibling-library RUNPATH setup; the real native host load test without overlay environment overrides is currently running. Artifact inspection/upload remain pending.

## Activated entrypoint source access (2026-11-18)
- Added `ModLifecycle::ensure_enabled()` as a state-only check, separate from capability grants.
- Added `ModManager::entrypoint_source(mod_id)` to borrow the validated manifest entrypoint bytes only after the mod is Enabled. The mod's own entrypoint is not represented as a user-grantable capability; privileged host APIs still require explicit capabilities.
- Added a test for denial before approval/activation and success after enable. Package CI after commits `be02358` and `ee903d7` is pending.

### Entrypoint source verification — 2026-11-18
- Package run `44365010182` on commit `ee903d7c76`: PASS — all 44 tests, including `entrypoint_source_is_borrowed_only_after_activation`, rustfmt and Clippy.
- Overlay run `44363904976` has passed API-only compile and `$ORIGIN` setup; real Horizon host loading without overlay environment overrides remains in progress.

### Overlay host-test harness correction (2026-11-18)
- Workflow review found the host-load step did not re-export `DREAMMOD_PREBUILT_DIR` in its own step environment; per-step shell exports do not persist across GitHub Actions steps.
- Updated `.github/workflows/dreammod-prebuilt.yml` to export the exact `$RUNNER_TEMP/dreammod-prebuilt` directory in the loader step, list files and ELF RUNPATHs for diagnostics, and bound the host-load test to 180 seconds.
- Prior overlay run `44363904976` passed API-only compilation and `$ORIGIN` setup but was cancelled by this workflow correction while the host-test command was running. A new overlay run will verify the corrected environment contract.


### ModManager atlas batch ownership — 2026-11-19
- Atlas declarations are stored per mod by `ModManager`; enabled mods can atomically replace their batch and Core-facing code can request one deterministic snapshot.
- Validation enforces owner namespace, safe atlas IDs/definition paths, duplicates and the 32-atlas active cap. Rejected batch changes roll back; disable/failure cleanup releases declarations.
- Package run `44468717308` passed: 62 tests, formatting and Clippy. Still no Core/Horizon live atlas registration.
