# Current State

Last updated: 2026-11-18
Status: ENGINE_REFERENCE_SMOKE_AND_UNIT_TESTS_VERIFIED; DREAMMOD_CORE_SOURCE_HMOD_VERIFIED; API_ONLY_PREBUILT_OVERLAY_HOST_LOAD_PENDING; DREAMMOD_PACKAGE_INSPECTOR_VERIFIED; RHAI_CANDIDATE_PROVISIONAL
Repository HEAD: resolve dynamically from `main`; latest verified package CI commit: `4e983c04ea8f3a4085aa2e13b6b53d691c307564`.

## Direction
- Active project: DreamMod, a deeply extensible interpreted modding runtime for Horizon.
- DreamMod Core is built once in trusted CI against a pinned Horizon SDK. The separate launcher distributes the prebuilt engine/native overlay and ordinary .dmod source packages.
- Player-installed .dmod scripts are parsed and interpreted at runtime. No Rust/native compilation, bytecode build or game rebuild occurs on player devices.
- The separate launcher is out of scope in this repository.
- Integrate through Horizon's current .hmod/native API and use mixins only when a real extension gap requires them. Do not copy Horizon source into this repository.

## Pinned baseline
- Horizon Nightly 2026-11-18: 0b14f8acfd97fa2c750e42795249dcc981178ca4
- SDK tag: sdk-0b14f8acfd97fa2c750e42795249dcc981178ca4
- horizon-mixins: e59da9094f0c2fdda6c627fe9eff22dec6a906f0
- mod API: 0.2
- game: 26.3
- Core manifest capability: registries, required by Horizon's current native template/validator. Treat it as a real permission.

## Implemented and verified
- mods/dreammod-core contains a minimal native Hooks bootstrap with horizon-mod.toml, Cargo manifest and src/lib.rs.
- Official pinned cargo-horizon-mod check and source package build passed in Actions run 44285877944.
- Downloaded and inspected the generated .hmod artifact. It contains horizon-mod.toml, Cargo.toml, src/lib.rs and uses.json; the manifest pins the expected Nightly/API/mixins revisions.
- mods/dreammod-runtime-prototype contains an evaluation-only Rhai 1.27.0 AST interpreter candidate. Formatting, behavior tests, Clippy and per-value resource-limit tests passed in runs 44284795812 and 44285379104. A later provider-validation addition needed rustfmt-only changes; run 44289702282 caught the formatting mismatch, fixed in 99be124. Run 44289964423 then caught an ambiguous EvalAltResult-to-Box conversion in the new type-validation path; fixed explicitly with Box::new in 5bdd485. The follow-up runtime workflow is pending.
- Comparable synthetic x86_64 benchmark: 50 x 10,000 item updates took 265.955 ms in Rhai AST vs 574.49 microseconds in Rust (462.94x). This is not Android/ARM64 data; it warns against per-entity/per-vertex script loops.
- Pinned Horizon source confirms the API-only Route::Api implementation builds mod dylibs against signed prebuilt runtime/API slices and reuses the existing client executable.
- The source-package workflow uses a cached workspace build of the pinned cargo-horizon-mod tool.

## Active verification
- The stock horizon-launcher native-build client and server export commands require a prepared Minecraft 26.3 installation. Horizon refuses to prepare it without an authenticated account that owns the game. Do not bypass this check or redistribute Mojang assets.
- To prove the API-only native route without Mojang assets, scripts/ci/patch_horizon_api_route.py adds CI-only tests to an ephemeral checkout of the pinned Horizon source:
  1. call the exact internal prebuilt::compile(..., Route::Api, ...) function with the signed SDK client runtime/API slices;
  2. create the normal native inventory with Horizon's library_record::collect;
  3. call the real horizon-mod-host::native_libraries::load and assert DreamMod Core is loaded.
