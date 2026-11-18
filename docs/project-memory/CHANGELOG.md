## 2026-11-18 — Fix Rhai provider error conversion after CI type check
- The runtime workflow 44289964423 passed formatting but found ambiguous conversion from EvalAltResult into Box<EvalAltResult> in the new provider output type check.
- Replaced the ambiguous .into() with an explicit Box::new(EvalAltResult::ErrorMismatchDataType(...)) in commit 5bdd485. Follow-up runtime workflow pending.

## 2026-11-18 — Correct prebuilt test workspace selection and runtime formatting
- API-only overlay run 44288685182 failed because Horizon's launcher is a separate Cargo workspace; `-p horizon-launcher` must run from `.ci/horizon/launcher`. Fixed in workflow commit 7a7cdb0 and expanded the Rust cache to cover both workspaces. Run 44289897914 is pending.
- Rhai provider batch validation had rustfmt differences; run 44289702282 caught them. Formatting corrected in 99be124; run 44289964423 is pending.

## 2026-11-18 — Verify the Core source package and correct the prebuilt harness status
- Official pinned cargo-horizon-mod check and .hmod source package workflow 44285877944 passed; the artifact was downloaded and inspected for manifest, source and uses.json.
- The stock native-build CLI client/server paths require a prepared, licensed Minecraft 26.3 installation. No ownership check is bypassed and no Mojang assets are included in DreamMod artifacts.
- The current CI-only harness calls Horizon's exact prebuilt::compile(..., Route::Api, ...) with signed SDK client runtime/API slices, then tests the real host loader. Run 44288437986 exposed that overriding HOME hides Rustup; fixed in 67bc587. Run 44288685182 is pending.
- Corrected Rhai limit descriptions: arrays/maps/strings are capped per value, not by aggregate per-mod heap accounting.

## 2026-11-18 — Replace blocked launcher CLI build with signed SDK API-route harness
- Confirmed that both client-instance and standalone server-export `horizon-launcher native-build` commands require a prepared Minecraft 26.3 installation; the official launcher refuses to prepare without an authenticated owning account. We will not bypass this check.
- Added `scripts/ci/patch_horizon_api_route.py`, which appends test-only harnesses to an ephemeral pinned Horizon checkout: one calls the exact internal `prebuilt::compile(..., Route::Api, ...)` with Horizon's signed SDK client runtime/API slices and writes the standard `native-mods.json`; the other calls the real `horizon-mod-host::native_libraries::load` against that overlay.
- Replaced the prebuilt workflow with this direct signed-SDK API route. It builds only the mod dylib and reuses the SDK's prebuilt `horizon-client`; it does not download or redistribute Mojang assets. The resulting overlay is intended to be installed beside a matching licensed base game. Workflow run 44288437986 is pending.
- Official Core source package workflow 44285877944 passed and uploaded the `.hmod` source package.

## 2026-11-18 — Switch account-free prebuilt proof to server export
- Horizon's official client `prepare` step refuses to fetch Minecraft 26.3 unless the owner is authenticated; a client native-build also refuses an unprepared instance. We will not bypass this ownership check or redistribute Mojang files.
- Changed the CI proof to Horizon's official `native-build --server-output ... --version nightly-2026-11-18-0b14f8a` path, which exports a standalone headless engine and native library inventory without selecting a client instance. This verifies the same API-only SDK route; it is not yet a client distribution or clean game launch.
- Renamed the artifact to `dreammod-horizon-nightly-native-server` for accuracy.

## 2026-11-18 — Set the initial script/provider performance boundary
- Recorded the 462.94× synthetic x86_64 Rhai-vs-Rust batch-loop gap as a design constraint.
- Initial script providers must be coarse-grained; Rust owns per-entity/per-vertex hot loops and batch storage. Arbitrary interpreted loops in physics/rendering are not promised.
- Added an explicit future decision point for an optional CI-prebuilt native kernel tier, which would still avoid player-side compilation but is not implemented and is separate from interpreted `.dmod` scripts.

## 2026-11-18 — Add CI workflow for official prebuilt Horizon distribution
- Added `.github/workflows/dreammod-prebuilt.yml` to download and checksum-verify pinned Horizon Launcher 0.2.78, install the pinned Nightly, create a clean instance, run `native-build` on DreamMod Core with trusted code, verify `native-mods.json` and the adjacent native library, and upload the native build overlay.
- First run 44286113386 exposed a checksum filename mismatch; corrected in 01c696c. Run 44286212788 then exposed the missing `libxkbcommon-x11.so.0` dependency; added the headless X11/XKB libraries and offscreen Qt setting in 6bd45dd. Fresh run pending.
- The build workflow verifies the manifest's pinned Horizon commit before uploading, preventing a silently newer Nightly from being mislabeled as the pinned baseline.

