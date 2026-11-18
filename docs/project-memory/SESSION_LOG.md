# Session Log (append-only)

This file is chronological and append-only. Never rewrite or delete earlier session entries. Add a new entry at the end for each meaningful work session. The validator rejects deletion of existing non-empty journal lines in pull requests and pushes when the base commit is available.

## S-2026-11-17-01 — PMCS hardening
Date: 2026-11-17
Commit: pending
Goal: Make project continuity durable across separate AI sessions and context compression.
Changes:
- Added mandatory onboarding and handoff rules.
- Clarified Minecraft Java Edition as behavior target and Horizon as inspiration only, with no copied code.
- Added session journal and change template.
- Added automated structural and memory-freshness validation plus GitHub Actions workflow.
Verification:
- Repository contents inspected at baseline commit 39c5651ba3c032d680f5f918ba4abd50055b4fdd.
- Local validator test: pending.
- GitHub Actions run: pending.
Limitations:
- Automation is not considered VERIFIED until CI runs successfully.
- Exact Minecraft Java Edition release/version remains unselected.

## S-2026-11-17-02 — Verify PMCS CI
Date: 2026-11-17
Commit: 0e1074a9bf6b4599820a7cc92373c76f51f59dad (automation commit)
Goal: Verify the new PMCS workflow on GitHub Actions.
Changes:
- Confirmed workflow was triggered by push on main.
- Inspected run and job steps for completion.
Verification:
- GitHub Actions run 44121524070: SUCCESS.
- Checkout repository history: SUCCESS.
- Set up Python 3.12: SUCCESS.
- Validate project memory: SUCCESS.
Not tested:
- Deliberately broken documentation and stale-memory negative cases.
- Full engine build, Android build, Vulkan, and Minecraft parity (no engine exists yet).
Next:
- Confirm exact Minecraft Java Edition target version, then write the target-version specification before engine code.

## S-2026-11-17-03 — Pin Java Edition target
Date: 2026-11-17
Commit: pending
Goal: Ensure PMCS preserves the previously established project target.
Changes:
- Recorded Minecraft Java Edition 26.3 as the selected parity target.
- Clarified Horizon is inspiration only and all engine implementation is independent.
Verification:
- Updated DECISIONS, COMPATIBILITY, PARITY_MATRIX, ISSUES, CURRENT_STATE, and ROADMAP.
Not tested:
- No Minecraft Java 26.3 parity behavior has been implemented or tested.
Next:
- Produce the version-specific technical specification before engine implementation.

## S-2026-11-17-04 — Synchronize current handoff and project memory
Date: 2026-11-17
Commit: pending
Goal: Ensure the mandatory AI entry documents agree with the selected version.
Changes:
- Updated AI_HANDOFF and PROJECT_MEMORY to state Minecraft Java Edition 26.3.
- Added a changelog entry for the target-version decision.
Verification:
- Structural markers retained for PMCS validation.
Not tested:
- This documentation-only commit does not test engine behavior or Minecraft parity.
Next:
- Start the Minecraft Java 26.3 technical specification.

This file is chronological and append-only. Never rewrite or delete earlier session entries. The validator rejects deletion of existing non-empty journal lines in pull requests and pushes when the base commit is available.

## S-2026-11-17-05 — PMCS regression hardening
Date: 2026-11-17
Commit: pending
Goal: Exercise PMCS failure paths and remove a stale commit identifier from current state.
Changes:
- Added isolated regression tests for missing required documents, implementation changes without memory updates, append-only journal violations, and a valid implementation-plus-memory update.
- Expanded the implementation freshness gate to require CHANGELOG, CURRENT_STATE, ISSUES, ROADMAP, and SESSION_LOG updates together.
- Excluded test fixtures from implementation freshness detection.
- Replaced the stale literal HEAD SHA in CURRENT_STATE with an instruction to resolve the branch head dynamically.
Verification:
- Pending GitHub Actions execution for the new regression suite.
- Existing PMCS push runs were successful, but they did not exercise these negative cases.
Not tested:
- Engine code, Android build, Vulkan, and Minecraft Java parity; no engine exists yet.
Next:
- Confirm the regression suite passes in GitHub Actions, then continue the version-specific technical specification.


## S-2026-11-17-06 — Verify PMCS regression suite
Date: 2026-11-17
Commit: pending
Goal: Confirm the PMCS validator rejects broken continuity states and accepts a valid implementation update.
Changes:
- Recorded the regression suite and verification outcome in the current state and roadmap.
- Confirmed CURRENT_STATE no longer stores a hard-coded HEAD SHA.
Verification:
- GitHub Actions run 44126182263: SUCCESS.
- Missing required document negative test: PASS (validator correctly failed).
- Implementation change without memory updates negative test: PASS (validator correctly failed).
- Existing session-log line edit negative test: PASS (validator correctly failed).
- Valid implementation change with required memory updates: PASS.
- Full PMCS structural/freshness validation: PASS.
Not tested:
- Engine code, Android build, Vulkan, Minelib upstream tests, and Minecraft Java parity; no engine exists yet.
Next:
- Begin the Minecraft Java Edition 26.3 technical/parity specification and finish architecture/platform risk review before writing engine code.


## S-2026-11-17-07 — Establish Minecraft Java 26.3 baseline
Date: 2026-11-17
Commit: pending
Goal: Convert the selected target version into an evidence-backed release baseline before engine implementation.
Changes:
- Added docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md.
- Recorded version metadata, 26.3-specific gameplay/data/rendering/network changes, unknown internals, reference-test method, and Android constraints.
- Updated roadmap and current state to mark the initial baseline as complete while keeping detailed internals and architecture open.
Verification:
- Consulted official Minecraft 26.3 release notes and official feedback release notes.
- Cross-checked protocol/data version and pack formats against version metadata references.
- No game binary or asset was downloaded, copied, or committed.
Not tested:
- No black-box fixtures captured from a local 26.3 installation.
- No engine implementation, Android build, Vulkan, Minelib upstream tests, or parity tests.
Next:
- Capture 26.3 version metadata and data/registry inventories, then specify chunk/world semantics with reproducible evidence.


## S-2026-11-17-08 — Audit Minelib renderer, assets and networking
Date: 2026-11-17
Commit: pending
Goal: Decide whether the remaining major Minelib crates can support an Android-first Java 26.3 engine.
Changes:
- Added docs/audits/MINELIB_RENDER_ASSETS_NET_AUDIT.md.
- Updated REUSE_MATRIX with renderer, assets, networking and embedded-pack decisions.
- Renderer: ADAPT techniques only; no Android surface/lifecycle proof and tight Minelib coupling.
- Assets: ADAPT directory/ZIP abstraction only; replace desktop path ownership and implement 26.3 formats independently.
- Networking: REPLACE Minelib's custom relay protocol for Java multiplayer.
- Embedded VoxelArt pack: BLOCKED pending provenance and redistribution rights.
Verification:
- Inspected pinned source manifests and relevant renderer initialization, asset path/resource-pack code, and network protocol/transport/game message definitions.
- No source, shader, binary, or asset copied.
Not tested:
- No upstream build/tests, Android build, Vulkan surface, GPU benchmark, security audit, or Java protocol interoperability test.
Next:
- Continue 26.3 reference data capture and prove Android surface/lifecycle before finalizing backend and crate boundaries.


## S-2026-11-17-09 — Propose modular Android-first engine architecture
Date: 2026-11-17
Commit: pending
Goal: Define clean logical boundaries before choosing crates or implementing engine systems.
Changes:
- Rewrote ARCHITECTURE.md as a layered proposal covering Android platform, runtime, vanilla data, world model/generation, simulation, physics, renderer, resource packs, persistence, audio, optional Java protocol, and testkit.
- Defined dependency direction and immutable renderer snapshots.
- Kept wgpu/Vulkan, ECS, save format, multiplayer scope and final crate boundaries OPEN pending proof and reference evidence.
- Updated roadmap and current state.
Verification:
- Documentation review against the 26.3 baseline and Minelib component audit.
- No engine source code was added.
Not tested:
- No Android app, surface, renderer, simulation, persistence or parity tests.
Next:
- Build the minimal Android lifecycle/surface proof and continue collecting 26.3 reference data before final architecture approval.