- Workflow run 44288437986 failed because the CI step changed HOME, hiding the installed Rustup toolchain. Commit 67bc587 preserves HOME and isolates only XDG directories. Run 44288685182 then failed because the cargo package was invoked from the wrong workspace; fixed in 7a7cdb0.
- Prebuilt overlay workflow run 44288685182 failed because cargo was run from the Horizon root workspace, which does not include the separate launcher workspace. Fixed in 7a7cdb0 by running the compiler test from .ci/horizon/launcher and caching both workspaces. Current run 44289897914 is in progress. No overlay artifact or real host-loader success is claimed yet.
- The intended artifact is a client engine/native-library overlay only; it contains no Mojang assets. A complete playable install still needs the user's matching licensed Minecraft 26.3 assets.

## Not implemented / not verified
- No verified prebuilt client overlay or successful native-host load yet.
- No complete game launch with DreamMod Core.
- `mods/dreammod-package` defines and validates the version-1 JSON manifest contract, capability declarations, dependency declarations, and deterministic dependency-first load order.
- inspect_dmod() now parses actual ZIP input in memory, performs metadata preflight and local-header consistency validation, checks the manifest and declared entrypoint, and streams regular entries to EOF for actual-size and CRC validation. Validated script/asset bytes are retained in a bounded Vec<PackageFile>; no filesystem extraction occurs.
- Dedicated package CI passed on commit `4e983c04ea8f3a4085aa2e13b6b53d691c307564`, Actions run `44299317538`: formatting step, all 19 unit tests, and Clippy with `-D warnings` succeeded.
- PMCS session-log validation is legacy-aware: old duplicate headings are preserved, while new entries must be unique against all prior IDs; regression tests passed in Actions runs 44299837834 and 44300145443; latest full PMCS validation is green.
- No filesystem installation, runtime capability enforcement, script execution, or production sandbox exists. The package layer retains validated script/asset bytes in memory, computes a pure explicit capability plan, and provides a default-deny CapabilityGate constructible only from a validated manifest plus explicit approvals. It is a host-side helper, not yet wired into Horizon API calls. Actions run 44300015439 passed on commit 2dd019b4a370fec853ec19849c4538f6cce9be44: formatting, all 19 tests, and Clippy. Capability-planner CI passed in Actions run 44300324687 on commit d83b3491b747bc880b657e4cf17e259a8af2ff62: 21 tests and Clippy.
- No production sandbox, capability enforcement, per-mod isolation, exact heap accounting, cancellation or crash containment.
- The Rhai prototype limits are per-array, per-map and per-string caps plus operation/call-depth/variable/function limits; they are not aggregate per-mod memory quotas.
- Rhai is not the final language choice and is not integrated into Core.
- No stable public DreamMod API, event bus, subsystem provider registry or AI/physics/rendering replacement implementation.
- No Android/ARM64 runtime benchmark or device validation.
- No final decision on an optional prebuilt native-kernel tier; it is not implemented and must not silently replace interpreted .dmod scripts.

## Next actions
1. Verify run 44288685182. Fix any Rust API/test harness or host-loader failures.
2. Inspect the client overlay artifact and its inventory; verify the real Horizon host loader accepts it.
3. Extend the package layer from verified ZIP inspection to a bounded in-memory package-content representation; keep extraction/install and script execution separate until trust and sandbox boundaries are designed.
4. Keep Rhai provisional; expose only coarse-grained/batched host contracts and benchmark realistic workloads on ARM64.
5. Integrate the runtime into Core using documented Horizon hooks (initial lifecycle/tick hooks first); external runtime dependencies may move Core from Route::Api to the registry build route, which must be verified in CI.
6. Define subsystem replacement contracts incrementally; do not claim full physics/rendering replacement until the required Horizon extension points and performance are proven.
7. Keep launcher implementation out of this repository.


## Latest DreamMod runtime hardening (2026-11-18)
- Added validation at the prototype physics provider boundary: equal input lengths, maximum 100,000 items, finite inputs, exact output length, numeric output and finite output.
- Added negative regression tests for mismatched lengths, NaN/infinity, malformed output length and non-finite provider output.
- Commit: 33639ee0d89627b7ad3342d694a2d26c1ff43fd5.
- CI status: pending inspection; implementation is not yet VERIFIED by the workflow.