## 2026-11-18 — Fix native manifest validation and cache Horizon tooling
- The first official `cargo horizon-mod check` run 44284751206 built the pinned tool successfully, then rejected the manifest because Horizon's current native template requires `capabilities = ["registries"]` even for native API mods. Added that required field in 9a5f9b5 and documented it as a real trust permission.
- Changed the Core workflow from `cargo install` to a cached workspace `cargo build --release` for the pinned `cargo-horizon-mod` tool, avoiding a full uncached 5-minute install on every run.
- Fresh official package check/build run is pending.

## 2026-11-18 — Add resource limits to the Rhai candidate harness
- Extended `RuntimeLimits` with maximum aggregate array items, map entries, string bytes, variables and functions, in addition to operation and call-depth budgets.
- Added an array-growth regression test that must stop when the configured aggregate array budget is exceeded.
- These are prototype-level Rhai safeguards only; production per-mod memory accounting, capability enforcement and fault isolation remain unimplemented. Runtime CI run 44285379104 is pending.

## 2026-11-18 — Record comparable Rhai AST benchmark
- Corrected benchmark run 44284795812 passed: 10,000 items × 50 rounds, Rhai AST interpreter 265.955 ms, Rust native 574.49 µs, ratio 462.94×.
- Both sides now clone their inputs and materialize/checksum their outputs, preventing the prior Rust-side work-elision bug.
- This synthetic x86_64 CI result is not an Android benchmark. It indicates that arbitrary per-item script loops are unsuitable for hot physics/rendering paths without a very coarse/batched architecture; no final language selection is made.

## 2026-11-18 — Correct interpreter shortlist against the no-bytecode requirement
- Rejected Starlark as a candidate for the strict product requirement: its public API parses source into AST, but its evaluator compiles that AST to bytecode before execution.
- Added Sprite as a research-only reference: it uses a tree-walking AST interpreter with no bytecode/VM, but its current language/value surface is too limited for DreamMod to adopt directly.
- Rhai remains the practical feature-rich AST-walking candidate, but its performance must be judged on representative workloads and ARM64 devices; no final language selection is made.

## 2026-11-18 — Add real Horizon native trait package metadata
- Corrected DreamMod Core from a bare Cargo Hooks crate into a native trait mod source package with `horizon-mod.toml`, API 0.2, no extra capabilities and exact Horizon/horizon-mixins pins.
- Moved the Core pin to the published Nightly 2026-11-18 build `0b14f8acfd97fa2c750e42795249dcc981178ca4` and verified the current pinned source still contains `Route::Api` and the native library host.
- Replaced the simple direct `cargo check` workflow with the official pinned `cargo-horizon-mod check` + source bundle build + artifact upload pipeline. The corrected workflow run 44284751206 is pending.
- Corrected YAML quoting after the first workflow revision was rejected before job creation.

## 2026-11-18 — Finish formatting correction for benchmark guard
- Applied rustfmt's required line wrapping to the corrected benchmark in commit 00a3360; fresh workflow run is pending.

## 2026-11-18 — Verify Rhai prototype tests and correct benchmark methodology
- Runtime Actions run 44283982705 passed formatting, all 4 behavior/budget tests and Clippy.
- Reviewed its benchmark output and rejected the apparent 1,070,530x ratio: the Rust-native workload was optimized away (371 ns for 500,000 outputs), making the comparison invalid.
- Added black_box guards around inputs and outputs and corrected the benchmark script's array length call in commit 89ed3a5. A fresh benchmark is pending; no device performance claim is made.

## 2026-11-18 — Fix Rhai host boundary and CI formatting
- Corrected the prototype host boundary to convert Rust vectors to Rhai arrays before script calls, and convert script results back to Rust vectors.
- Corrected the script's array length call to Rhai's `len()` method.
- CI run 44283921499 stopped at rustfmt due solely to import ordering; corrected that in commit 92c3508. The earlier behavior run 44278072893 had two failures caused by the vector/array mismatch; the corrected behavior has not yet completed a passing CI run.
- No final scripting language decision; no production runtime integration.

## 2026-11-18 — Verify Horizon API-only prebuilt route and harden CI
- Traced pinned Horizon launcher Route::Api through prebuilt.rs and api_route.rs: API-only mods are built as separate dylibs against prebuilt SDK runtime/API slices; the existing client executable is returned unchanged. The host loads adjacent libraries from native-mods.json after SHA-256 and game/Rust identity checks.
- Confirmed the current Hooks-only DreamMod Core shape qualifies for Route::Api; CI production of the versioned native build and clean startup remain unverified.
- Fixed the Rhai prototype's feature-gated optimizer import and added rustfmt/clippy components to its workflow. The corrected run is pending.
- Fixed PMCS append-only validation for appending to the historical session log that lacked a final newline; regression suite run 44277791784 passed.

