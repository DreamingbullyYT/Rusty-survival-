# Roadmap

## Phase 0 — Project continuity
- [x] Preserve PMCS, validator, regression tests and GitHub Actions workflow.
- [x] Verify legacy-aware session ID validation and its duplicate/reuse regression tests (Actions run 44299837834 passed; final PMCS run 44300145443 passed).
- [x] Record the 2026-11-18 DreamMod project pivot in persistent memory.
- [x] Define the initial DreamMod architecture and prebuilt-distribution model.

## Phase 1 — Horizon integration research
- [x] Audit current Horizon mod API/host, package formats, native route and WASM runtime.
- [x] Pin initial Horizon and horizon-mixins revisions in the Core manifest.
- [x] Add minimal native .hmod Core source and pinned-source compile workflow.
- [x] Source-verify Horizon's API-only prebuilt native library route (the CI artifact and clean startup proof remain open).
- [x] Pass official cargo horizon-mod check and package a native source .hmod in CI (run 44285877944; artifact inspected).
- [ ] Produce and inspect a CI-built API-only client engine/native-library overlay using signed SDK slices and Horizon's real host loader; current run 44289897914 pending; prior attempts exposed and fixed Rustup HOME isolation and separate launcher workspace selection; no Mojang assets are included.
- [ ] Combine the overlay with a matching licensed Minecraft 26.3 installation for a complete playable client; never bypass the official ownership check.
- [x] Pin Horizon Nightly 2026-11-18, horizon-mixins revision and API version in the native trait mod manifest, including Horizon's required `registries` capability.
- [x] Get the minimal Core bootstrap CI compile check passing (Actions run 44249522292; pinned Horizon API cargo check succeeded).
- [ ] Produce and verify a prebuilt Horizon package containing the bootstrap.
- [ ] Confirm ordinary installation does not invoke native mod compilation on the player device.

## Phase 2 — Runtime decision and prototype
- [x] Establish candidate shortlist (Rhai AST evaluator plus a research-only tree-walking reference); reject Starlark because its evaluator compiles AST to bytecode.
- [x] Add an isolated Rhai AST-evaluator prototype with typed test host calls and an explicit operation budget.
- [x] Add tests for event-style dispatch, batch world query and script-side batch physics replacement.
- [x] Verify runtime formatting, 4 behavior/budget tests and Clippy after the Vec-to-Rhai-Array fix (Actions run 44283982705).
- [x] Add black_box guards after detecting that the first Rust-native benchmark result was optimized away.
- [x] Run the comparable benchmark with equal input cloning and observable output checks (Actions 44284795812: 462.94× slower than native for a 10k-item loop on x86_64 CI).
- [ ] Design/benchmark a coarse provider boundary that minimizes script crossings and retains native hot loops; test on ARM64 before selecting a language.
- [ ] Decide explicitly whether a future optional CI-prebuilt native kernel tier is allowed for hot subsystem replacements; keep it separate from interpreted `.dmod` scripts.
- [ ] Evaluate whether a tiny tree-walking interpreter can provide enough language features for DreamMod without creating an unmaintainable custom language.
- [ ] Benchmark event dispatch, batch world/entity queries and coarse subsystem replacement on desktop and ARM64 devices.
- [ ] Select a runtime only after performance, safety and Android/target compatibility evidence.
- [x] Add prototype operation/call-depth and per-array/per-map/per-string/variable/function limits plus an array-growth regression test (Actions 44285379104 passed; not aggregate heap quotas).
- [ ] Define production execution budgets, exact memory quotas, cancellation and diagnostics.
- [x] Add prototype validation for physics provider batch input/output shape and finite numeric values (commit 33639ee0; Actions verification pending).

