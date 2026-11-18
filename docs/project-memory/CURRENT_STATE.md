# Current State

Last updated: 2026-11-18
Status: CORE_SOURCE_PACKAGE_VERIFIED; API_ONLY_PREBUILT_OVERLAY_HARNESS_IN_PROGRESS; DREAMMOD_MANIFEST_CONTRACT_CI_PENDING; RHAI_CANDIDATE_PROVISIONAL
Repository HEAD at last inspection: 67bc5874fcbb8157d9e4f9fb51a919f6aeb81a9a

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
- `mods/dreammod-package` now defines and validates the version-1 JSON manifest contract (strict fields, semantic versions/API range, safe script entrypoint, capability allowlist, dependency declarations and size/count limits); its new CI workflow has not yet been inspected.
- No ZIP/archive loader, dependency resolver, runtime lifecycle or hot reload.
- No production sandbox, capability enforcement, per-mod isolation, exact heap accounting, cancellation or crash containment.
- The Rhai prototype limits are per-array, per-map and per-string caps plus operation/call-depth/variable/function limits; they are not aggregate per-mod memory quotas.
- Rhai is not the final language choice and is not integrated into Core.
- No stable public DreamMod API, event bus, subsystem provider registry or AI/physics/rendering replacement implementation.
- No Android/ARM64 runtime benchmark or device validation.
- No final decision on an optional prebuilt native-kernel tier; it is not implemented and must not silently replace interpreted .dmod scripts.

## Next actions
1. Verify run 44288685182. Fix any Rust API/test harness or host-loader failures.
2. Inspect the client overlay artifact and its inventory; verify the real Horizon host loader accepts it.
3. Extend the standalone package contract with hostile-archive validation (without extracting files), then dependency resolution and lifecycle; keep interpreter integration behind a pluggable interface.
4. Keep Rhai provisional; expose only coarse-grained/batched host contracts and benchmark realistic workloads on ARM64.
5. Integrate the runtime into Core using documented Horizon hooks (initial lifecycle/tick hooks first); external runtime dependencies may move Core from Route::Api to the registry build route, which must be verified in CI.
6. Define subsystem replacement contracts incrementally; do not claim full physics/rendering replacement until the required Horizon extension points and performance are proven.
7. Keep launcher implementation out of this repository.


## Latest DreamMod runtime hardening (2026-11-18)
- Added validation at the prototype physics provider boundary: equal input lengths, maximum 100,000 items, finite inputs, exact output length, numeric output and finite output.
- Added negative regression tests for mismatched lengths, NaN/infinity, malformed output length and non-finite provider output.
- Commit: 33639ee0d89627b7ad3342d694a2d26c1ff43fd5.
- CI status: pending inspection; implementation is not yet VERIFIED by the workflow.