## 2026-11-18 — Add interpreted runtime candidate prototype
- Added mods/dreammod-runtime-prototype using Rhai 1.27.0 as an evaluation-only AST interpreter candidate. The optional Grain bytecode feature and AST optimization are disabled.
- Added tests for event-style dispatch, coarse batch host queries, script-side batched physics calculation and execution-operation limits.
- Added a desktop microbenchmark comparing one batched interpreted algorithm with its native Rust equivalent; CI-host timings are diagnostic only, not device guarantees.
- Added .github/workflows/dreammod-runtime-prototype.yml for tests and benchmark execution.
- Verified the existing DreamMod Core pinned Horizon API check: Actions run 44249522292 succeeded. Runtime prototype workflow result is pending.
- Rhai is not selected as the final language. No production .dmod loader, sandbox, Core integration or prebuilt game distribution is claimed.

## 2026-11-17 — Verify offline reference tooling checks
- GitHub Actions run 44162815461 passed on commit `965287f36d9d8855e9f9f2f8e715c3e61d4ed58e`: all nine Python unit tests passed and the unconditional cleanup step confirmed the temporary directory was absent.
- The artifact smoke workflow runs offline tests on matching pushes and only downloads the Mojang server jar on manual dispatch. The live download and deliberate-failure cleanup paths still require manual dispatch.

## 2026-11-17 — Implement official Java 26.3 artifact acquisition
- Added a standard-library Python resolver/downloader that pins release 26.3 through Mojang's official v2 manifest and version JSON, verifies metadata SHA-1, restricts official HTTPS hosts and redirects, and checks exact server jar size/SHA-1 while streaming.
- Added offline unit tests for official URL policy and artifact verification.
- Added an automatic offline tooling-test workflow and a manual artifact-only Actions workflow. The latter has unconditional temporary-directory cleanup and an optional deliberate-failure input; it does not execute an oracle or claim parity.
- No Minecraft binary was committed, cached, uploaded, or released.

## 2026-11-17 — Add version-pinned reference fixture contract
- Added `rusty-survival-testkit` as a separate workspace crate with a typed JSON fixture model and validation for schema version, Java 26.3 target, subsystem, artifact metadata, scenario, normalization and capture provenance.
- Added five synthetic self-tests and one explicitly synthetic fixture example; these validate the harness contract only and do not count as Minecraft parity.
- Kept the engine runtime crate free of testkit dependencies. Official artifact download, Java oracle, comparator and reference workflow remain unimplemented.

## 2026-11-17 — Move engine foundation directly to main
- User clarified that all work must be performed directly on `main`, without creating PRs.
- Fast-forwarded `main` from `af4b87c74225cf46f80b78eab6a56e76fabf471a` to `8bf795c2959a1ca78e4fe40618b418df015162ae`, carrying the tested engine-core foundation and PMCS updates.
- The prior PR #4 was closed after the direct branch update; GitHub reports it as merged because its head commit is now on `main`. No PR merge operation was used.
- Engine Tests run 44140955601 and PMCS PR run 44142349002 passed before the direct main update. Main push runs 44142921426 (Engine Tests) and 44142921444 (PMCS) were triggered; their final results are pending verification.
- Future work: direct commits to `main`, no PR workflow. No Minecraft Java 26.3 parity is claimed.

## 2026-11-17 — Engine test foundation CI verified
- Engine Tests run 44140955601 passed on PR head f504ba1f3629702330380c3bb71c44c4a5879150: rustfmt, cargo test --workspace --all-targets, and Clippy all succeeded.
- Project Memory Validation run 44140955639 also passed, including regression tests and structure/freshness validation.
- PR #4 remains open and unmerged; no Minecraft Java 26.3 parity is claimed.

## 2026-11-17 — Fix remaining Rust formatting difference
- Engine Tests run 44140653759 showed one remaining unformatted assertion in the negative-coordinate test; cargo test and Clippy were skipped.
- Reformatted all matching assertions and recorded the correction in the session log. A fresh successful workflow result is not yet confirmed.


## 2026-11-17 — Correct Rust test formatting after first CI run
- Engine Tests run 44140498548 correctly stopped at the formatting gate; rustfmt differences were found in the integration test.
- Applied the canonical rustfmt layout without changing test behavior; compilation, tests and Clippy remain pending rerun.


## 2026-11-17 — Add initial Rust engine core and automated tests
- Added a dependency-free Cargo workspace and a small platform-independent engine-core crate.
- Implemented horizontal block-to-chunk mapping with Euclidean division for negative coordinates; no dimension height or gameplay rules were invented.
- Added integration tests for positive/negative boundaries, round trips, integer extremes, and 16-block horizontal chunk width.
- Added Engine Tests GitHub Actions workflow for formatting, tests and Clippy.
- Added the initial engine test strategy; these invariants are not a Minecraft Java 26.3 parity claim.
- Android POC debugging is paused and was not modified.


# Project Changelog