## Phase 3 — Core runtime
- [x] Define version-1 `.dmod` manifest and capability declaration schema in `mods/dreammod-package`.
- [x] Implement side-effect-free manifest parsing/validation for schema version, strict fields, IDs, semantic versions/API ranges, safe entrypoint paths, allowlisted/unique capabilities, dependency IDs/ranges and manifest size/count limits.
- [x] Add a side-effect-free archive inventory policy that rejects traversal, non-portable paths, case-folded duplicates, symlinks/other types, encrypted entries, unapproved roots, missing manifest/entrypoint and resource-exhaustion metadata before decompression.
- [x] Integrate an in-memory ZIP reader with strict local-header/central-directory consistency checks, bounded metadata preflight and CRC-checked streaming; no filesystem extraction (CI run 44299317538 passed: 19 tests + Clippy).
- [x] Implement deterministic dependency validation and load-order planning (runtime lifecycle remains open).
- [x] Verify the standalone package contract workflow: formatting, 19 unit tests and Clippy passed on commit 4e983c0 (Actions run 44299317538).
- [x] Verify bounded in-memory retention of validated script/asset bytes; Actions run 44300015439 passed formatting, 19 tests and Clippy.
- [x] Add pure default-deny capability grant planning with explicit approval input (Actions run 44300324687 passed 21 tests and Clippy).
- [x] Add a private-state CapabilityGate guard helper; Actions run 44300879584 passed 22 tests and Clippy. Wiring it into actual Horizon host API calls remains open.
- [x] Add a default-disabled lifecycle policy: validated → awaiting approval → ready → explicitly enabled; disable blocks calls and failure discards grants. Four lifecycle regression tests added.
- [x] Add a generic guarded host adapter that checks activation and capability before invoking a host callback; add bounded failure diagnostics. Concrete Horizon API wiring remains open.
- [x] Extend guard regression coverage so disabled and failed mods cannot invoke host callbacks; package CI run 44304258172 passed formatting, tests and Clippy.
- [x] Define an owned, validated player snapshot DTO and a typed `entities.read` adapter gate; concrete Horizon host implementation and runtime exposure remain open.
- [x] Add a bounded player snapshot batch (256 max) with duplicate-UUID rejection and one capability check per batch; concrete Horizon collector and script binding remain open; package Actions run 44309674771 passed formatting, tests and Clippy.
- [x] Inspect the exact pinned Horizon Hooks/Host/phase surface and record the bridge contract in docs/dreammod/HORIZON_HOST_BRIDGE.md.
- [x] Implement the first Core-side bounded online-player snapshot collector against the pinned ServerHost API; Core workflow run 44310547586 passed SDK build, host/WASM validation and packaging.
- [x] Require a per-mod player-read authorization interface before Core touches ServerHost; regression tests prove denial happens before the host callback. Binding the interface to actual ModLifecycle instances remains open.
- [x] Add Core-local snapshot validation/batch-limit unit tests.
- [x] Add explicit Core CI formatting and cargo test steps after pinned SDK setup; first execution result pending.
- [x] Verify the corrected gated collector source and gated-read regression tests in Core Actions run 44322745253 (HEAD 741f7ff1).
- [ ] Implement sandbox and capability enforcement.
- [ ] Load and run one minimal example .dmod end-to-end.

## Phase 4 — Extensible engine API
- [ ] Define stable versioned APIs for events, gameplay, world, entities, assets, UI and lifecycle.
- [ ] Implement system registration and ordering.
- [ ] Define subsystem provider contracts and override priority/fallback.
- [ ] Add AI and physics replacement prototypes with correctness/performance tests.
- [ ] Add rendering extension/replacement only after a viable batched contract is proven.
- [ ] Add compatibility tests against pinned and candidate Horizon revisions.

## Phase 5 — Release engineering
- [ ] Reproducible build pipeline for prebuilt Horizon + DreamMod Core.
- [ ] Generate compatibility metadata and checksums.
- [ ] Verify package update/reload behavior independently of Core rebuilds.
- [ ] Document trust, permissions, API compatibility and mod-author tooling.

## Exit criteria before engine implementation
This heading is retained for PMCS compatibility; the former standalone engine exit criteria are superseded. Active implementation may begin after Phase 1 integration points and the runtime prototype plan are evidence-backed.


## Next — performance and bounded loading (2026-11-18)
1. Verify package CI for the new 60-second deadline and contiguous UUID validation.
2. Verify Core CI after installing rustfmt; confirm cargo test actually executes.
3. Add realistic benchmark fixtures for snapshot batch validation and host-call batching; later measure ARM64 before making FPS claims.
4. When the runtime loader exists, create one deadline at load start and pass it through dependency resolution, asset/script parsing and registration. Poll cooperatively and fail the mod closed on expiry; do not block the game thread.
5. Only then add immutable per-dispatch snapshot reuse with explicit invalidation/thread rules.


## Next optimization steps (2026-11-18)
1. Verify Core and package CI for the fixed stack-buffer duplicate check.
2. Benchmark snapshot construction and duplicate validation at representative batch sizes (0, 1, 32, 128, 256).
3. Add per-dispatch immutable snapshot reuse only after scheduler thread ownership, re-entrancy and invalidation are explicit.
4. Measure host-call batching on ARM64 before stating any FPS or slowdown target.
5. Propagate the single 60-second monotonic deadline through every stage once the full .dmod loader exists.