## Latest package resolver work (2026-11-18)
- Added `resolve_load_order()` to the standalone package contract.
- Validates the installed manifest set, rejects duplicate IDs, missing dependencies and semver mismatches, detects dependency cycles, and produces a deterministic dependency-first order using lexicographic tie-breaking.
- The resolver is a pure planning function: it does not load, execute, grant capabilities or mutate installed packages.
- Commit: 1817a3b4b9a3f4ae974793c2d46c9cbce65b3fe6.
- CI status: not yet inspected; tests are added but not claimed as passing.


## Latest .dmod ZIP reader implementation (2026-11-18)
- Added a real in-memory ZIP inspector in mods/dreammod-package using the exact-pinned zip 9.6.1 crate with only Deflate enabled.
- Rejects oversized compressed input, invalid ZIP structures, invalid UTF-8 names, unsupported compression, encrypted entries, symlinks/unsupported types, unsafe paths, duplicate names, excessive expanded sizes and suspicious compression ratios.
- Calls the ZIP reader validate_header_consistency() before opening members, checking local-header/central-directory consistency, duplicate local records and overlapping entry ranges.
- Reads the manifest first after metadata preflight, validates the declared entrypoint, then streams every remaining regular entry to EOF in bounded chunks so CRC and actual decompressed sizes are checked. No filesystem extraction occurs.
- Added real ZIP fixture tests for valid packages, traversal, truncated/non-ZIP input and payload corruption.
- Commits: 32d52810bb2181c7c6bb03a4f7152b402a8c8d9e, d7e7794567efb0cf1873686964fe47e7364ccbf4, 8bea071635acc45098bd4e93c5c66cbdf5296b30, 15c650a0e61ed3707e6ff76f784180f7f9115452, fc6af40387fa9b861b19ec7fb6b148046c83df8d and 6add01bb156b8ff0e826de40dbd7712a014daf41.
- CI status: not yet verified. The implementation is committed but must pass the dedicated workflow before it is treated as verified.
- Limitations: package inspection is in-memory and bounded; it does not install files, grant capabilities, execute scripts, or establish a production sandbox.


## Latest in-memory package-content representation (2026-11-18)
- Extended ValidatedPackage with files: Vec<PackageFile>, retaining validated script and asset bytes after full streaming/CRC checks.
- manifest.json remains parsed metadata and is not duplicated in files; directories are not retained.
- Retention remains bounded by the existing 64 MiB aggregate uncompressed package limit and 16 MiB per-file limit. No disk extraction or installation occurs.
- Added a fixture assertion for exact script and asset bytes. Package CI is pending for this extension.


## Latest explicit capability planning (2026-11-18)
- Added plan_capability_grants(manifest, approved), a pure deterministic planner.
- Empty approvals grant nothing; requested-but-unapproved capabilities remain denied; unknown, unrequested and duplicate approvals are rejected.
- The plan is data only. The runtime host must enforce every grant at each API boundary; no capability is actually activated by this crate.
- Regression tests cover default-deny, partial grants and invalid approvals. Package CI run 44300324687 passed all 21 tests and Clippy.


## Latest host capability gate helper (2026-11-18)
- Added CapabilityGate::from_approvals(), allows() and require().
- The gate stores a private granted set, validates approvals through the manifest planner, and denies unapproved or unknown capabilities.
- This is a reusable guard primitive only. DreamMod Core does not yet wrap Horizon host APIs with it, so end-to-end enforcement is not claimed.
- Added gate regression tests. Package Actions run 44300879584 passed all 22 tests and Clippy.


## Latest explicit mod lifecycle policy (2026-11-18)
- Added `mods/dreammod-package/src/lifecycle.rs` and exported it from the package crate.
- A validated package starts in `AwaitingApproval`; explicit capability approvals move it only to `Ready`; a separate explicit `enable()` is required before privileged calls can pass.
- `disable()` blocks all capability checks. `fail()` discards the gate and makes the instance terminal/failed.
- Added regression tests for default-disabled behavior, approval/activation separation, capability denial, disable, fail-closed behavior, and invalid transitions.
- Commits: `14a4bf8`, `6f1adf2`, `eae8f1c`.
- Verification: pending the package workflow triggered by the latest source changes. This is a pure lifecycle policy helper, not a running script scheduler or Horizon integration.


