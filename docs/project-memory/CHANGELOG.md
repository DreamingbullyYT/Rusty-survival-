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