## S-2026-11-17-10 — Strengthen AI onboarding for specifications and audits
Date: 2026-11-17
Commit: pending
Goal: Ensure new AI sessions discover the version baseline, component audits, and architecture proposal without relying on chat memory.
Changes:
- Updated AI_HANDOFF to require reading relevant docs/specs and docs/audits.
- Updated PROJECT_MEMORY identity to Rusty Survival Engine and included reference-fixture evidence as a source of truth.
- Added a rule against guessing unknown Minecraft Java 26.3 behavior.
Verification:
- Documentation cross-check only.
Not tested:
- No engine or Android behavior; current architecture remains a proposal.
Next:
- Prove Android lifecycle/surface integration and continue version-specific reference capture.


## S-2026-11-17-11 — Refine Android renderer feasibility assessment
Date: 2026-11-17
Commit: pending
Goal: Determine whether a realistic Rust Android Vulkan proof-of-concept path exists.
Changes:
- Cross-checked wgpu 25 supported-platform documentation and winit 0.30 Android integration/lifecycle documentation.
- Clarified that wgpu 25 lists Vulkan on Android and winit 0.30 has an Android glue path, but Minelib's pinned manifests do not enable that Android entrypoint/configuration.
- Updated the Minelib renderer audit and architecture proposal; wgpu+winit is now a credible POC candidate, not a selected backend.
- Changed I-006 to INVESTIGATING and made the POC the next action.
Verification:
- Source manifests and upstream release workflow inspected.
- Official winit docs confirm Android glue feature and Resumed/Suspended surface lifecycle requirements.
- wgpu 25 docs list Vulkan support on Android.
Not tested:
- No Android compile, APK, device runtime, Vulkan adapter, surface, or swapchain test.
Next:
- Build an isolated clear-screen Android POC with wgpu 25 + winit 0.30; do not add engine/gameplay code to it.


## S-2026-11-17-12 — Implement isolated Android Vulkan POC
Date: 2026-11-17
Commit: pending
Goal: Test a minimal Android app lifecycle and Vulkan presentation path without starting engine/gameplay implementation.
Changes:
- Added poc/android-vulkan Rust cdylib using NativeActivity glue, winit 0.30.13 and wgpu 25.0.0 with Vulkan backend.
- Added explicit Resumed/Suspended handling; surface-dependent GPU state is dropped on suspend and recreated on resume.
- Added adapter/driver/features/limits/surface capability logging and a clear-only render pass.
- Added Gradle packaging and a dedicated ARM64 Android build workflow.
- Added README with local build and device-test instructions.
Verification:
- Source authored and committed; CI build pending.
Not tested:
- No Android compiler result, APK install, real-device Vulkan presentation, lifecycle, rotation, or performance test yet.
- No engine or Minecraft parity code.
Next:
- Fix any CI compile/package failures, then test the generated APK on real hardware.


## S-2026-11-17-13 — Record Android POC validation gate
Date: 2026-11-17
Commit: pending
Goal: Ensure PMCS explicitly tracks the newly added Android POC build and device-validation gate.
Changes:
- Added I-009 for Android Vulkan POC build and real-device validation.
- Kept the POC status INVESTIGATING until CI and hardware results exist.
Verification:
- Documentation consistency check.
Not tested:
- Android compile/package and device runtime are still pending.
Next:
- Review Android Vulkan POC and PMCS workflow results; fix any failures before device testing.


## S-2026-11-17-14 — Fix Android SDK setup action failure
Date: 2026-11-17
Commit: pending
Goal: Remove the first CI blocker before the Android Rust build begins.
Changes:
- Android POC run 44127348128 failed in setup-android@v3 because that action attempted to install Google's removed SDK tools package.
- Updated the Android POC workflow to setup-android@v4, whose current documentation no longer requests the removed package by default.
- Kept Android SDK/NDK package installation explicit.
Verification:
- Failure root cause confirmed from the workflow job log.
- Retry/build on the corrected workflow is pending.
Not tested:
- Rust Android compilation, Gradle packaging, APK installation, Vulkan device runtime.
Next:
- Verify setup-android@v4, then fix any Rust/Gradle errors exposed by the build.


## S-2026-11-17-15 — Android Vulkan POC CI passes
Date: 2026-11-17
Commit: pending
Goal: Record successful ARM64 native compilation and APK packaging, keeping hardware runtime validation separate.
Changes:
- GitHub Actions run 44127492869 completed SUCCESS.
- ARM64 Rust native library build passed.
- Gradle debug APK packaging passed.
- Artifact 14347124661 created (2,379,313 bytes; SHA-256 d4ec89aca87ae27dbd22f7505965be1017b5f522790323363581cb8ed4fafd61).
- Clarified that CI success does not prove device Vulkan presentation or lifecycle behavior.
Verification:
- Workflow run and every build/package step reported success.
Not tested:
- No real-device install, Vulkan presentation, pause/resume, rotation or surface recreation.
Next:
- Install artifact APK on a real Android ARM64 device and record adapter/driver and lifecycle results.


## S-2026-11-17-16 — Expand Minecraft 26.3 reference baseline
Date: 2026-11-17
Commit: pending
Goal: Convert the selected Minecraft Java 26.3 target into a sourced initial behavior baseline and a test-oriented parity matrix.
Changes:
- Added docs/specs/MINECRAFT_JAVA_26.3_BASELINE.md with release identity, technical formats, version-specific gameplay/rendering changes, unresolved core specifications and evidence policy.
- Expanded PARITY_MATRIX.md into concrete domains and a required deterministic fixture format.
- Updated CURRENT_STATE.md and ROADMAP.md to distinguish documented release facts from unverified protocol/data-version metadata and incomplete mechanics.
Verification:
- Cross-checked release-specific facts against Mojang's official 26.3 release notes.
- Protocol 777 and data version 5023 are recorded as community-reference values and explicitly marked for confirmation from official version.json before implementation.
Not tested:
- No official jar has been extracted; no registry inventory, fixture, engine code or parity test exists yet.
Next:
- Obtain lawful reference artifact access, verify version.json, then extract registry/tag/block-state metadata.


## S-2026-11-17-17 — Verify PMCS and extract installable APK
Date: 2026-11-17
Commit: pending
Goal: Confirm the documentation update passes PMCS and make the generated APK directly available for device testing.
Changes:
- Latest PMCS validation run 44134445257 completed SUCCESS for commit cf521ffa8c936b868fe82b302483fd65e6d9390c.
- Downloaded workflow artifact 14347124661 and inspected its archive; it contains app-debug.apk.
- Extracted APK to the conversation output and confirmed the packaged native library is ARM64-only at lib/arm64-v8a/libmain.so.
- APK size: 7,726,873 bytes. SHA-256: 27493418f81bd866dbbfe89cff4bcdc5ff9f4ef058142d127adcc10545538552.
Verification:
- APK archive structure and ARM64 native library presence confirmed.
- PMCS workflow passed.
Not tested:
- APK installation and runtime on the user's device; Vulkan rendering, lifecycle and surface recreation remain unverified.
Next:
- Install APK on ARM64 Android, then capture logcat and verify Vulkan adapter/surface/present and lifecycle.


## S-2026-11-17-18 — Add initial engine core and automated tests
Date: 2026-11-17
Commit: pending
Goal: Start engine work with a small platform-independent core primitive and an automated test gate, without inventing incomplete Minecraft behavior.
Changes:
- Added root Cargo workspace and crates/engine-core with no third-party dependencies.
- Added horizontal block-to-chunk mapping using Euclidean division/remainder; negative block coordinates map to valid local X/Z coordinates.
- Kept Y absolute and intentionally did not define dimension height, section height, block states, world generation or gameplay.
- Added integration tests for positive/negative chunk boundaries, round-trip behavior across -4097..=4097, integer extremes, and the 16-block chunk width.
- Added .github/workflows/engine-tests.yml to run formatting, tests and Clippy on Rust changes.
- Added docs/specs/ENGINE_TEST_STRATEGY.md.
Verification:
- Source and test code added; GitHub Actions result pending.
- Local Rust tests not run because rustc/cargo are unavailable in the current execution environment.
Not tested:
- No official Minecraft Java 26.3 reference fixture has been captured.
- No world/chunk storage, sections, registries, simulation, physics, lighting, fluids, generation, persistence or renderer implementation.
- No Android build or Vulkan work was performed in this session.
Next:
- Wait for Engine Tests workflow; fix any failures and only then add the next small, evidence-backed engine primitive.