## Shared snapshot follow-up (2026-11-18)
1. Verify Core/package CI for Arc-backed snapshot batches and pointer-identity tests.
2. Implement per-dispatch snapshot reuse only once a scheduler exists and its thread/re-entrancy semantics are proven.
3. Ensure each mod's capability is checked before exposing a shared immutable batch.
4. Benchmark cloning and batch delivery at representative player counts on x86_64 and ARM64.


## Verification refresh — 2026-11-18
1. [x] Core pinned SDK check, formatting, unit tests and source packaging pass in run 44322745253 (HEAD 741f7ff1).
2. [x] Package Arc-backed batch tests pass in run 44318172781; cooperative archive-deadline CI passed in run 44317125357.
3. [ ] Repair and rerun API-only overlay CI. Run 44322745328 exposed a type-name collision: use prebuilt::Route::Api, not the UI-facing picker::Route::Api.
4. [ ] Verify real Horizon native host loading and inspect the resulting overlay artifact. The host-loader step has not run successfully.
5. [ ] Continue with capability/lifecycle wiring only after defining a concrete per-mod runtime owner; do not add a global snapshot cache without scheduler/thread/re-entrancy semantics.
6. [ ] Add representative snapshot benchmarks, then ARM64 measurements. No FPS or throughput target is verified.


## Prebuilt toolchain compatibility (2026-11-18)
1. [x] Read the compiler version from the exact pinned Horizon SDK producer source: tools/ci/rust-version = 1.99.1.
2. [x] Change the overlay CI toolchain and artifact metadata from 1.95.0 to 1.99.1 (commit 96d2e740).
3. [ ] Verify overlay run 44328650675 reaches successful API-only prebuilt compilation.
4. [ ] If compile succeeds, verify the real Horizon native host loads the library and inspect/archive the overlay. If it fails, inspect the first new diagnostic before changing Core code.


## Engine reference and HNMod integration follow-up — 2026-11-18
1. [x] Make official Minecraft Java 26.3 acquisition automatic for engine push/PR CI, verify the downloaded archive, run existing engine tests/Clippy, and delete the jar in an unconditional cleanup step (run 44331401582 PASS).
2. [ ] Add a real Java 26.3 oracle driver and differential fixtures. The current smoke workflow does not compare engine outputs with the game.
3. [ ] Expand the engine beyond the current coordinate primitive only when each domain has a pinned 26.3 reference fixture and deterministic tests.
4. [x] Verify DreamMod Core's native `.hmod` source check, unit tests and source packaging (run 44322745253 PASS).
5. [ ] Wire the owned Core host collector through a concrete per-mod lifecycle/capability adapter and expose only bounded owned DTOs to the interpreted runtime.
6. [ ] Connect the package/runtime layer to the trusted Core HNMod without duplicating the package DTOs or Horizon host functionality.
7. [ ] Finish API-only overlay native-host loading, archive inspection and artifact upload (run 44330930078 pending).
8. [ ] Full-game launch remains gated on the user's matching licensed Minecraft installation; never bypass Horizon's account/ownership checks or upload Mojang assets.

## Section coordinate and HNMod bridge increment — 2026-11-18
1. [x] Add a dimension-independent `SectionPos` / `SectionAddress` geometric contract using Euclidean division on all axes.
2. [x] Expose section and local-Y coordinates through the HNMod bridge without retaining Horizon handles.
3. [x] Enforce byte-identical canonical engine source and HNMod mirror in Core and overlay CI.
4. [ ] Verify the latest engine formatting/tests/Clippy after correcting the initial formatting and impossible overflow test.
5. [ ] Verify Core formatting, tests and pinned Horizon source packaging after adding the section bridge.
6. [ ] Verify API-only overlay compilation and real native-host loading with the runtime dylib search path configured. The previous host test failed on a missing sibling `libhorizon_mod_runtime.so`; the workflow now sets `LD_LIBRARY_PATH` but the rerun is pending.
7. [ ] Define chunk section storage and block-state representation only after pinning the Java 26.3 reference behavior and test fixtures. Do not infer dimension height or palette serialization from this coordinate contract.