## 2026-11-17 — Verify PMCS and extract installable APK
- Latest PMCS validation passed for commit cf521ffa8c936b868fe82b302483fd65e6d9390c.
- Extracted the successful Android workflow artifact and verified it contains an ARM64 debug APK.
- Recorded APK size and SHA-256; real-device runtime testing remains pending.


## 2026-11-17 — Expand Minecraft 26.3 reference baseline
- Added a sourced initial baseline for 26.3 release-specific features and technical changes.
- Expanded parity matrix with explicit domain coverage and reproducible fixture requirements.
- Kept protocol/data-version metadata provisional pending official version.json verification.


## 2026-11-17 — Android Vulkan POC CI passes
- ARM64 native library compilation and Gradle debug APK packaging passed in workflow run 44127492869.
- Artifact 14347124661 produced; SHA-256 recorded in CURRENT_STATE.md and ISSUES.md.
- Real-device Vulkan runtime validation remains pending.


## 2026-11-17 — Fix Android SDK setup action failure
- Android POC run 44127348128 failed before compilation because setup-android@v3 requested Google's removed SDK tools package.
- Updated the workflow to setup-android@v4 and made platform-tools installation explicit.
- Rust compilation and APK packaging remain pending.


## 2026-11-17 — Track Android POC validation gate
- Added I-009 to track ARM64 compilation, APK packaging and real-device Vulkan lifecycle validation.
- POC remains INVESTIGATING until CI and hardware results are recorded.


## 2026-11-17 — Implement isolated Android Vulkan POC
- Added a minimal ARM64 NativeActivity + winit 0.30.13 + wgpu 25.0.0 Vulkan clear-screen app.
- Added explicit Resumed/Suspended handling and surface recreation path.
- Added adapter/driver/capability logging and Gradle APK packaging.
- Added dedicated GitHub Actions workflow for Android compilation and debug APK packaging.
- CI and real-device results are pending; this is a platform POC, not engine implementation.


## 2026-11-17 — Refine Android renderer feasibility assessment
- Confirmed wgpu 25 documents Vulkan support on Android and winit 0.30 documents an Android glue/lifecycle path.
- Minelib's pinned manifests do not enable that Android entrypoint/configuration; its renderer remains unproven on Android.
- Updated the backend status to candidate-only and planned an isolated clear-screen POC before selecting the renderer.
- No Android build or device test has been performed.


## 2026-11-17 — Strengthen AI onboarding for specs and audits
- AI_HANDOFF now requires reading relevant version-pinned specifications and audit documents.
- PROJECT_MEMORY identifies Rusty Survival Engine and treats reference fixtures as a source of truth.
- Added an explicit rule against guessing unknown Minecraft Java 26.3 behavior.


## 2026-11-17 — Propose modular Android-first engine architecture
- Rewrote ARCHITECTURE.md as a layered design proposal with narrow interfaces and simulation/render/platform separation.
- Documented logical modules, dependency direction, deterministic scheduling principles, first vertical slice, and open decisions.
- Kept renderer backend, ECS, save format, multiplayer scope, and final crate layout undecided pending Android proof and 26.3 reference evidence.
- No engine code or parity implementation added.


## 2026-11-17 — Minelib renderer, assets and networking audit
- Added docs/audits/MINELIB_RENDER_ASSETS_NET_AUDIT.md.
- Renderer: ADAPT GPU techniques only; wgpu is a candidate but Minelib's renderer is not Android-proven and is coupled to winit and Minelib world/assets.
- Assets: ADAPT directory/ZIP loading abstraction; replace platform path handling and 26.3 pack semantics.
- Networking: REPLACE custom Minelib relay protocol for Minecraft Java 26.3 networking.
- Embedded VoxelArt pack: BLOCKED pending independent provenance and redistribution review.
- Source inspection only; no builds, tests, benchmarks, or security audit performed.

## 2026-11-17 — Establish Minecraft Java 26.3 compatibility baseline
- Added docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md with verified release metadata, release-specific gameplay/technical changes, unknown internals, reference-fixture methodology, and Android-first constraints.
- Recorded data version 5023, protocol 777, data pack 121.0, and resource pack 97.1 with source links.
- Explicitly marked world-height/chunk internals, generation algorithms, light/fluid scheduling, collision constants, storage internals, and full protocol schema as unresolved pending reference evidence.
- No engine implementation or parity verification is claimed.

## 2026-11-17 — PMCS regression suite verified
- GitHub Actions run 44126182263 completed successfully.
- Verified negative cases for missing documentation, implementation changes without required memory updates, and edits to existing session-log lines.
- Verified the positive case for implementation changes accompanied by all required memory updates.
- PMCS regression hardening is now VERIFIED; no engine, Android, Vulkan, or Minecraft parity tests are implied.