## Latest guarded host boundary (2026-11-18)
- Added a generic `HostAdapter::invoke(capability, closure)` to the lifecycle module. It checks that the mod is Enabled and has the requested capability before invoking the supplied host operation; denied calls do not invoke the closure.
- Added a 512-byte, control-character-free failure diagnostic limit.
- Added tests for callback non-invocation on denial, successful authorized invocation, capability mismatch, and bounded failure reasons.
- Commits: `7107022` and compile-structure correction `407a762`.
- Verification: package CI pending. This is a reusable authorization boundary, not yet wired to concrete Horizon API functions. It does not make arbitrary host handles safe to expose.


## Latest host-boundary hardening and Horizon API inspection (2026-11-18)
- Extended `HostAdapter` regression coverage: a callback is not invoked when the mod is not enabled, lacks the requested capability, is Disabled, or is Failed.
- Corrected the failure-diagnostic test to use an actual newline control character; failure messages remain capped at 512 UTF-8 bytes and reject control characters.
- Package workflow run 44304258172 passed on source commit `d9a524f6ff0b4ef6124abf2c0fdd8aaebdc19747`: Rust formatting, all package tests and Clippy `-D warnings` succeeded.
- Inspected the exact pinned Horizon API at `0b14f8acfd97fa2c750e42795249dcc981178ca4`: `Hooks`, `ServerHost`/`ClientHost`, `Wire`, explicit load phases and `server_tick` phase semantics are present. Hook instances are `Send + Sync`; callbacks can run across threads and re-enter through host calls.
- Added `docs/dreammod/HORIZON_HOST_BRIDGE.md` documenting the verified API surface and rules for owned values, phase/side checks, per-operation capability checks and batched calls.
- The generic HostAdapter is still not connected to concrete Horizon APIs; no script runtime, sandbox, scheduler or end-to-end .dmod execution exists.
- Latest relevant commits: source `d9a524f`; host bridge doc `d016ea5`; roadmap `16c1898`; session log `2c87f41`.




## Owned player snapshot bridge contract (2026-11-18)
- Added `mods/dreammod-package/src/bridge.rs` with an owned `PlayerSnapshot` containing UUID, a bounded Minecraft player name, a validated dimension identifier and finite bounded position coordinates.
- The DTO stores only owned values; no Horizon handle, reference or borrowed string can cross this boundary.
- Added `HostAdapter::read_player_snapshot()`. It checks `entities.read` and the enabled lifecycle state before invoking the host-read closure, then returns only a validated DTO.
- Tests cover valid DTO creation, malformed names/dimensions, NaN/out-of-range positions, denied callback non-invocation and invalid snapshot rejection.
- This is a typed contract and policy guard only. DreamMod Core does not yet implement the closure with `ServerHost`, and no script receives snapshots yet. Package Actions run 44309091208 passed formatting, all package tests and Clippy on commit `7bae3eaad6128ff815e657541dd3cd1880dca485`.


## Bounded player batch contract (2026-11-18)
- Added `PlayerSnapshotBatch` with a hard cap of 256 snapshots and duplicate UUID rejection.
- Added `HostAdapter::read_player_snapshots()` so one `entities.read` authorization check protects one bounded batch callback, avoiding per-player interpreter/native crossings.
- Tests cover the batch limit, duplicate identities and callback non-invocation when authorization fails. Batch package Actions run 44309674771 passed formatting, all package tests and Clippy on commit `4397de1a28d536b0ffca448b553478ecae3467fb`.
- No concrete Horizon host collector or script-visible batch API exists yet.