## Verification follow-up — 2026-11-18
1. [x] Verify API-only prebuilt compilation, real Horizon native-host loading, archive inspection and upload (overlay run 44341466372 PASS).
2. [ ] Verify DreamMod Core formatting, unit tests and source packaging after the import-format correction (run 44361109477 pending).
3. [ ] Verify whether the actual distributed launcher configures a native library search path equivalent to the successful CI host test; do not infer full launch readiness from CI's LD_LIBRARY_PATH.
4. [ ] Wire the Core collector through a concrete per-mod lifecycle/capability adapter and interpreted runtime; current Hooks remain empty.
5. [ ] Build production .dmod interpreter/loader/sandbox; none is implemented.


## Verification follow-up — 2026-11-18 (Core pass)
1. [x] Verify Core native/API/host-WASM checks, mirror parity, formatting, unit tests and source packaging (run 44361109477 PASS; 12 tests; artifact 14454406903).
2. [ ] Verify self-contained Linux overlay dependencies with $ORIGIN RUNPATH and no inherited overlay LD_LIBRARY_PATH (run 44361815335 pending).
3. [ ] Verify actual launcher/game startup on a matching licensed installation after the native-host test is self-contained.
4. [ ] Wire Core player collection through a concrete per-mod lifecycle/capability adapter and interpreted runtime; current Hooks remain empty.
5. [ ] Implement the production .dmod interpreter, loader and sandbox; current prototype is not production integration.

## Shared lifecycle-to-Core contract — 2026-11-18
1. [x] Extract the player-read authorization trait and `entities.read` identifier into dependency-light `mods/dreammod-contract`.
2. [x] Make Core's bounded online-player collector consume the shared trait.
3. [x] Implement the shared trait for package `ModLifecycle`, checking Enabled state and explicit capability approval.
4. [ ] Verify package/Core/overlay Actions for this dependency change.
5. [ ] Add a runtime manager that owns one `ModLifecycle` per validated package and supplies the matching lifecycle reference to each Core host operation.
6. [ ] Expose only owned bounded DTOs to the chosen interpreted runtime after per-mod authorization. No interpreter or script binding exists yet.

## Authorization contract verification follow-up — 2026-11-18
1. [x] Re-export the single canonical `entities.read` capability identifier from the shared contract crate.
2. [x] Add a trait-level test for all lifecycle states and explicit-grant behavior.
3. [ ] Confirm package workflow `44362952060` passes.
4. [ ] Confirm Core workflow `44362777239` passes pinned SDK check, formatting and tests.
5. [ ] Confirm overlay workflow `44362776804` passes native host loading without overlay environment overrides.

## ModManager milestone — 2026-11-18
1. [x] Add deterministic manager owning validated packages and per-mod lifecycle state.
2. [x] Require dependencies to be Enabled before enabling dependents.
3. [x] Prevent disabling a dependency while a direct dependent remains Enabled.
4. [x] Cascade failures to transitive dependents in reverse dependency order.
5. [ ] Verify package workflow `44363310681` and PMCS `44363323071`.
6. [ ] Design transactional host registration/unregistration with rollback and resource quotas.
7. [ ] Choose and embed the interpreted language/runtime only after sandbox and execution-budget evaluation.

### ModManager validation note
- The initial package run before `manager` module export did not include manager tests. A cascade-order defect found during review was fixed in `bb8806d`; require a post-fix package run that lists and passes all manager tests before marking the milestone verified.

### Verification status — 2026-11-18
- [x] Package/ModManager: run `44363502581` passed formatting, 41 tests (including manager tests), and Clippy.
- [x] DreamMod Core/shared authorization: run `44362777239` passed pinned Horizon SDK/tooling validation, source/API/host-WASM checks, mirror parity, formatting, tests and source packaging.
- [ ] Prebuilt overlay host-load verification: run `44362776804` remains in API-only compilation; require the real-host load test without overlay environment overrides.

## Authorization callback integration follow-up — 2026-11-18
1. [x] Remove the local Cargo contract dependency rejected by Horizon prebuilt source staging.
2. [x] Make Core player collection accept a one-shot authorization closure checked before host access.
3. [x] Expose `ModManager::require_capability` for per-mod lifecycle+grant checks and add a callback-oriented regression test.
4. [ ] Verify package, Core and overlay Actions after the callback refactor.
5. [ ] Implement the runtime coordinator and invoke Core collection from a real supported Horizon host hook/context.

### Callback hardening verification
- [x] Expose `ModManager::require_capability` as the callback target for Core's closure-based authorization.
- [x] Add lifecycle/capability callback regression coverage.
- [x] Handle already-Failed intermediate dependents during transitive failure propagation.
- [ ] Verify package run after commit `4103668`; recheck Core `44363904945` and overlay `44363904976`.