## 2026-11-17 — PMCS regression hardening
- Added isolated regression tests for missing required documents, implementation changes without required memory updates, append-only session journal violations, and valid implementation-plus-memory updates.
- Expanded code-change freshness requirements to include CHANGELOG, CURRENT_STATE, ISSUES, ROADMAP, and SESSION_LOG.
- Removed a stale commit SHA from CURRENT_STATE; the live repository head must be resolved from Git.

## 2026-11-17 — Pin Minecraft Java Edition 26.3
- Recorded the previously established project target as Minecraft Java Edition 26.3 in the handoff, project memory, compatibility record, parity matrix, decisions, and roadmap.
- Confirmed Horizon remains architectural inspiration only; no Horizon code or implementation is copied.
- No engine implementation or parity verification is claimed.

## 2026-11-17 — PMCS CI verification
- GitHub Actions run 44121524070 completed successfully for commit 0e1074a9bf6b4599820a7cc92373c76f51f59dad.
- Verified checkout history, Python setup, and PMCS structural/freshness validator step.
- PMCS automation is VERIFIED for its initial push workflow execution.

## 2026-11-17 — PMCS automation and continuity hardening
- Corrected project direction: Minecraft Java Edition is the behavior target; Horizon is inspiration only and no Horizon code is to be copied.
- Added append-only session journal and a standard change-record template.
- Added a standard-library Python validator for required memory files, required sections, duplicate session IDs, and memory freshness when implementation files change.
- Added GitHub Actions workflow for push, pull request, and manual validation.

## 2026-11-17 — Minelib world source audit
- Inspected all nine source files in crates/world/src and the world crate manifest.
- Documented custom 16×16×512 chunk model, fixed custom block IDs/registry, custom terrain/noise, custom RGB/torch lighting, fluid geometry without demonstrated Java-compatible fluid simulation, mesh architecture, and DDA raycast.
- Updated REUSE_MATRIX.md with module-level ADAPT/REPLACE decisions.
- Confirmed source contains unit tests, but they were not executed in this audit.
- No upstream source/assets copied; no Android build or Java parity claim made.

## 2026-11-17 — Preliminary Minelib audit
- Pinned upstream reference to 1cd3faeca7a29acf491b794dda1eacf768d61e41.
- Verified root AGPLv3 license text and noted GitHub metadata NOASSERTION.
- Inspected workspace manifests for all nine crates.
- Inspected platform, render, world, app entry point, audio module, and release workflow.
- Added preliminary REUSE_MATRIX.md.
- Identified world as the strongest candidate for deeper audit; no final KEEP decision.
- Confirmed Android support is not demonstrated by README or CI workflow.
- No upstream source or assets copied; no build or parity claim made.

## 2026-11-17 — Repository bootstrap
- Replaced the previous default-branch file tree with the Minelib Hybrid project bootstrap.
- Added initial PMCS documentation.
- Preserved prior Git history; no old files were inspected or reused.
- No game implementation or build is claimed.


## 2026-11-17 — Define engine layout and temporary reference-test policy
- Added docs/specs/ENGINE_LAYOUT.md with staged crate ownership, dependency direction, source-file conventions and engine-only scope.
- Added docs/specs/REFERENCE_TEST_PIPELINE.md with official-source reference acquisition, temporary runner storage, checksum/size verification, no cache/artifact/release and cleanup requirements.
- Clarified that modding is out of scope.
- No engine code, downloader or reference workflow was added in this documentation step.


## 2026-11-18 — Pivot repository to DreamMod
- User approved implementing DreamMod in this repository and selected Horizon as the target engine.
- Added docs/specs/DREAMMOD_ARCHITECTURE.md defining HNMod build-time integration, prebuilt Horizon distribution, interpreted .dmod runtime, extensibility, performance, security and the open language decision.
- Updated active project memory, compatibility policy, roadmap and README. The former Minelib/standalone Minecraft Java 26.3 engine direction is marked superseded/historical, not deleted.
- No HNMod, interpreter, loader, API, .dmod package or runtime build has been implemented in this documentation-only change.
- Verification: repository files and PMCS validator requirements inspected; GitHub Actions validation pending.


## 2026-11-18 — Audit Horizon's existing mod platform
- Inspected Horizon main commit 1c45cf95d8431a7b49d94df007f7e75a890370c9 and horizon-mixins commit e59da9094f0c2fdda6c627fe9eff22dec6a906f0.
- Found an existing typed mod API/host with Hooks, Hosts, Parts, attachments, capabilities, API injection points and native-only mixins/bridges/loop slots; DreamMod should reuse and extend it.
- Confirmed the current WASM runtime has a child-process compilation path and Wasmtime module compilation/cache, so it does not meet the strict interpreted-source/no-player-compilation requirement.
- Confirmed native host code loads prebuilt adjacent dynamic libraries and validates SHA-256/build identity; a CI-built DreamMod Core library may be distributable without player-side compilation, but end-to-end proof remains pending.
- Added docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md and updated DreamMod architecture/PMCS. No Horizon code was changed and no build was run.