## First concrete Core-side player collector (2026-11-18)
- Added `mods/dreammod-core/src/player_snapshot.rs` and exported it from Core.
- The collector calls the pinned Horizon `ServerHost::players().server_players()`, checks the 256-player cap before reading individual players, then copies UUID, name, dimension and entity position into owned DreamMod values.
- It rejects missing/stale reads, malformed names/dimension identifiers, non-finite/out-of-range coordinates and duplicate UUIDs. The returned batch preserves Horizon connection order.
- It stores no Horizon handle or borrowed host reference in the result. It is a public Core-side collector function, not yet called by a script runtime.
- Source commits: `7ecd981babec69a5081aecacd25f7b36629f71ea`, `b2362c6a1347390f4f330655fb0823e8563d94d8`.
- Verification: pinned SDK Core workflow run 44310547586 is building cargo-horizon-mod; host/WASM check and package steps have not run yet. PMCS run 44310547866 failed because required memory files were not updated in the same change; this entry and companion updates address that failure.


## Core player-read authorization seam (2026-11-18)
- The Core player snapshot collector now requires a `PlayerReadAuthorization` implementation before it touches `ServerHost`.
- Added a helper and regression tests proving a denied permission check prevents the host callback, while an authorized check invokes it once.
- Source commit: `99929e38d5f1a815e8b17cb069a5eff8dfcc8e3e`.
- Core SDK check and API-only overlay workflows are running for this source commit. Do not claim this new change verified until they finish.
- The authorization trait is not yet implemented by a per-mod `ModLifecycle` adapter. The interpreter/runtime still needs to supply that adapter; no script binding or sandbox exists.


## Core snapshot regression coverage (2026-11-18)
- Added Core-local unit tests for player name/dimension/position validation, duplicate UUID rejection and the 256-player batch limit.
- Source commit: `ebe09145849063ade0e01f18e55a23bec18cd4e4`.
- Core workflow must be checked; the existing Horizon checker validates host/WASM compilation, but whether it executes `#[cfg(test)]` tests is not assumed.


## Gated collector source correction (2026-11-18)
- Fixed a duplicate-function/source-structure regression in the collector refactor. The unchecked Horizon read is now private and called only after the public authorization gate succeeds.
- Corrected source commit: `eadd1aa545851d487da4e2ff2f37edfbec327dfb`. New Core Actions must verify this commit; earlier in-flight runs target the intermediate source.


## Core test CI
- Added explicit formatting and `cargo test` steps to the Core workflow after `cargo-horizon-mod check` has fetched the pinned SDK.
- Workflow commit: `ffe93f053ab6a0a0df99953a1206262661e8b111`.
- This is not yet verified; check the first workflow run for dependency-path/test-harness issues.


## API optimization and load deadline (2026-11-18)
- Optimized duplicate UUID validation in Core and package PlayerSnapshotBatch: replace BTreeSet node insertions with one contiguous UUID vector plus sort_unstable/adjacent duplicate detection. The batch cap remains 256 and connection order remains unchanged.
- Core commit: d650c58a9ee9b08a365cbe4f830a16e8510daaec. Package bridge commit: 5e8e58a77515b1f8ec6c5aa0511005c403b44dd5.
- Added a cooperative 60-second monotonic deadline to .dmod ZIP inspection. inspect_dmod() uses the default budget; inspect_dmod_with_deadline() lets a future loader share one deadline across validation, script parsing and registration. Archive reader checks between entries and decompression chunks. Commit: b689551994c45b603fcab8f08c5708973c9f2c3b. CI pending.
- Fixed Core workflow to install rustfmt (commit e47e8e0f858c9c9d09088db17d23a74584ae420a). Run 44313213891 passed Horizon source/API/host-WASM checks but stopped at formatting; unit tests were skipped.
- This is not yet a complete runtime load timeout: future loader must pass the same deadline through every stage and poll it during long-running parsing/initialization. No async cancellation or script runtime exists yet.
- No performance benchmark has been run; this is a targeted allocation reduction, not proof of FPS improvement.


## Latest snapshot optimization (2026-11-18)
- Replaced the temporary UUID Vec with a fixed [u128; 256] stack buffer in both Core and package PlayerSnapshotBatch validation. This removes the heap allocation for duplicate detection; original snapshot order and 256-item cap remain unchanged.
- Core commit: b2185e8917e351dfd0408231e855efe47e6b3bfd. Package bridge commit: f3a55af3a332073db92df24de00e347bc55a3bb8.
- Package deadline implementation CI passed in run 44317125357. The latest stack-buffer package/Core runs are pending.
- Core formatting now targets only the DreamMod Core manifest, avoiding formatting generated Horizon SDK source under .horizon/sdk. Fresh Core run pending.