### Latest CI snapshot — 2026-11-18
- [x] ModManager package verification: run `44364198594` passed rustfmt, 42 tests (including all seven manager tests), and Clippy.
- [x] PMCS validation: run `44364312080` passed.
- [ ] Callback refactor Core verification: run `44363904945` still building `cargo-horizon-mod`.
- [ ] Overlay API-only build and native host loading: run `44363904976` still compiling API-only overlay.

## Asset API milestone — 2026-11-18
1. [x] Expose validated asset bytes through `ModManager::read_asset` without extraction.
2. [x] Enforce Enabled state and explicit `assets.read` capability.
3. [x] Revalidate asset namespace/path and test denied, allowed, traversal and missing cases.
4. [ ] Verify package CI after commit `f90d0b1`.
5. [ ] Add bounded decoding/registration transactions and cache ownership only after interpreter/runtime budgets are specified.

### Asset API verification — 2026-11-18
- [x] Package asset API tests: run `44364618050` passed all 43 tests, rustfmt and Clippy.
- [x] PMCS validation: run `44364715327` passed regression and structure/freshness validation.
- [ ] Core unit tests/package: run `44363904945` has passed pinned checker/API/host-WASM/mirror/format checks; unit tests still in progress.
- [ ] Overlay API-only build and native host loading: run `44363904976` still compiling.

### Core callback verification update
- [x] Core closure-based authorization passed run `44363904945`, including pinned Horizon native/API/host-WASM checks, mirror parity, formatting, tests and source packaging.
- [x] Overlay API-only prebuilt compile and Linux RUNPATH setup passed in `44363904976`.
- [ ] Finish real native host load test without overlay environment overrides, then inspect and upload overlay artifact.

## Entrypoint bridge milestone — 2026-11-18
1. [x] Separate enabled-state checks from named capability checks.
2. [x] Expose borrowed validated entrypoint bytes only after explicit enable.
3. [x] Add lifecycle regression test for before/after activation.
4. [ ] Verify package workflow after `ee903d7`.
5. [ ] Select and sandbox the interpreter; parse/execute source only with deterministic time, memory and instruction budgets.

### Entrypoint bridge CI
- [x] Package run `44365010182` passed 44 tests, formatting and Clippy, including activation-gated entrypoint source access.
- [ ] Complete native host loading in overlay run `44363904976` and archive/upload verification.

### Overlay host-load harness
- [x] Re-export `DREAMMOD_PREBUILT_DIR` in the loader step; add ELF/RUNPATH diagnostics and a 180-second timeout.
- [ ] Verify the new overlay run passes host loading without `LD_LIBRARY_PATH`, then inspect/upload the artifact.


## Resource ownership transaction milestone — 2026-11-18
1. [x] Add strict resource identifier validation and deterministic ownership records.
2. [x] Stage registrations privately and commit atomically; reject duplicates/conflicts without partial mutation.
3. [x] Add drop rollback, deterministic owner cleanup, and per-mod/global quotas.
4. [ ] Verify package workflow run `44385572845` (tests, formatting, Clippy).
5. [ ] Integrate reservations with ModManager lifecycle disable/failure and add rollback tests for external host publication.
6. [ ] Verify the actual Horizon resource registration APIs before implementing the host adapter.

Scope note: current registry is metadata bookkeeping only; no Horizon resource is registered or visible in-game.


### Resource registry CI follow-up
- [x] All 51 unit tests passed on run `44385572845`.
- [x] Rust formatting passed on run `44385572845`.
- [x] Correct strict Clippy findings from that run.
- [ ] Confirm follow-up package run after `d1d89c64aad8` passes tests, formatting and Clippy.


### ModManager resource lifecycle integration
1. [x] Give ModManager ownership of the transactional resource registry.
2. [x] Permit registration only for Enabled mods.
3. [x] Enforce each mod's own resource namespace.
4. [x] Release reservations on disable and transitive failure.
5. [x] Add lifecycle integration tests.
6. [ ] Verify package CI run `44386150155` (format, tests, Clippy).
7. [ ] Verify pinned Horizon SDK resource registration APIs before implementing any external host adapter.

Scope: internal resource ID reservations only; no live Horizon resources are registered.


### Resource lifecycle test follow-up
- [x] Initial integration run `44386150155`: 55 tests, rustfmt and strict Clippy passed.
- [x] Add duplicate-ID batch rollback regression and accurate test naming.
- [ ] Verify package workflow after `51aa347d1140`.
- [ ] Verify latest PMCS validator run after the full five-file update.