## S-2026-11-17-19 — Fix initial Rust formatting check
Date: 2026-11-17
Commit: pending
Goal: Resolve the first Engine Tests CI failure without weakening the checks.
Changes:
- Engine Tests run 44140498548 installed stable Rust successfully, then failed at cargo fmt --all -- --check because two assertions and one small array did not match rustfmt output.
- Reformatted the integration test to rustfmt's canonical layout; test logic is unchanged.
Verification:
- Root cause confirmed from the exact GitHub Actions job log.
- Follow-up Engine Tests run is pending.
Not tested:
- cargo test and Clippy did not run in the failed attempt because the formatting step stopped the job.
Next:
- Re-run Engine Tests and inspect compile, test and lint results.


## S-2026-11-17-20 — Complete rustfmt correction
Date: 2026-11-17
Commit: pending
Goal: Fix the remaining formatting difference reported by the second Engine Tests run.
Changes:
- Engine Tests run 44140653759 still failed at cargo fmt because the negative-coordinate case contained a second assertion with the same long-line formatting.
- Updated all matching assertions to the canonical multi-line rustfmt layout.
Verification:
- Exact remaining diff identified from job 134242296995.
- Latest Engine Tests run after this commit is pending.
Not tested:
- cargo test and Clippy remain unexecuted because formatting stops the job first.
Next:
- Verify the next Engine Tests run reaches compilation, tests, and Clippy.


## S-2026-11-17-21 — Verify engine test foundation CI
Date: 2026-11-17
Commit: f504ba1f3629702330380c3bb71c44c4a5879150 (PR head)
Goal: Confirm the corrected engine foundation passes the complete automated Rust test gate.
Changes:
- Reviewed the latest Engine Tests run 44140955601 for the current PR head.
- Confirmed rustfmt, cargo test --workspace --all-targets, and cargo clippy --workspace --all-targets -- -D warnings all completed successfully.
- Confirmed Project Memory Validation run 44140955639 passed, including regression and freshness checks.
- Updated current state, issue evidence, roadmap, and changelog with the verified result.
Verification:
- Engine Tests run 44140955601: SUCCESS; job 134243338135 and all six substantive setup/check/test/lint steps succeeded.
- Project Memory Validation run 44140955639: SUCCESS; regression tests and structure/freshness validation succeeded.
- The PR remains OPEN and unmerged. No merge was performed.
Limitations:
- These tests prove the current foundational coordinate invariants and Rust quality gates only; they do not establish Minecraft Java Edition 26.3 parity.
- No version-pinned 26.3 differential fixture, world storage, chunk sections, or gameplay implementation exists yet.
- Local Rust tests were not run because rustc/cargo are unavailable in this execution environment; GitHub Actions executed the Rust checks.
Next:
- Design a lawful, deterministic 26.3 reference-fixture capture format and runner before implementing version-specific world behavior.


## S-2026-11-17-22 — Move engine foundation directly to main
Date: 2026-11-17
Commit: 8bf795c2959a1ca78e4fe40618b418df015162ae
Goal: Apply the already-tested engine foundation directly to main, following the user's explicit repository workflow preference.
Changes:
- User clarified that no pull requests or intermediate feature branches should be used; all work should be done directly on main.
- Fast-forwarded main from af4b87c74225cf46f80b78eab6a56e76fabf471a to the tested head 8bf795c2959a1ca78e4fe40618b418df015162ae.
- The old PR #4 was closed after its head was moved onto main. GitHub reports merged=true because the PR head is now contained in main; no PR merge operation was performed.
- Established direct-to-main as the default workflow for future changes.
Verification:
- Engine Tests run 44140955601: SUCCESS (formatting, cargo test --workspace --all-targets, Clippy).
- PMCS run 44142349002: SUCCESS on the exact main head before push.
- Main push runs 44142921426 (Engine Tests) and 44142921444 (PMCS) were triggered; outcomes pending at the time of this entry.
Limitations:
- Engine tests cover foundational coordinate invariants only; no Minecraft Java Edition 26.3 parity is established.
- No reference fixtures, world storage, sections, registry, or gameplay implementation yet.
Next:
- Verify both main push workflows, then continue the version-pinned reference-fixture harness directly on main.


## S-2026-11-17-23 — Define engine layout and temporary reference-test policy
Date: 2026-11-17
Commit: dcbc7d0c858918c6a52769a54f0e6a9a7d2b8934 (layout), 6e6ef01402b8bdc4cf1d17a2ffe9f23ff225acf7 (reference pipeline), followed by project-memory synchronization commits.
Goal: Turn the user's engine-only direction into a readable implementation map and a compliant, non-redistributing reference-test plan.
Changes:
- Reviewed the current Rusty Survival main branch and current Horizon architecture/layer, native-client, parity and CI documentation.
- Re-inspected Minelib's AGPLv3 root license, world crate, raycast/noise/mesh concepts, renderer and asset path assumptions. No source was copied.
- Added docs/specs/ENGINE_LAYOUT.md with staged crate ownership, dependency direction, source-file conventions and explicit no-modding scope.
- Added docs/specs/REFERENCE_TEST_PIPELINE.md for official-source reference tests using ephemeral $RUNNER_TEMP, size/hash checks, no cache/artifact/release, and cleanup on all outcomes.
- Updated AI_HANDOFF, CURRENT_STATE and PROJECT_MEMORY to record the engine-only scope and reference-artifact policy.
Verification:
- Read main HEAD cd78a65be0c779e1a1a676e2524fd973ab3c3ecc before the documentation changes.
- Inspected Horizon docs/layers.md, AGENTS.md, native-client.md, rust-parity.md and parity-workflow.md.
- Inspected Minelib Cargo manifests, root AGPLv3 LICENSE, world chunk/raycast/noise/mesh sources, renderer entry point and asset path implementation.
- Documentation commits were applied directly to main.
Not tested:
- GitHub Actions validation for these documentation updates is pending.
- No Rust source, fixture runner, Mojang downloader or reference workflow was implemented.
- No Minecraft artifact was downloaded; no Java 26.3 differential fixture exists.
- Only engine-core exists; the crate tree is a staged plan, not an implemented game.
Next:
- Implement fixture schema and synthetic harness tests, then official metadata/hash-verified temporary download and manual Actions workflow. Keep direct-to-main and do not start modding.


## S-2026-11-17-24 — Implement version-pinned fixture contract and validator
Date: 2026-11-17
Commit: implementation commits `6a0fb18f95a0126a791d0889a1a30c7f2e308420`, `e8647f94df5f785fd0f6b03436fa1688b5b7510e`, `39a665d96584b916b7997fb68ac410992bcb6504`, and workspace commit `9fe6551a680bf359cd48b2efe63eba31b6226a61`; memory synchronization commits follow.
Goal: Establish a typed, version-pinned fixture contract before downloading or executing a Minecraft reference artifact.
Changes:
- Added `crates/testkit` as a separate workspace crate; engine runtime dependencies remain unchanged.
- Added a serde JSON model for fixture identity/classification, Java 26.3 target, subsystem, optional artifact identity/hash/size/official URL, scenario inputs/actions, expected output, normalization and capture provenance.
- Added fail-closed validation for schema/version, required identity fields, artifact SHA-1/size/official Mojang HTTPS host, action names, normalization paths/tolerance and provenance.
- Added five synthetic self-tests and a clearly labelled synthetic coordinate fixture. Synthetic data is harness-only and is not Java parity evidence.
- Updated project memory, roadmap and handoff.
Verification:
- Repository HEAD and previous PMCS run were verified before changes; run 44149547685 completed SUCCESS on prior HEAD `63c979c1865f932df3e2572d9a56236524616711`.
- Rust CI and PMCS validation for this change are pending.
Not tested:
- No local Rust compiler is available in this environment; no local cargo fmt/test/clippy was run.
- No official Mojang metadata or game artifact was downloaded.
- No Java oracle, Rust probe comparator or Java 26.3 differential fixture exists.
- No Minecraft parity is established by synthetic tests.
Next:
- Verify Rust CI and PMCS; fix any reported issues. Then implement official metadata resolution and temporary artifact verification/cleanup, followed by a manual reference workflow. Keep direct-to-main and engine-only scope.