## Shared batch storage (2026-11-18)
- Core and package PlayerSnapshotBatch now store players as Arc<[PlayerSnapshot]>. Batch clones share immutable backing storage instead of deep-copying every player record.
- Core commit: 3f629cc54a27992f9a7f402dfda3fcfa0505b4bf. Package commit: 74310a797393ec319fb9192d931a7b860ba55ced.
- Added pointer-identity tests proving clones share the same Arc allocation. Package run 44318172781 and Core run 44318166133 are pending; previous Core run 44317880298 is still running against the earlier stack-buffer version.
- Runtime dispatcher does not yet reuse snapshots. Capability checks must remain per mod before a shared snapshot is returned.


## Verification refresh — 2026-11-18 (HEAD 741f7ff1)
- DreamMod Core workflow run 44322745253 passed on HEAD 741f7ff15bb22a2b16b8a5a667fe471d3ee3ddc9. The pinned Horizon native/API/host-WASM check, Core formatting, Core unit tests and native source packaging all passed.
- The Core test fixture type mismatch was fixed in commit 741f7ff15bb22a2b16b8a5a667fe471d3ee3ddc9; this is now covered by the successful run above.
- Package workflow run 44318172781 passed on commit 74310a797393ec319fb9192d931a7b860ba55ced, including the Arc-backed immutable batch tests. The archive inspection deadline package CI had already passed in run 44317125357.
- PMCS run 44322745310 failed because five required memory documents were not updated alongside the latest Core source/test changes. Its six validator regression tests passed; companion docs are being refreshed in this change set.
- API-only overlay run 44322745328 failed before the real host-loader step. The CI-only test imported the public picker::Route (used for UI fit status) instead of the private prebuilt::Route required by sdk_cache::slices and prebuilt::compile. The pinned upstream source confirms the correct type is prebuilt::Route::Api; the test patch is corrected accordingly. No overlay artifact or host-loader success is claimed.
- The Core authorization trait remains an integration seam; it is not yet backed by a concrete per-mod lifecycle adapter. There is still no production interpreter, .dmod loader, sandbox or dispatcher.
- Arc-backed cloning and stack-buffer duplicate detection are implemented and regression-tested; no performance benchmark or FPS claim exists.


## Prebuilt compiler compatibility investigation — 2026-11-18
- Overlay run 44324363090 passed CI test injection and compiled the pinned Horizon launcher, then failed inside Horizon's API-only prebuilt build. The diagnostic showed SDK crate mod_api had been compiled by an incompatible rustc; the compiler in our overlay workflow was 1.95.0.
- Inspected the exact pinned Horizon source at 0b14f8acfd97fa2c750e42795249dcc981178ca4: tools/ci/rust-version is 1.99.1, which is the compiler used to produce the signed SDK. Updated .github/workflows/dreammod-prebuilt.yml to use 1.99.1 and to record that exact compiler in overlay metadata (commit 96d2e74071afb75f5657b0c2fe88caf2e529adc7).
- Overlay rerun 44328650675 is in progress. It has not yet reached the prebuilt compile step; no result is claimed for the rustc mismatch or the secondary generated crate::hooks error.
- PMCS run 44328650216 passed all validator regression tests and freshness checks on commit 96d2e740.