## 2026-11-18 — Add DreamMod Core bootstrap skeleton
- Added mods/dreammod-core as a minimal native .hmod source package pinned to Horizon 1c45cf95d8431a7b49d94df007f7e75a890370c9 and horizon-mixins e59da9094f0c2fdda6c627fe9eff22dec6a906f0.
- Added a no-op typed Hooks implementation; no script runtime or public DreamMod API exists yet.
- Added a GitHub Actions workflow that compiles the Core against the pinned Horizon source checkout.
- CI result pending. No native library was prebuilt or loaded; no .dmod package was executed.


## 2026-11-18 — Harden the interpreted physics provider boundary
- Added pre-dispatch validation for equal position/velocity lengths, a fixed maximum batch size, and finite numeric inputs.
- Validate provider output length, numeric type and finite values before returning results to the host.
- Added regression tests for mismatched inputs, NaN/infinite inputs, wrong-sized outputs and non-finite outputs.
- Commit: 33639ee0d89627b7ad3342d694a2d26c1ff43fd5.
- Verification: GitHub Actions was triggered by the main push; its result has not yet been inspected. No production sandbox or full runtime is claimed.

## 2026-11-18 — Add the first versioned DreamMod package contract
- Added standalone `mods/dreammod-package` with a strict version-1 `manifest.json` model and side-effect-free validation.
- Validates schema/unknown fields, reverse-DNS-style IDs, semantic versions and API ranges, safe relative script entrypoint, explicit capability allowlist, duplicate capabilities, dependency IDs/ranges, and manifest/count limits.
- Added `.github/workflows/dreammod-package.yml` for formatting, tests and Clippy. CI result is pending; no passing result is claimed.
- This is manifest validation only: no archive parsing/extraction, capability granting, dependency resolution, script execution or production sandbox.

## 2026-11-18 — Add pre-extraction .dmod archive inventory policy
- Added a bounded metadata-only archive validator to `dreammod-package`.
- Rejects traversal/non-portable paths, case-folded duplicates, symlinks/unsupported/encrypted entries, unapproved file roots, excessive entry count/file size/aggregate expanded size/compression ratio, missing/duplicate manifest and missing script entrypoint.
- No ZIP parser or extraction code is integrated; central-directory/local-header consistency and actual archive handling remain future work. CI pending.


## 2026-11-18 — Add deterministic dependency planning
- Added a bounded dependency planner for installed `.dmod` manifests.
- Validates duplicate IDs, missing dependencies and semver requirements; detects cycles; returns stable dependency-first order.
- Added regression tests for deterministic ordering and invalid dependency graphs.
- Commit: 1817a3b4b9a3f4ae974793c2d46c9cbce65b3fe6.
- Verification: Actions status has not yet been inspected; no passing result is claimed.


## 2026-11-18 — Implement real in-memory .dmod ZIP inspection
- Added an exact-pinned zip dependency (9.6.1, default features disabled, Deflate only).
- Added strict archive header consistency validation before accessing entries.
- Added compressed-input, entry-count, per-file, aggregate expanded-size and compression-ratio limits; rejects encrypted entries, symlinks, unsupported compression, unsafe/duplicate paths and unsupported roots.
- Metadata is preflighted before decompression. The manifest is parsed first; all regular files are then streamed to EOF without extraction to validate actual output size and CRC.
- Added real ZIP fixture tests for valid packages, path traversal, malformed/truncated input and payload corruption.
- Commits: 32d52810bb2181c7c6bb03a4f7152b402a8c8d9e, d7e7794567efb0cf1873686964fe47e7364ccbf4, 8bea071635acc45098bd4e93c5c66cbdf5296b30, 15c650a0e61ed3707e6ff76f784180f7f9115452, fc6af40387fa9b861b19ec7fb6b148046c83df8d, 6add01bb156b8ff0e826de40dbd7712a014daf41.
- Verification: CI not yet inspected; no passing result claimed.


## 2026-11-18 — Verify DreamMod package inspection CI and fix regressions
- Inspected Actions run 44291904988 and found the path-traversal test inserted a literal backslash into JSON without escaping it. Corrected the fixture to JSON-escape backslashes; all 19 package tests then passed.
- Clippy then reported a needless boolean branch in the archive path validator. Simplified the predicate and reran the complete workflow.
- Final verification: Actions run 44299317538 on commit 4e983c04ea8f3a4085aa2e13b6b53d691c307564 passed formatting, all 19 tests and Clippy with `-D warnings`.
- Updated source comments to accurately describe the in-memory ZIP inspector.
- No filesystem extraction, installation, capability grant, script execution, or production sandbox is implemented.