## S-2026-11-17-25 — Verify fixture testkit Rust CI
Date: 2026-11-17
Commit: pending (documentation-only verification addendum)
Goal: Record the complete Rust CI result for the new fixture contract and its formatting correction.
Changes:
- Engine Tests run 44156147284 initially failed only at rustfmt; the job log identified formatting-only differences in `crates/testkit/src/lib.rs`.
- Applied rustfmt's reported layout in commit `d6c07dad02aebcd310317c306e17d7c423075661`; no test logic changed.
Verification:
- Engine Tests run 44156319837 on exact commit `d6c07dad02aebcd310317c306e17d7c423075661`: SUCCESS.
- `cargo fmt --all -- --check`: PASS.
- `cargo test --workspace --all-targets`: PASS, including all five testkit synthetic contract tests and existing engine-core coordinate tests.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- Project Memory Validation run 44156319814 was created for the same commit but remained pending with no job assigned at the time of this entry.
Limitations:
- Rust CI validates the fixture contract only; no Java 26.3 oracle, Mojang artifact downloader, comparator or parity fixture exists.
- No Mojang binaries/assets were downloaded or committed.
Next:
- Confirm PMCS validation for this commit, then implement official manifest resolution and temporary server artifact download with strict host/size/SHA-1 verification and unconditional cleanup.


## S-2026-11-17-26 — Implement official Java 26.3 artifact acquisition
Date: 2026-11-17
Commits: downloader `24dcb37d65d68d347229c67d1b3b804a957b20ca`; package/test setup `23b6f618366cd35e6f97cfec29e469ff2f3d5cff`, `5dc11eb210655aa36635d24022d8aff0a5163483`, `1638651c0763b767e403a2b1e27576b6da2ed984`; test fixture hash correction `be3d60060411ddfaf96f699abb88808813d5f86b`; workflows `bdb07db33ba0e94c4b708d57e21192594b3fe327`, `8ccec737f1677bcb7898db180471b1edaa9bb3c8`; documentation synchronization follows.
Goal: Implement official-source artifact acquisition without retaining or publishing Minecraft binaries.
Changes:
- Added standard-library-only `tools/fetch_mojang_server.py` for exact Java 26.3 release resolution via Mojang's v2 manifest and version JSON.
- Validates the manifest-pinned version JSON SHA-1; allows only official Mojang metadata and artifact HTTPS hosts, including redirect targets.
- Streams the official server jar into a caller-provided temporary directory and verifies exact size and SHA-1 before exposing `server.jar`; partial files are removed on failure.
- Added offline Python unit tests for URL validation, credentials/nonstandard ports, exact size and SHA-1 checks.
- Added `.github/workflows/reference-tool-tests.yml` for offline tests and `.github/workflows/reference-artifact-smoke.yml` for manual artifact acquisition, no binary upload/cache, unconditional cleanup and an optional deliberate-failure path.
- Updated reference pipeline spec and PMCS docs. Scope remains vanilla engine only; no modding.
Verification:
- Confirmed official Java 26.3 server metadata values from Mojang-derived metadata references: SHA-1 `33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`, size 62,294,556 bytes, Java runtime major 25. Actual downloader always resolves and verifies live official metadata rather than hardcoding these values.
- Computed the deterministic unit-test fixture SHA-1 for the literal bytes `vanilla` and corrected the expected value.
Not yet verified:
- No GitHub Actions run for the new Python tests/workflow has been inspected yet.
- No Mojang artifact has been downloaded by this session.
- Cleanup on normal and deliberate-failure paths is not yet proven.
- No Java server, oracle, Rust probe or differential comparator is implemented; this workflow is artifact acquisition only and makes no parity claim.
Next:
- Inspect latest main Actions runs; fix offline test issues. Manually dispatch artifact smoke workflow with `simulate_failure=false`, then with `true`, and verify cleanup step success on both. Only then implement the first oracle/probe comparison.


### Verification addendum — 2026-11-17
- The first artifact-workflow commits produced push runs with zero jobs because the workflow used the `runner.temp` context at job-level scope. Removed that unsupported job-level expression and now derives the unique directory from the runner-provided `RUNNER_TEMP` environment inside steps.
- The workflow now explicitly supports push checks (offline tests only) and manual dispatch (actual download). This avoids a no-trigger workflow run and prevents ordinary pushes from downloading Mojang binaries.
- Push run 44162815461 on commit `965287f36d9d8855e9f9f2f8e715c3e61d4ed58e`: SUCCESS. All nine offline Python tests passed; the artifact download was correctly skipped for a push event; unconditional cleanup passed and confirmed the temp directory was absent.
- A manual `workflow_dispatch` could not be initiated through the available GitHub connection (no dispatch action is exposed). Therefore the live Mojang download, actual downloaded-file verification and deliberate-failure cleanup path remain unverified and require a manual run from the repository Actions tab.


## S-2026-11-18-01 — Pivot project to DreamMod
Date: 2026-11-18
Commit: architecture commit 422a9e3fc57ffce0c2e9399a9b7466a1f15a6e2a; memory synchronization commits follow
Goal: Start the user-approved DreamMod project in Rusty-survival- while preserving the existing PMCS continuity system.
Changes:
- User approved using this repository for DreamMod and selected the Horizon-based prebuilt-distribution architecture.
- Added docs/specs/DREAMMOD_ARCHITECTURE.md covering HNMod build-time integration, prebuilt Horizon distribution, interpreted .dmod packages, deep extension points, performance, sandboxing, language evaluation and phases.
- Reframed active project memory, architecture, decisions, issues, roadmap, compatibility matrix and README around DreamMod.
- Marked the previous Minelib/standalone Minecraft Java 26.3 engine direction as superseded/historical. Existing files and history are preserved.
- Kept direct commits to main and retained PMCS append-only session history.
Verification:
- Inspected main commit 279059bda915b2b2d87bc13731d4d7c3d3594286 and relevant PMCS files/validator.
- Added architecture document in commit 422a9e3fc57ffce0c2e9399a9b7466a1f15a6e2a.
- Documentation updates committed directly to main; PMCS Actions validation pending.
Not tested:
- No Horizon source integration, HNMod build, interpreter, loader, API, .dmod, benchmark, sandbox or runtime distribution has been implemented or tested.
- No Rust code changed and no cargo tests were run.
Next:
- Verify PMCS Actions; then inspect and pin current Horizon, horizon-mixins and hooks/API revisions before creating the minimal Core bootstrap.


## S-2026-11-18-02 — Audit Horizon's existing mod platform
Date: 2026-11-18
Commit: audit commit 21ff8bd96672d0304533170781f9f5f8d05d36aa; architecture and memory synchronization commits follow
Goal: Verify the best integration route against current Horizon source instead of duplicating existing mod infrastructure.
Changes:
- Inspected Horizon main 1c45cf95d8431a7b49d94df007f7e75a890370c9 and horizon-mixins e59da9094f0c2fdda6c627fe9eff22dec6a906f0.
- Audited existing mod API/host, package formats, hooks, Hosts, Parts, attachments, capabilities, typed API injection, native mixins/bridges/loop slots and runtime loading.
- Confirmed WASM runtime includes a child-process Wasmtime compilation path; it is unsuitable for the strict no-player-compilation/interpreted-source requirement.
- Confirmed native host can load prebuilt adjacent dynamic libraries with SHA-256 and build-identity checks; end-to-end CI packaging/startup proof remains OPEN.
- Added docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md and revised docs/specs/DREAMMOD_ARCHITECTURE.md to reuse existing Horizon facilities.
- Updated issues, roadmap and AI handoff with the new evidence.
Verification:
- Source/documentation inspection only; pinned files and loader behavior are recorded in the audit.
Not tested:
- No Horizon build, native .hmod build, prebuilt library package, clean install/startup, interpreter, sandbox or benchmark.
Next:
- Prove a CI-built native .hmod can be shipped as prebuilt files and loaded by a clean pinned Horizon distribution without invoking launcher compilation.


## S-2026-11-18-03 — Add minimal DreamMod Core native bootstrap
Date: 2026-11-18
Commit: this atomic change set; resolve exact SHA from Git history
Goal: Establish the smallest compile-checkable native Core integration before implementing the interpreted runtime.
Changes:
- Added mods/dreammod-core/Cargo.toml, horizon-mod.toml and src/lib.rs as a minimal native .hmod project using Horizon's typed Hooks API.
- Pinned Horizon 1c45cf95d8431a7b49d94df007f7e75a890370c9 and horizon-mixins e59da9094f0c2fdda6c627fe9eff22dec6a906f0.
- Added .github/workflows/dreammod-core.yml to compile the package against the exact Horizon source checkout; CI rewrites only its temporary copy to use the checkout's local API crate.
- Updated current state, issues, roadmap and changelog in the same atomic commit.
Verification: source and workflow committed atomically; CI execution pending.
Not tested: no successful compile yet; no native library, prebuilt distribution, interpreter, .dmod loader, sandbox or benchmarks.
Next: get the Core workflow green, then prove prebuilt native library installation without launcher-side compilation.


