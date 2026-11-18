# Current State

Last updated: 2026-11-18
Status: CORE_SOURCE_PACKAGE_VERIFIED; API_ONLY_PREBUILT_OVERLAY_HARNESS_IN_PROGRESS; DREAMMOD_PACKAGE_INSPECTOR_VERIFIED; RHAI_CANDIDATE_PROVISIONAL
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
- This is a typed contract and policy guard only. DreamMod Core does not yet implement the closure with `ServerHost`, and no script receives snapshots yet. Package CI is pending.