## Engine reference smoke verification — 2026-11-18
- Added push/PR-triggered workflow `.github/workflows/reference-artifact-smoke.yml`: it runs offline downloader security tests, downloads official Minecraft Java 26.3 into a unique `RUNNER_TEMP` directory, verifies the bundled server ZIP/CRCs, runs workspace formatting/tests/Clippy, and deletes the temporary directory with `if: always()`.
- First workflow revision failed before job creation because `runner.temp` was referenced in job-level environment. Corrected it to step-level environment; successful run: 44331401582 on commit `bada0f033ca31a8fd6ad4f46dcd58a907227bd8f`.
- Run 44331401582 downloaded 62,294,556 bytes; official metadata SHA-1 verified as `33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`. ZIP CRC inspection passed. Engine workspace tests passed: 5 coordinate tests + 5 testkit contract tests; formatting and Clippy passed; cleanup confirmed the temporary directory was absent.
- This is artifact-integrity smoke plus existing engine unit tests, NOT a differential comparison against Minecraft behavior. The engine still implements only horizontal block-to-chunk mapping; a real Java 26.3 oracle/harness and captured vanilla fixtures remain open.
- Existing native Core workflow run 44322745253 passed the pinned Horizon SDK check, Core tests and source `.hmod` packaging. The Core `Hooks` implementation remains empty; player snapshot collection is not yet connected to the DreamMod runtime or script API.
- Prebuilt overlay run 44328650675 completed API-only compilation successfully with Rust 1.99.1, but was cancelled during the real native-host loader step when a newer commit arrived. A clean rerun, 44330930078, is building the overlay; host loading, archive inspection and artifact upload remain unverified.
- Overlay metadata compiler field was corrected to 1.99.1 in commit `d23bc48fd1e606ef2191e0b4f059ec50451c14b9`. PMCS validation passed in run 44331401564.

## Three-dimensional section address bridge — 2026-11-18
- Added `SectionPos` and `SectionAddress` to the canonical engine coordinate module. `BlockPos::section_address()` maps all three axes with Euclidean division and exposes local coordinates in the 0..16 range.
- Added checked reconstruction to `BlockPos`; the address fields remain private and are produced from valid block coordinates. The API is deliberately geometric and does not specify serialized section layout, palette format, dimension min/max Y, or block-state IDs.
- DreamMod Core exposes section coordinates and local Y through `engine_bridge::locate_block`. The HNMod copy is byte-identical to the engine source and is enforced by `check_engine_bridge_sync.py` in both workflows.
- A first test run failed rustfmt and included an impossible overflow expectation for an address created from an `i32` block coordinate. That test was removed; canonical and mirrored files were formatted consistently. Latest engine and Core runs are pending.
- Overlay run 44335372878 proved the API-only compile but failed in the real host loader because a sibling runtime dylib was not on the Linux loader path. Workflow run 44341223178 adds the overlay folder to `LD_LIBRARY_PATH`; verify it before claiming a successful load.


## Verification refresh — 2026-11-18 (commit 42d9ac7)
- API-only overlay run 44341466372: PASS for signed-SDK API-only compilation, real Horizon native-host loading, archive inspection and artifact upload. This is a native-host integration test, not a full launcher startup or licensed game launch.
- Core run 44341466411: Horizon native/API/host-WASM validation and canonical/HNMod source parity passed; formatting failed only on import wrapping in engine_bridge.rs. Corrected in 42d9ac762502ad5ef3db9158a9885c99a24ae238. Core rerun 44361109477 is in progress.
- PMCS run 44341597122 passed. PMCS run 44361109707 failed because the prior code change was not accompanied by all five required memory documents in its compared change range; the current update records the verified results and formatting correction.
- No full launcher/game startup, production .dmod interpreter, runtime-to-Core lifecycle adapter, or live mod execution is verified.


## Verification refresh — 2026-11-18 (Core run 44361109477)
- DreamMod Core: PASS. Pinned Horizon native/API/host-WASM checks, canonical engine/HNMod mirror parity, rustfmt, 12 unit tests, source .hmod bundle packaging and upload all passed. Artifact ID 14454406903 (9,939 bytes).
- The section bridge is verified in Core CI: negative coordinates and X/Y/Z section addressing are tested. This does not implement chunk-section storage, world simulation, block-state serialization or Minecraft Java 26.3 parity.
- Overlay artifact inspection found that libmod_vocab.so depends on libhorizon_mod_runtime.so but lacks its own RUNPATH. The launcher does not explicitly set LD_LIBRARY_PATH on Linux. Workflow commit 4d2c910c adds $ORIGIN RUNPATH to bundled .so files, refreshes the receipt hash and runs the native-host test without an inherited overlay LD_LIBRARY_PATH. Run 44361815335 is pending.
- No production .dmod interpreter, complete loader/sandbox, lifecycle-to-Core adapter or full launcher/game startup is verified.