## S-2026-11-18-04 — Add interpreted runtime candidate prototype
Date: 2026-11-18
Commits:
- Runtime crate: b94a8aebeef1774c288dd6156fc480ce64d788c2
- Runtime implementation/tests: f07c8cef189bf4e41e03b9bf067ea36c59b139d9
- Benchmark example: 90dbc7bb140c572556d88bcd3e5ef3b6d467ea95
- Runtime CI workflow: ce30c8a3de65f209bcf7f65beded3978baf7e105
- Architecture/status/memory synchronization commits follow.
Goal: Start evidence-based evaluation of source-interpreted runtime candidates without selecting a language prematurely.
Changes:
- Added an isolated Rhai 1.27.0 prototype using the AST evaluator. The optional Grain bytecode feature is not enabled and AST optimization is disabled.
- Added tests for event-style function dispatch, one coarse-grained world-health batch call, one script-side batched physics algorithm, and operation-budget termination of an infinite loop.
- Added a coarse benchmark comparing the interpreted batch algorithm with a Rust-native equivalent. Desktop CI timings are diagnostic only and do not establish Android performance.
- Added a dedicated GitHub Actions workflow for tests and benchmark.
- Corrected project architecture/current-state records so they no longer claim that no DreamMod code exists.
Verification:
- Existing Core workflow run 44249522292: SUCCESS. The pinned Horizon API cargo check completed successfully.
- Runtime prototype CI workflow: pending at the time of this log entry.
Not tested:
- No final scripting-language decision.
- No Starlark comparison or ARM64 device benchmark.
- No production .dmod package parser/loader, permissions enforcement, sandbox, lifecycle, hot reload or DreamMod Core integration.
- No prebuilt native library inventory/package or clean Horizon game launch.
Next:
- Inspect runtime prototype Actions results and fix failures.
- Compare Starlark using equivalent event, batch and subsystem workloads.
- Prove the native prebuilt library route separately before claiming launcher installs can avoid local compilation.

## S-2026-11-18-05 — Verify API-only prebuilt route and correct CI findings
Date: 2026-11-18
Goal: Confirm the exact Horizon route for shipping DreamMod Core prebuilt and correct failures found by automated verification.
Changes:
- Traced pinned Horizon source from native_mods::pipeline through prebuilt::compile and Route::Api into api_route::compile.
- Confirmed API-only mods are compiled as separate dynamic libraries against the prebuilt SDK runtime/API slices; api_route returns the existing horizon-client executable unchanged plus runtime and mod libraries.
- Confirmed the native host reads native-mods.json, validates adjacent library names, SHA-256, game build identity and Rust compiler identity, then loads exported native mod entries.
- Confirmed current DreamMod Core uses Hooks only and has no native source patch/internals dependency, so it should use Route::Api and avoid rebuilding Horizon's engine. This is source-level evidence; artifact creation and clean startup remain pending.
- Corrected the Rhai prototype after CI identified an optimizer import hidden by the no_optimize feature, and applied rustfmt output. Added rustfmt/clippy toolchain components to CI.
- Corrected the PMCS append-only validator's false positive when appending to a legacy file whose previous final line had no newline.
Verification:
- PMCS regression run 44277791784: SUCCESS after the validator correction.
- Initial runtime candidate CI failures were inspected: one compile error due to importing a feature-gated OptimizationLevel, and one rustfmt mismatch. Fixes committed; fresh runtime workflow run is pending.
- Existing Core pinned Horizon API compile run 44249522292 remains SUCCESS.
Not tested:
- No successful Rhai runtime test/benchmark result yet.
- No Starlark comparison or ARM64 runtime benchmark.
- No official native-build output artifact, native-mods.json package, or clean Horizon client launch.
- No production .dmod loader, sandbox, or Core integration.
Next:
- Verify the current runtime prototype and PMCS workflows.
- Create a reproducible CI build using Horizon's official API-only native route and inspect the complete version folder/inventory.
- Test that a clean pinned Horizon install starts with the shipped Core libraries without running native-build on the player device.

## S-2026-11-18-06 — Correct Rhai host boundary and rerun candidate checks
Date: 2026-11-18
Commits:
- Host boundary conversion: 961825d425a3621a0b2886d480458549b5495e6c
- Import ordering correction: 92c3508b709e74ee0487644e60f0a3ff80e1a108
Goal: Fix the first runtime prototype CI behavior failures and rerun the candidate validation pipeline.
Changes:
- Converted Rust Vec<f64> arguments into Rhai Array values before calling interpreted script functions, and converted returned Rhai Array values back to Vec<f64>.
- Changed the script loop to use Rhai's array length method.
- Inspected workflow logs: the earlier behavior run 44278072893 failed because Vec<f64> was passed as a Rust custom type rather than a Rhai array; `len` was consequently unavailable.
- The first post-fix workflow run 44283921499 stopped at rustfmt because the new import order did not match rustfmt. Corrected import ordering in 92c3508.
Verification:
- `cargo fmt --check` passed before the import-order change in the earlier run but failed on the new import order in 44283921499; corrected, fresh workflow is required.
- The fixed behavior tests have not yet completed successfully in CI.
- PMCS validator regression tests passed in the run before the latest memory sync; the structural/freshness check flagged missing memory updates for the runtime code change. This session entry and state sync record the change, and PMCS must be checked again.
Not tested:
- No successful post-fix test suite, Clippy run or benchmark yet.
- No Starlark comparison or ARM64 benchmark.
- No production .dmod loader, sandbox or Core integration.
- No CI-built prebuilt Horizon distribution or clean game startup.
Next:
- Verify the latest runtime and PMCS workflows.
- Resolve remaining runtime test/Clippy issues before making any language decision.
- Continue the reproducible native-build artifact and clean startup proof.

## S-2026-11-18-07 — Verify Rhai tests and correct benchmark methodology
Date: 2026-11-18
Commits:
- Benchmark anti-optimization correction: 89ed3a56bc209431b860811340c8c7446d7e65ad
Goal: Verify the interpreted runtime candidate after the host-boundary fix and ensure its benchmark measures actual work.
Changes:
- Confirmed Actions run 44283982705 completed successfully: formatting passed, 4 tests passed (event dispatch, batched host query, batched physics algorithm, infinite-loop operation budget), and Clippy passed with warnings denied.
- Inspected benchmark output and rejected its apparent 1,070,530x interpreted/native ratio. The Rust-native path took only 371 ns for 500,000 output values because the optimizer eliminated unused output calculations.
- Added std::hint::black_box around benchmark inputs and outputs so both implementations must materialize observable results; corrected the script's len() call.
Verification:
- Runtime tests and Clippy: VERIFIED in Actions run 44283982705.
- Benchmark anti-optimization correction: IMPLEMENTED in commit 89ed3a5; fresh CI timing pending.
- No performance conclusion is drawn from the invalid previous measurement.
Not tested:
- No ARM64/Android benchmark.
- No Starlark comparison or final language choice.
- No production .dmod loader, sandbox, Core integration or native prebuilt distribution.
Next:
- Inspect the corrected benchmark run and ensure the two sides do equivalent observable work.
- Add Starlark as a competing interpreted-source candidate and compare with the same host boundary/workloads.
- Continue native API mod packaging and CI-built prebuilt distribution proof.

## S-2026-11-18-08 — Finish benchmark formatting correction
Date: 2026-11-18
Commit: 00a3360bd33d53c54ec75543da83798e0fdc9c30
Changes:
- Applied rustfmt's exact line wrapping to the black_box-protected benchmark call after CI run 44284293061 stopped at formatting.
Verification:
- Fresh runtime workflow 44284388165 and PMCS workflow 44284388228 are queued; no test or benchmark result is claimed yet.
Next:
- Inspect the corrected runtime CI run and PMCS validation.
- Once benchmark is valid, compare another AST-interpreted candidate and begin native trait mod package integration.