## 2026-11-18 — Make PMCS session ID checks compatible with legacy history
- PMCS regression run 44299535756 exposed repeated identifiers already present in the historical append-only session log; a new entry had reused one of those identifiers.
- Updated validation to preserve legacy headings while rejecting any newly appended ID that duplicates another new ID or reuses an ID from the prior history.
- Added negative tests for duplicate IDs in one update and reuse of a historical ID.
- Full PMCS workflow verification is pending.


## 2026-11-18 — Retain validated .dmod script and asset bytes in memory
- Extended ValidatedPackage with bounded PackageFile records containing validated script and asset bytes.
- The manifest is parsed separately; directory entries and manifest bytes are not retained as package files.
- Each regular package file is streamed to EOF and CRC-checked before its bytes are returned. Existing 16 MiB per-file and 64 MiB aggregate uncompressed limits bound retained content.
- Added real-ZIP fixture assertions for exact script and asset payloads.
- No disk extraction, installation, capability grant or script execution is implemented. CI pending.


## 2026-11-18 — Verify in-memory package contents and PMCS migration
- DreamMod package Actions run 44300015439 passed on commit 2dd019b4a370fec853ec19849c4538f6cce9be44: formatting, all 19 tests, and Clippy with -D warnings.
- The real ZIP fixture confirms that script and asset payload bytes are retained exactly after validation; no disk extraction is used.
- Legacy-aware PMCS session ID validation passed regression tests and full structural/freshness validation in run 44300145443.
- Historical duplicate session headings remain unchanged; new IDs are checked against prior history.


## 2026-11-18 — Add explicit default-deny capability planning
- Added plan_capability_grants(manifest, approved), returning deterministic granted and denied capability sets.
- Empty approval input grants nothing. Approvals cannot exceed manifest requests; unknown, unrequested and duplicate approvals are rejected.
- Added regression tests for default-deny, partial approval and invalid approval inputs.
- The planner is pure metadata; it does not grant runtime authority or replace host-side enforcement. CI pending.


## 2026-11-18 — Verify explicit capability planning
- Actions run 44300324687 passed on commit d83b3491b747bc880b657e4cf17e259a8af2ff62: formatting, all 21 tests and Clippy with -D warnings.
- Default-deny, partial approval, unknown/unrequested approval and duplicate approval tests passed.
- CapabilityPlan remains a pure planning result; actual host-side enforcement is not implemented.


## 2026-11-18 — Add default-deny host capability gate
- Added CapabilityGate::from_approvals(), allows() and require() with a private granted set.
- Gate construction routes through manifest validation and the explicit approval planner; unapproved and unknown capabilities are denied.
- Added tests for approved, denied and unknown capabilities.
- This helper is not yet wired into Horizon host calls; end-to-end enforcement is not claimed. CI pending.


## 2026-11-18 — Verify default-deny host capability gate
- Actions run 44300879584 passed on commit 5085971c93c3dbc2a0089e136e5f9a7aecc21a49: formatting, all 22 tests and Clippy with -D warnings.
- Gate tests verify approved access, denied unapproved access and rejection of unknown capabilities.
- This verifies the helper only; Horizon host API integration and end-to-end sandbox enforcement remain open.


## 2026-11-18 — Define owned player snapshot bridge contract
- Added an owned `PlayerSnapshot` DTO for UUID, bounded Minecraft player name, dimension ID and finite bounded position.
- The DTO contains no Horizon handles, references or borrowed strings and can safely outlive the hook that produced it.
- Added `HostAdapter::read_player_snapshot()`, which checks the `entities.read` capability and enabled lifecycle state before invoking the Horizon-side read closure.
- Added tests for valid DTOs, invalid names/dimensions/coordinates, denied callback non-invocation and invalid snapshot rejection.
- This is the typed bridge contract and policy boundary only; no concrete `ServerHost` implementation or script exposure is wired yet.
- Source commits: `3e8baf1`, `8845d96`, `7bae3ea`. Verification: Actions run `44309091208` passed formatting, all package tests and Clippy.


## 2026-11-18 — Add bounded player snapshot batches
- Added `PlayerSnapshotBatch` with a maximum of 256 entries and duplicate-UUID rejection.
- Added `HostAdapter::read_player_snapshots()`, which performs one `entities.read` lifecycle/capability check before one batch host callback.
- This is designed for one coarse native-to-script crossing per batch rather than one crossing per player. The concrete Horizon collector and script binding remain open.
- Added tests for batch size, duplicate identities and authorization before the host callback. CI pending.


## 2026-11-18 — Verify bounded player snapshot batch contract
- DreamMod package Actions run 44309674771 passed on source commit 4397de1a28d536b0ffca448b553478ecae3467fb.
- Formatting, all package tests and Clippy with -D warnings succeeded.
- The batch remains a contract/policy primitive; no concrete Horizon collector or script binding is claimed.