## Shared per-mod authorization contract — 2026-11-18
- Added `mods/dreammod-contract` with a dependency-free `PlayerReadAuthorization` trait and canonical `entities.read` constant.
- DreamMod Core's bounded player collector now consumes that shared trait. `dreammod-package::ModLifecycle` implements it by delegating to its lifecycle+capability check.
- Core does not depend on the archive inspector or its ZIP/JSON/semver dependencies. This is a typed integration seam; no runtime instance manager, interpreter, scheduler or script binding exists yet.
- Actions runs for Core `44362777239`, package `44362757352`, overlay `44362776804`, and PMCS `44362787017` were started; results must be checked before marking verified.

## Authorization regression coverage — 2026-11-18
- The package bridge re-exports the shared `entities.read` constant from `dreammod-contract`.
- Added a direct trait-level lifecycle regression test covering awaiting approval, no grant, ready-but-not-enabled, enabled with grant, disabled and failed states.
- Package run `44362952060` is queued for this test; Core run `44362777239` is still building the pinned Horizon checker. Overlay run `44362776804` remains queued. Results pending.

## ModManager milestone — 2026-11-18
- Added and exported `dreammod_package::manager::ModManager`.
- It owns validated package contents plus one lifecycle per mod; validates dependencies before publishing the manager; exposes deterministic load order and read-only package/lifecycle access.
- It enforces dependency-before-dependent enable, prevents disabling an enabled dependency, and propagates failures transitively to dependents.
- New tests cover these invariants. Package CI run `44363310681` and PMCS run `44363323071` are pending; do not mark verified until completed.
- No `.dmod` interpreter or script execution, scheduler, registration transaction, or concrete Horizon runtime binding exists.

### ModManager cascade correction
- Failure propagation now walks dependency-first load order (root dependency before dependent) to correctly cascade through multi-level dependency chains. The original reverse traversal could skip a grandchild in the same pass.
- Run `44363310681` was against the pre-export module state and its 36 passing tests did not include `manager.rs`; the corrected manager must be verified in the new package workflow after commit `bb8806d`.

## Verified ModManager and shared Core authorization — 2026-11-18
- Package run `44363502581` on the corrected manager implementation passed formatting, all 41 unit tests (including all five `manager::tests`) and Clippy with warnings denied.
- Core run `44362777239` passed the pinned Horizon `cargo-horizon-mod` build, native manifest/source/API/host-WASM checks, engine mirror parity, formatting, Core tests, source packaging and artifact upload.
- Overlay run `44362776804` remains in progress at API-only prebuilt client compilation; its real-host no-overlay-env test has not run yet.
- PMCS run `44363588460` is queued/pending at the time of this update; await its final result.

## Packaging-compatible authorization callback — 2026-11-18
- The external `dreammod-contract` path dependency was removed after the real Horizon overlay build failed because the SDK staging process does not include arbitrary local Cargo dependencies (`unresolved import dreammod_contract`, run `44362776804`).
- Core player collection now takes an authorization closure and checks it before touching `ServerHost`.
- `ModManager::require_capability(mod_id, capability)` exposes the exact per-mod lifecycle+grant check; it can be passed as Core's closure. This keeps HNMod Core free of non-SDK dependencies.
- A runtime coordinator and live Horizon hook call site are still missing. New package/Core/overlay CI runs are pending.

### Callback and failure-cascade hardening
- `ModManager::require_capability` is now the package-side authorization closure target for Core player collection; tests exercise default-deny, explicit grant, activation and capability denial.
- Added coverage for a dependency failure when an intermediate dependent was already Failed; the manager avoids trying to transition Failed → Failed and still propagates to descendants.
- Latest package run `44364113419` passed its checks, but predates the final extra cascade regression; run after commit `4103668` pending. Core run `44363904945` and overlay run `44363904976` are still in progress.