## S-2026-11-18-09 — Convert Core bootstrap to an official Horizon native trait package
Date: 2026-11-18
Commits:
- Native manifest pin and metadata: e6af375efe89e0a265997f45da6c100612fadd33
- Official check/package workflow: b58f0d94ade9f74ac92b4ddb0e8292d140a5d1de
- YAML quoting correction: cc5c841945c13381f4d0ea67f925d83cce8a1f66
Goal: Move from a bare Hooks API compile skeleton to a correctly described native trait mod package and verify it with Horizon's own tooling.
Changes:
- Added/updated `horizon-mod.toml` with `api = 0.2`, `capabilities = []`, `side = both`, `game = 26.3`, Horizon Nightly 2026-11-18 pin `0b14f8acfd97fa2c750e42795249dcc981178ca4`, and horizon-mixins pin `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.
- Removed the unrelated `registries` capability from DreamMod Core; the bootstrap is not a content registry mod.
- Verified current Nightly source still has the API-only Route::Api implementation and native library loader.
- Reworked the Core workflow to install the pinned `cargo-horizon-mod` tool from the Horizon source checkout, install wasm32, run official package/source/API checks for host and wasm32, build the source .hmod and upload it as a temporary CI artifact.
- The first workflow edit had invalid YAML quoting and produced a failed run with no jobs; corrected the quoting in commit cc5c841.
Verification:
- The new official Core workflow run 44284751206 is in progress at the time of this entry; no package check or artifact success is claimed yet.
- The earlier direct source API cargo check (run 44249522292) is historical evidence only and used the previous source pin.
- The current pinned Nightly source still exposes the API-only route in source; this does not prove the prebuilt native distribution yet.
Not tested:
- No completed official `cargo horizon-mod check` or source .hmod artifact yet.
- No CI-built prebuilt native distribution or clean game startup.
- No production .dmod loader or sandbox.
Next:
- Inspect run 44284751206 and fix package/toolchain issues.
- Inspect the uploaded .hmod source artifact and build it through the official native-build/prebuilt API route.
- Verify that a clean pinned Horizon game starts with DreamMod Core without player-side compilation.

## S-2026-11-18-10 — Correct the interpreter shortlist for strict source interpretation
Date: 2026-11-18
Goal: Ensure every language candidate truly satisfies the user's no-bytecode/no-player-compilation requirement.
Changes:
- Inspected Starlark 0.14 documentation and implementation descriptions: although its public API parses source to AstModule, its execution pipeline compiles the AST to bytecode before the Evaluator runs it. Starlark is therefore rejected for this strict requirement.
- Identified Sprite as a small Rust tree-walking interpreter with no bytecode/VM, but its current language has only a small set of value types and no substantial standard library. It is research-only and not a production candidate without significant expansion.
- Updated the architecture, roadmap and handoff so they no longer describe Starlark as an AST-walking runtime candidate.
Verification:
- Rhai CI run 44283982705 passed formatting, four behavior/budget tests and Clippy.
- The newest benchmark run 44284795812 is still pending; it now adds equal input cloning and observable output checks after the earlier benchmark showed that compiler optimization had removed the Rust-side work.
Not tested:
- No final language selection.
- No Android/ARM64 benchmark.
- No production .dmod loader, sandbox or Core integration.
Next:
- Verify the corrected benchmark and decide whether Rhai's interpreted performance is acceptable for the intended extension contracts.
- Inspect the official Core package workflow and its .hmod artifact.
- Continue prebuilt API-only native distribution proof.

## S-2026-11-18-11 — Record comparable interpreted/native benchmark
Date: 2026-11-18
Goal: Establish a fair first performance signal for the strict AST-interpreted candidate.
Changes:
- Updated the benchmark so both the Rhai and Rust paths clone equivalent 10,000-element input vectors, materialize 10,000 outputs per round and consume an output checksum through black_box.
- The Rhai side includes the public API boundary conversion to/from Rhai arrays; the Rust side uses typed vectors. Both execute the same per-item position + velocity * delta operation.
Verification:
- Actions run 44284795812: SUCCESS. 50 rounds × 10,000 items: Rhai AST total 265.955429 ms; Rust native total 574.49 µs; ratio 462.94×.
- This is a synthetic x86_64 GitHub-hosted runner result, not an Android/ARM64 measurement or a full-game benchmark.
Interpretation:
- The measured gap is too large to recommend interpreted per-entity/per-vertex loops in hot physics/rendering paths.
- Keep engine hot loops native and investigate coarse subsystem provider boundaries, where scripts make a small number of large-grained decisions or invoke batched native operations.
- Do not select Rhai as final solely because the functional tests pass; benchmark on ARM64 and with realistic host API traffic.
Not tested:
- No ARM64/Android benchmark.
- No final language selection or production runtime integration.
- No production .dmod loader, sandbox or Core integration.
Next:
- Inspect and complete the official native Core package workflow.
- Verify prebuilt native build distribution and clean startup.
- Prototype a coarse provider contract and test whether interpreted code can remain out of per-item hot loops.

## S-2026-11-18-12 — Add resource limits to the interpreted runtime candidate
Date: 2026-11-18
Commit: b6b1f32f81fb4b9b486ac59c375a71b622473bb3
Goal: Strengthen the evaluation-only Rhai harness with explicit resource bounds before considering production sandbox design.
Changes:
- Added RuntimeLimits fields for max aggregate array items, map entries, string bytes, variables and functions.
- Applied Rhai Engine limits for array size, map size, string size, variable count and function count, alongside the existing operation and call-depth limits.
- Added a regression test that attempts to grow an array beyond its configured budget and expects execution to fail.
Verification:
- The code commit's PMCS check 44285379097 failed because the implementation change did not include the required five memory documents in the same base diff; this entry and synchronized memory updates are now being added. The PMCS regression tests themselves passed.
- Runtime prototype workflow 44285379104 is in progress; no passing result is claimed yet.
Not implemented:
- This prototype does not provide exact byte-level heap accounting, per-mod memory isolation, filesystem/network capabilities, cancellation across threads or production fault containment.
Next:
- Verify the resource-budget regression and Clippy in CI.
- Keep the 462.94x desktop interpreted/native result as a warning against hot per-item loops; prototype a coarse provider boundary.
- Continue the official native-build pipeline and prebuilt distribution proof.

## S-2026-11-18-13 — Correct required native capability and optimize Core CI
Date: 2026-11-18
Commits:
- Manifest capability correction: 9a5f9b5abe23ce0648840ded5f3b5e31f1eea57d
- Cached Horizon tool build: 1747cd5728fa7189e1ec8db105953333b43eed6e
Goal: Fix the official native package validation failure and make repeated CI runs cheaper.
Changes:
- Inspected the current Horizon native template and Manifest validator. The official native template always includes `capabilities = ["registries"]`; the current validator rejects an empty list even when `[native]` is present. DreamMod Core now includes this required capability. Treat it as a real permission in the future trust UI; do not claim the Core requests no permissions.
- Replaced `cargo install --path` with a cached workspace `cargo build --release -p cargo-horizon-mod` and added `Swatinem/rust-cache` scoped to the pinned Horizon checkout.
Verification:
- Official Core workflow 44284751206 successfully built and installed the pinned `cargo-horizon-mod` tool, then failed at manifest validation with `a content mod has capabilities = ["registries"]`. The manifest fix is committed; fresh official validation/build is pending.
- Runtime resource-budget tests, formatting and Clippy passed in run 44285379104.
- PMCS validator regression tests and structural validation passed in run 44285628749 before these latest changes; the new five-document memory update is being recorded after the implementation commit.
Not tested:
- No successful official package check or .hmod source artifact yet.
- No native-build distribution, artifact inspection or clean game startup.
Next:
- Verify the new Core workflow after the manifest fix.
- Run the official `horizon-launcher native-build` command in CI against the published Nightly SDK and upload the resulting native build folder.
- Prove a clean client/host startup and inspect the native library inventory.

## S-2026-11-18-14 — Add official Horizon prebuilt build workflow
Date: 2026-11-18
Commits:
- Initial workflow: 2ae2738494e6c5f2897d4acf570db2c003a2d7f9
- Pin/inventory verification: 01c696c86084991fceb9978cc9ae2cd24443312b
- Checksum filename correction: f827ab905a7e664c9d75d791b03034cf76773e75
- Headless shared-library dependencies: 6bd45dd533224b11bbfb2a59ee58a7481c8272cf
Goal: Use the official Horizon Launcher in trusted CI to create the prebuilt native build overlay that the user's separate launcher can distribute.
Changes:
- Added a workflow pinned to Horizon Launcher 0.2.78. It downloads the Linux x86_64 tarball, verifies its SHA256SUMS entry, isolates HOME/XDG data, installs Nightly, creates an isolated instance and invokes `horizon-launcher native-build mods/dreammod-core --instance DreamModCI --trust-native-code --profile optimized`.
- The workflow inspects native-mods.json, verifies the DreamMod Core ID and exact pinned Horizon commit, requires horizon-client and an adjacent native dynamic library, then archives the versioned native build overlay for 14 days.
- The initial checksum check failed because sha256sum expected the original asset filename while the download was saved as launcher.tar.gz; corrected the checksum line with sed.
- The next run found the launcher binary needed libxkbcommon-x11.so.0; added the relevant X11/XKB shared libraries and offscreen Qt mode for the headless runner.
Verification:
- Prebuilt workflow run 44286113386 failed at checksum verification; fixed in 01c696c.
- Run 44286212788 passed download/checksum/extraction and HOME isolation, then failed at launcher startup due to missing libxkbcommon-x11.so.0; fix committed in 6bd45dd, fresh run pending.
- Official Core package workflow run 44284751206 built the cargo-horizon-mod tool successfully but failed on the initial manifest's empty capabilities list. Current Horizon native template requires `capabilities = ["registries"]`; fixed in 9a5f9b5. Cached-tool rerun 44285877944 is in progress.
Not tested:
- No successful native-build output or uploaded prebuilt overlay yet.
- No clean game or server startup with DreamMod Core.
Next:
- Verify the cached official package workflow and source .hmod artifact.
- Verify the headless official launcher can install Nightly and execute native-build.
- Inspect the actual output layout and adjust artifact packaging if the library/inventory is not adjacent to horizon-client.
- Prove clean startup with a host process that loads the native library.

## S-2026-11-18-15 — Define initial provider performance boundary
Date: 2026-11-18
Goal: Convert the measured interpreter performance gap into a concrete architecture constraint without prematurely abandoning source interpretation.
Changes:
- Updated DreamMod architecture to require coarse subsystem/event provider calls and keep high-frequency entity/vertex iteration and batch storage in Rust.
- Recorded the current synthetic x86_64 benchmark: 265.955 ms Rhai AST vs 574.49 µs Rust for 50 × 10,000 items (462.94×). This is diagnostic only, not Android/ARM64 data.
- Added a future explicit decision point for an optional CI-prebuilt native kernel tier for hot subsystem replacement. It would avoid player-side compilation but is not implemented and is separate from interpreted `.dmod` scripts.
Next:
- Finish the official Core source-package check and native-build workflow.
- Design a provider interface and benchmark realistic event/batch traffic on ARM64 before committing to the final runtime.

## S-2026-11-18-16 — Switch prebuilt proof to account-free server export
Date: 2026-11-18
Goal: Keep the official prebuilt native build proof reproducible without handling Minecraft account credentials or bypassing ownership checks.
Changes:
- Verified that `horizon-launcher install nightly` installs exact tag `nightly-2026-11-18-0b14f8a` and that an instance can be pinned to that exact tag.
- The client native-build path refuses an unprepared instance. `horizon-launcher prepare` then refused because the isolated CI HOME has no Minecraft 26.3 installation and no authenticated owning account. This is expected ownership behavior; do not bypass it or download/redistribute Mojang files.
- Reworked the CI build to use the official server export mode: `horizon-launcher native-build mods/dreammod-core --server-output <folder> --version nightly-2026-11-18-0b14f8a --trust-native-code --profile optimized`.
- The server export is standalone and should contain `horizon-engine`, `native-mods.json` and the DreamMod Core native library. It proves the API-only native build path without an account, but not a client install.
- Renamed the CI artifact to `dreammod-horizon-nightly-native-server`.
Verification:
- Client instance native-build failed because the version was not prepared; `prepare` failed because Minecraft 26.3 was not present and no owning account was signed in.
- Account-free server-export workflow is pending; no native library artifact is claimed yet.
Next:
- Verify the server export completes and inspect its exact folder layout/inventory.
- If the library is present, run a headless engine smoke test where possible.
- Keep client overlay gated on a prepared licensed Minecraft 26.3 directory; never add credentials or bypass ownership checks.

## S-2026-11-18-17 — Build client overlay from signed SDK slices and test the real host loader
Date: 2026-11-18
Commit: ea9d81551aa6a43b35bc1e6bed05746fd8fef759
Goal: Produce the API-only client engine/native-library overlay without downloading Mojang assets or bypassing Horizon's ownership check.
Changes:
- Confirmed that both client-instance and standalone server-export `horizon-launcher native-build` paths require a prepared Minecraft 26.3 installation. `prepare` correctly refuses without an authenticated owning account. No attempt will be made to bypass this or redistribute Mojang assets.
- Added `scripts/ci/patch_horizon_api_route.py`, a test-only patcher for an ephemeral checkout of the exact Horizon Nightly source. It appends a test module to `launcher/src/native_mods.rs` that uses Horizon's own signed `sdk_cache::index` and `sdk_cache::slices`, then calls the exact `prebuilt::compile(..., Route::Api, ...)` function. It copies the prebuilt `horizon-client`, all adjacent runtime/mod libraries, and writes the native inventory with Horizon's `library_record::collect`.
- The patcher also adds a test to `crates/horizon-mod-host/src/native_libraries.rs` that calls the real `load` function on the generated overlay and asserts exactly one DreamMod Core native entry is loaded. This validates library identity, hashes, exports and runtime dependencies without needing Mojang game assets.
- Replaced the previous launcher CLI/server export workflow with this signed-SDK client overlay build and real host-loader smoke test. Artifact explicitly records that it contains no Mojang assets.
Verification:
- Official Core source package workflow 44285877944: SUCCESS; generated `.hmod` source package was uploaded and inspected. It contains `horizon-mod.toml`, `Cargo.toml`, `src/lib.rs`, and `uses.json`; manifest pins Horizon 0b14f8a and required registries capability.
- New API-route overlay workflow 44288437986 is in progress. No client overlay artifact or host-loader pass is claimed yet.
Not tested:
- No complete playable game launch; Mojang assets remain with the licensed user's installation.
- No ARM64/Android build of the overlay.
Next:
- Verify the test-only harness compiles and the signed SDK slices are fetched.
- Fix any Rust API/type or loader issues found by CI.
- Once the overlay passes the host loader, upload it and document how the user's launcher applies it to a matching base version.

## S-2026-11-18-DM-BATCH-GUARD — Harden script provider boundary
Date: 2026-11-18
Commit: 33639ee0d89627b7ad3342d694a2d26c1ff43fd5
Intent: prevent malformed or non-finite script results from crossing the prototype physics batch boundary.
Files: mods/dreammod-runtime-prototype/src/lib.rs; PMCS CHANGELOG, CURRENT_STATE, ISSUES, ROADMAP and SESSION_LOG.
Implementation: reject unequal input lengths, batches above 100,000 items, non-finite inputs, wrong-sized provider output, non-numeric output and non-finite output. Added focused negative regression tests.
Verification: push to main triggers the existing runtime prototype workflow and PMCS workflow. Results were not available for inspection at the time of this entry; do not mark CI verified until confirmed.
Risks/limits: prototype-only validation; the 100,000 item cap is a test boundary, not a production memory quota. No aggregate heap accounting, package sandbox or Core integration.
Next: inspect Actions; then move toward a standalone runtime crate and a versioned manifest/permissions contract.


## S-2026-11-18-18 — Verify source package artifact and correct CI/runtime status
Date: 2026-11-18
Goal: Synchronize project memory with actual artifacts and the current signed-SDK overlay test.
Changes:
- Downloaded and inspected Actions artifact 14419047604 from run 44285877944. It contains dreammod_core.hmod; the package contains horizon-mod.toml, Cargo.toml, src/lib.rs and uses.json. The manifest pins Horizon 0b14f8a, API 0.2, game 26.3 and the required registries capability.
- Corrected memory status: the official pinned cargo-horizon-mod check and source .hmod package build are VERIFIED; the native dylib overlay and real host-loader test are still in progress.
- The direct API-route harness calls Horizon's signed SDK cache/index/slice functions, prebuilt::compile(..., Route::Api, ...), library_record::collect, and the actual horizon-mod-host::native_libraries::load test. It produces a client engine/native-library overlay without Mojang assets; a complete playable install still needs a matching licensed base installation.
- Fixed the first harness attempt's Rustup issue by preserving the runner's original HOME and isolating only XDG data/config/cache paths (commit 67bc587). The new run 44288685182 is in progress.
- Corrected the documented scope of Rhai resource limits: max array/map/string sizes are per-value caps, with operation/call-depth/variable/function limits. They are not aggregate per-mod heap quotas.
Verification:
- Core source package workflow 44285877944: SUCCESS; .hmod artifact downloaded and inspected.
- Runtime candidate behavior/Clippy and per-value budget tests: PASS in 44284795812 and 44285379104.
- Current API-only prebuilt overlay workflow 44288685182: IN_PROGRESS; no overlay or host-loader success is claimed.
Next:
- Verify the direct API-route build and host-loader test; fix any Rust type/visibility or dynamic-library issues.
- Once the overlay passes, inspect the artifact and record its exact hash/build identity.
- Implement the production .dmod package schema/loader and interpreter abstraction as a separate runtime crate; keep Rhai provisional and preserve the no-player-compilation rule.

## S-2026-11-18-19 — Fix prebuilt workspace invocation and runtime formatting
Date: 2026-11-18
Goal: Correct CI failures without weakening verification.
Changes:
- The API-only overlay run 44288685182 failed because Horizon's launcher is a separate Cargo workspace; the root workspace has no package named horizon-launcher. Updated the workflow to run cargo test from .ci/horizon/launcher and cache both launcher/root workspaces (commit 7a7cdb0).
- The runtime prototype workflow 44289702282 caught rustfmt differences in the newly added provider batch validation. Applied rustfmt-equivalent line wrapping in commit 99be124.
Verification:
- PMCS validator regression tests passed in run 44289964314, but base-diff validation failed because the code-only formatting commit 99be124 was not accompanied by fresh memory docs relative to that run's base. This session entry and required memory files synchronize the record.
- New prebuilt overlay run 44289897914 is in progress.
- New runtime prototype run 44289964423 is in progress.
Next:
- Inspect run 44289897914 and fix any remaining compiler/test harness errors.
- Inspect run 44289964423 and ensure formatting, tests and Clippy pass.

## S-2026-11-18-01 — Implement the first DreamMod manifest contract
Date: 2026-11-18
Commit: 8923ce6444b3a9c491c6a66c0f431852f5d4cfcd (workflow adjustment; crate commits immediately precede it)
Goal: Establish a versioned, strict `.dmod` manifest boundary before archive loading or script execution.
Changes:
- Added standalone `mods/dreammod-package` crate with version-1 manifest parsing and validation.
- Added checks for unknown fields/schema, mod IDs, semantic versions/API ranges, safe relative entrypoint paths, explicit capability allowlist/duplicates, dependency declarations and input/count limits.
- Added dedicated package-contract GitHub Actions workflow for formatting, tests and Clippy; removed `--locked` because this initial crate has no committed Cargo.lock yet.
Verification:
- GitHub files and commit responses confirm the crate and workflow were published to `main`.
- CI has not been inspected; tests and Clippy are PENDING, not VERIFIED.
Limitations:
- No ZIP/archive parsing or extraction, entrypoint existence check, capability granting, dependency resolver, runtime lifecycle, script execution or production sandbox.
Next:
- Inspect package-contract CI. Fix all failures before adding archive handling; then design a bounded, non-extracting ZIP inventory validator.


## S-2026-11-18-20 — Fix Rhai provider error conversion
Date: 2026-11-18
Goal: Correct the type-check failure found by the runtime prototype workflow.
Changes:
- Runtime workflow 44289964423 passed rustfmt but failed compilation at the new physics provider result type check because .into() was ambiguous between several conversions to Box<EvalAltResult>.
- Replaced the ambiguous conversion with explicit Box::new(EvalAltResult::ErrorMismatchDataType(...)) in commit 5bdd485.
Verification:
- No follow-up test result yet; a new workflow was triggered by 5bdd485.
Next:
- Verify runtime tests, Clippy and benchmark after the explicit error conversion.
- Continue monitoring API-only prebuilt overlay run 44289897914.

## S-2026-11-18-02 — Add pre-extraction archive inventory policy
Date: 2026-11-18
Commit: 24bb7904b46671f74c90b1a1ed11b8b8c496fd67 (directory-root hardening; validator introduced in 93772ffc8df18d60da045c299175e97b3234bfe7)
Goal: Define resource and path limits before a future ZIP reader opens untrusted package entries.
Changes:
- Added a metadata-only archive inventory validator to `dreammod-package`.
- Enforced strict portable paths, approved roots, case-folded duplicate detection, entry/file/aggregate size limits, compression ratio limits, and rejection of symlink/unsupported/encrypted entries.
- Required exactly one root `manifest.json` and the declared script entrypoint to be present in the inventory.
Verification:
- Changes were committed to `main`; dedicated GitHub Actions result has not been inspected and is PENDING.
Limitations:
- No ZIP parser, local-header/central-directory consistency check, CRC validation, extraction, package discovery, or real archive fixture tests yet. This policy is only as trustworthy as the metadata supplied by a future parser.
Next:
- Verify CI, then integrate a ZIP reader with strict archive-structure checks and fuzzable adversarial fixtures. Do not extract files before all validation passes.


## S-2026-11-18-03 — Deterministic dependency planner
Date: 2026-11-18
Commit: 1817a3b4b9a3f4ae974793c2d46c9cbce65b3fe6
Goal: Plan a safe, repeatable dependency-first load order before adding runtime lifecycle behavior.
Changes:
- Added a pure dependency planner with an installed-mod count limit.
- Rejects duplicate IDs, missing dependencies, semver incompatibilities and cycles.
- Uses a lexicographically ordered ready set to keep load order deterministic.
- Added tests for ordering, missing/incompatible dependencies, cycles and duplicate IDs.
Verification:
- Source committed to `main`; CI has not yet been inspected. Test status remains PENDING.
Limitations:
- This does not parse archives, install packages, resolve optional dependencies, execute scripts or grant capabilities.
Next:
- Verify the package workflow; then implement a real ZIP reader with central-directory/local-header checks and no filesystem extraction.


## S-2026-11-18-04 — Real ZIP reader and bounded package inspection
Date: 2026-11-18
Commits: 32d52810bb2181c7c6bb03a4f7152b402a8c8d9e, d7e7794567efb0cf1873686964fe47e7364ccbf4, 8bea071635acc45098bd4e93c5c66cbdf5296b30, 15c650a0e61ed3707e6ff76f784180f7f9115452, fc6af40387fa9b861b19ec7fb6b148046c83df8d, 6add01bb156b8ff0e826de40dbd7712a014daf41
Goal: Replace metadata-only archive policy with actual safe inspection of untrusted .dmod ZIP bytes.
Changes:
- Pinned zip 9.6.1 with only Deflate support enabled.
- Added inspect_dmod() with compressed input limit, archive structural checks, strict UTF-8 path handling, compression allowlist, metadata preflight and no extraction.
- Enforced local-header/central-directory consistency before reading entries.
- Parsed manifest from the archive, checked its entrypoint against the inventory, and streamed all regular entries to EOF in fixed-size chunks to verify actual sizes and CRC.
- Added real ZIP fixtures for success, traversal, truncation/non-ZIP and payload corruption.
Verification:
- Commits are on main. CI has not yet been inspected; status remains PENDING.
Limitations:
- Inspection does not install files, grant capabilities, execute scripts or provide a production sandbox.
Next:
- Verify the package workflow. Fix compiler/format/lint issues before continuing to package installation and lifecycle.


## S-2026-11-18-01 — Verify and repair DreamMod package CI
Date: 2026-11-18
Commit: 4e983c04ea8f3a4085aa2e13b6b53d691c307564 (package implementation); PMCS synchronization follows.
Goal: Verify the in-memory .dmod ZIP inspector and correct the issues exposed by CI.
Changes:
- Fixed the Windows-backslash path test fixture by escaping the backslash in JSON before parsing.
- Simplified the archive path validator to satisfy Clippy without suppressing the lint.
- Corrected stale source comments that still described the crate as manifest-only/metadata-only.
Verification:
- First rerun: all 19 unit tests passed; Clippy exposed one needless-bool lint.
- Final Actions run 44299317538: formatting step SUCCESS, 19/19 unit tests SUCCESS, Clippy with -D warnings SUCCESS.
Limitations:
- This verifies in-memory ZIP inspection and validation only. No files are extracted or installed; package contents are not yet retained for runtime use; no capabilities are granted and no script is executed.
Next:
- Add a bounded in-memory package-content representation that retains validated script/assets bytes, then design explicit install/permission/lifecycle boundaries before connecting an interpreter.