## 2026-11-18 — Add the first concrete Core-side Horizon player collector
- Added `mods/dreammod-core/src/player_snapshot.rs`, using the pinned Horizon `ServerHost` player service and live `PlayerRef` reads.
- Copies UUID, validated player name, dimension identifier and finite bounded position into DreamMod-owned DTOs.
- Rejects more than 256 online players before per-player reads; rejects unavailable/stale fields, invalid identifiers/coordinates and duplicate UUIDs.
- Does not retain Horizon handles or expose snapshots to scripts. No .dmod runtime or sandbox integration is implied.
- Source commits: `7ecd981`, `b2362c6`. Core SDK check/build pending; PMCS validation previously failed because implementation changes lacked required memory updates.


## 2026-11-18 — Gate Core player collection behind per-mod authorization
- Changed the Core collector's public entry point to require a `PlayerReadAuthorization` implementation before it touches `ServerHost`.
- Added a generic authorization helper with regression tests proving denied reads do not invoke the host operation and authorized reads invoke it exactly once.
- Kept the actual lifecycle adapter open: the trait is an integration seam, not proof that `ModLifecycle` is already wired to Core.
- Core source commit: `99929e38d5f1a815e8b17cb069a5eff8dfcc8e3e`; Core and overlay Actions are running.


## 2026-11-18 — Add Core snapshot validation regressions
- Added Core-local unit tests for malformed player names, invalid dimension identifiers, NaN/infinite/out-of-range positions, duplicate player UUIDs and the 256-entry batch ceiling.
- Source commit: `ebe09145849063ade0e01f18e55a23bec18cd4e4`. Core Actions are queued/running; tests have not yet been reported as executed by CI.


## 2026-11-18 — Correct gated collector refactor
- Corrected the collector body after a source-edit regression left the previous public function declaration duplicated inside the private helper.
- The latest source commit `eadd1aa545851d487da4e2ff2f37edfbec327dfb` restores one private unchecked collector, called only from the authorization-gated public entry point.
- The prior in-flight Core run targets the intermediate broken commit; check the new run for the corrected source.


## 2026-11-18 — Execute Core tests in CI
- Extended `.github/workflows/dreammod-core.yml` to run `cargo fmt --all -- --check` and `cargo test` after the pinned Horizon SDK check has fetched/verified the SDK.
- This makes the Core-local authorization and snapshot validation tests executable in CI instead of merely present in source.
- Workflow commit: `ffe93f053ab6a0a0df99953a1206262661e8b111`. First run must confirm the test command works with the generated SDK path.


## 2026-11-18 — API optimization and bounded package loading
- Replaced BTreeSet-based duplicate UUID checks in Core and package snapshot batches with one contiguous UUID vector, sort_unstable, and adjacent duplicate detection; batch cap/order unchanged.
- Added a cooperative 60-second monotonic deadline to .dmod archive inspection, including checks between entries and decompression chunks, plus an expired-deadline regression test.
- Fixed Core CI toolchain setup to install rustfmt after run 44313213891 exposed a missing component (Horizon check had passed; tests did not run).
- Verification pending: package and Core Actions must pass before these changes are marked verified. No performance benchmark has been run.


## 2026-11-18 — Remove heap allocation from snapshot duplicate checks
- Core and package bridge now use a fixed 256-entry u128 stack scratch array plus sort_unstable to detect duplicate player UUIDs. The batch cap and output order are unchanged.
- Core commit b2185e8917e351dfd0408231e855efe47e6b3bfd; package bridge commit f3a55af3a332073db92df24de00e347bc55a3bb8.
- Core formatting now runs against the Core manifest only, excluding generated SDK sources. CI is pending for the latest changes.


## 2026-11-18 — Share immutable player snapshot batches
- Changed Core and package PlayerSnapshotBatch backing storage to Arc<[PlayerSnapshot]> so cloning a batch shares immutable data instead of deep-copying player records.
- Added pointer-identity regression tests. Core commit 3f629cc54a27992f9a7f402dfda3fcfa0505b4bf; package commit 74310a797393ec319fb9192d931a7b860ba55ced.
- CI pending. This enables cheap sharing but does not add a runtime dispatcher or cache.


## 2026-11-18 — Verify Core snapshot implementation and correct overlay test route
- Core workflow run 44322745253 passed on HEAD 741f7ff15bb22a2b16b8a5a667fe471d3ee3ddc9: pinned Horizon native/API/host-WASM validation, formatting, unit tests and native source packaging.
- The earlier Core unit-test fixture mismatch was corrected by explicitly converting the test UUID range value to u128; the corrected test suite passed in the run above.
- Package Arc-backed snapshot CI run 44318172781 passed; archive inspection deadline CI run 44317125357 passed.
- PMCS run 44322745310 failed because required companion memory documents were missing; this change set records the actual CI evidence and remaining work.
- Overlay run 44322745328 failed at compile time because the injected test referred to the UI-facing picker Route instead of prebuilt::Route. The patch script now qualifies the API route correctly. No overlay or host-loader success is claimed until a fresh run passes.
