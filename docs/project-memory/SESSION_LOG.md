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


## S-2026-11-18-21 — Make session ID validation legacy-aware
Date: 2026-11-18
Commit: pending
Goal: Repair PMCS validation without rewriting the append-only historical session journal.
Changes:
- Confirmed the existing historical journal already contains repeated IDs from earlier sessions; the newly appended package-verification entry also reused an old ID.
- Updated the validator to preserve legacy history but require every newly appended session ID to be unique against the entire prior journal and within the current update.
- Added regression tests for duplicate IDs in one update and reuse of a legacy ID.
- Kept the existing append-only diff check intact.
Verification:
- GitHub Actions PMCS regression run 44299535756 exposed the duplicate-ID issue; the fix and added tests are pending CI.
Limitations:
- Historical duplicate headings remain unchanged to preserve the append-only journal. New entries must use unique IDs.
Next:
- Verify PMCS regression tests and full structure/freshness validation, then resume the DreamMod package-content loader.


## S-2026-11-18-22 — Retain validated package files in memory
Date: 2026-11-18
Commit: 2dd019b4a370fec853ec19849c4538f6cce9be44
Goal: Make the verified .dmod inspector return usable script and asset contents without extracting them to disk.
Changes:
- Added PackageFile and a files collection to ValidatedPackage.
- Retain regular files under the already-approved scripts/ and assets/ roots only after streaming to EOF, validating actual decompressed length and CRC.
- Keep manifest.json as parsed metadata rather than duplicate retained payload; drop its temporary byte buffer after parsing.
- Added fixture assertions for exact script and asset bytes and updated the crate description.
Verification:
- Package workflow pending for this change.
Limitations:
- The package bytes are held in memory only; there is no install path, capability grant, interpreter execution or production sandbox.
Next:
- Verify package CI, then define the trust/permission boundary before any script can execute.


## S-2026-11-18-23 — Verify retained package contents and PMCS migration
Date: 2026-11-18
Commit: 2dd019b4a370fec853ec19849c4538f6cce9be44 (package code); PMCS updates follow.
Goal: Verify that validated .dmod payload bytes are retained safely and project continuity checks remain green.
Changes:
- Confirmed the package inspector returns the exact script and asset bytes in memory after ZIP validation.
- Updated current state, roadmap, issues and changelog with the package verification result.
- Confirmed the legacy-aware session ID migration preserves append-only history while rejecting duplicate/reused new IDs.
Verification:
- DreamMod package Actions run 44300015439: formatting SUCCESS, 19/19 tests SUCCESS, Clippy SUCCESS.
- PMCS Actions run 44300145443: regression tests SUCCESS and structure/freshness validation SUCCESS.
Limitations:
- No filesystem installation, permission grant, script execution or production sandbox.
Next:
- Define the package trust/permission boundary and a pluggable runtime input contract before implementing execution.


## S-2026-11-18-24 — Add explicit default-deny capability planning
Date: 2026-11-18
Commit: d83b3491b747bc880b657e4cf17e259a8af2ff62
Goal: Separate manifest capability requests from explicit user approvals before any runtime integration.
Changes:
- Added CapabilityPlan and plan_capability_grants().
- Empty approvals grant nothing; requested-but-unapproved capabilities are denied.
- Reject unknown, unrequested and duplicate approval values.
- Added regression tests for default-deny, partial grants and invalid approvals.
Verification:
- Dedicated package workflow triggered; result pending.
Limitations:
- The planner returns metadata only. No permission UI, runtime host enforcement, sandbox or script execution exists.
Next:
- Verify CI, then define the host-side capability gate and package lifecycle state machine before connecting an interpreter.


## S-2026-11-18-25 — Verify explicit capability planning
Date: 2026-11-18
Commit: d83b3491b747bc880b657e4cf17e259a8af2ff62
Goal: Verify the pure default-deny capability planner before runtime host integration.
Changes:
- Recorded the verified capability-planning API and its enforcement limitation in project memory.
Verification:
- DreamMod package Actions run 44300324687: formatting SUCCESS, 21/21 tests SUCCESS, Clippy SUCCESS.
- Tests cover no approvals, partial approvals, unrequested/unknown capabilities and duplicate approvals.
Limitations:
- No user-facing permission prompt, runtime capability enforcement or sandbox exists.
Next:
- Design host-side capability checks and a lifecycle state machine without executing untrusted scripts prematurely.


## S-2026-11-18-26 — Add default-deny host capability gate
Date: 2026-11-18
Commit: 5085971c93c3dbc2a0089e136e5f9a7aecc21a49
Goal: Provide a small host-side guard primitive so runtime integrations can check explicit capability approvals.
Changes:
- Added CapabilityGate with private granted state and from_approvals(), allows() and require().
- Gate construction uses the validated manifest and explicit approval planner; no caller can construct its private grant set directly.
- Added tests for an approved capability, an unapproved request and an unknown capability.
Verification:
- Package workflow pending.
Limitations:
- The gate is not yet wired into Horizon host APIs or DreamMod Core; it is not an end-to-end sandbox.
Next:
- Verify CI, then integrate the gate at the actual host-call boundary before adding lifecycle execution.


## S-2026-11-18-27 — Verify default-deny host capability gate
Date: 2026-11-18
Commit: 5085971c93c3dbc2a0089e136e5f9a7aecc21a49
Goal: Verify the capability guard helper before it is connected to Horizon host APIs.
Changes:
- Recorded the gate verification and retained the explicit limitation that it is not yet integrated into the host.
Verification:
- DreamMod package Actions run 44300879584: formatting SUCCESS, 22/22 tests SUCCESS, Clippy SUCCESS.
- Approved capability succeeds; unapproved and unknown capabilities are denied.
Limitations:
- No Horizon host API wiring, permission UI, interpreter or production sandbox.
Next:
- Design a host adapter that checks CapabilityGate before every privileged operation, then add lifecycle states without executing scripts.


## S-2026-11-18-06 — Default-disabled package lifecycle
Date: 2026-11-18
Commit: eae8f1c0cda26e2f9a17282e5f8798efe01ec7bc
Goal: Separate package validation, capability approval and activation into explicit lifecycle transitions.
Changes:
- Added a lifecycle policy module with AwaitingApproval, Ready, Enabled, Disabled and Failed states.
- Capability approvals alone do not enable a mod; activation requires a separate explicit transition.
- Disable blocks capability checks; failure discards the capability gate and is terminal.
- Added four lifecycle regression tests.
Verification:
- GitHub Actions package workflow is triggered by the source change; result pending inspection.
Limitations:
- This is policy/state tracking only. It does not execute scripts, install packages, schedule lifecycle callbacks, or enforce permissions at actual Horizon host API boundaries.
Next:
- Verify package CI, then design the host adapter so every privileged Horizon call consults the lifecycle gate.


## S-2026-11-18-07 — Guarded host operation boundary
Date: 2026-11-18
Commit: 407a7628abbf7f381a6f992272047103924112f9
Goal: Ensure a host operation callback cannot run unless lifecycle and capability checks succeed.
Changes:
- Added generic HostAdapter::invoke(capability, callback), which checks Enabled state and the explicit grant before invoking the callback.
- Added regression tests proving denied operations never invoke callbacks and authorized operations do.
- Bounded failure diagnostics to 512 bytes and rejected control characters.
Verification:
- Package workflow triggered by code changes; CI status pending inspection.
Limitations:
- Generic host boundary only. No concrete Horizon API calls are routed through it yet; no sandbox, scheduler, or script execution.
Next:
- Verify CI, inspect the exact pinned Horizon Hooks/API extension points, then implement narrowly scoped adapters that route each supported host operation through this guard.


## S-2026-11-18-28 — Harden host-call denial and inspect pinned Horizon host API
Date: 2026-11-18
Commits:
- Code: d9a524f6ff0b4ef6124abf2c0fdd8aaebdc19747
- Host bridge contract: d016ea58297e06d90abcc0de06cc32333a5ac82e
- Roadmap: 16c18983b8e770d6b978f507089deeb3981b4051
Goal: Extend lifecycle guard regression coverage and replace assumptions about Horizon integration with facts from the exact pinned SDK.
Changes:
- Extended HostAdapter tests to prove callbacks remain uncalled when a mod is disabled or has failed, in addition to the existing not-enabled and capability-denied cases.
- Corrected the failure-reason test to pass an actual newline control character rather than the two-character escaped sequence.
- Inspected the pinned Horizon mod API and its modding guide: Hooks, ServerHost/ClientHost, Wire values, explicit load phases, server_tick phase semantics, Send + Sync and cross-thread/re-entrant constraints.
- Added docs/dreammod/HORIZON_HOST_BRIDGE.md with the verified upstream surface, adapter rules, initial integration sequence and explicit implementation gaps.
Verification:
- DreamMod package Actions run 44304258172: SUCCESS. Formatting, all package tests and Clippy with `-D warnings` passed on code commit d9a524f6ff0b4ef6124abf2c0fdd8aaebdc19747.
Limitations:
- HostAdapter remains a generic policy guard. No concrete Horizon host operation is routed through it; it is not a sandbox and does not execute scripts.
Next:
- Check package workflow 44304258172. Then implement the first narrow owned-value adapter against the pinned SDK, only after defining its DreamMod operation contract and testing the capability check before the Horizon call.


## S-2026-11-18-29 — Record package CI verification
Date: 2026-11-18
Commit: d9a524f6ff0b4ef6124abf2c0fdd8aaebdc19747
Goal: Record the completed verification for the lifecycle host-call denial regression tests.
Verification:
- DreamMod package Actions run 44304258172: SUCCESS.
- Rust formatting, all package tests and Clippy with `-D warnings` passed.
Limitations:
- This verifies the package/lifecycle crate only. It does not verify concrete Horizon host integration or a sandbox.
Next:
- Continue with the first narrow, owned-value Horizon adapter after its operation contract is specified.


## S-2026-11-18-30 — Add owned player snapshot boundary
Date: 2026-11-18
Commits:
- DTO: 3e8baf14cd1750ff0ec2ce45ebbb55aaf13aa4e6
- Export: 8845d96f3130e90a381aab85244546dcad447f3d
- Capability-gated adapter: 7bae3eaad6128ff815e657541dd3cd1880dca485
- Bridge contract docs: c20caab7545a15f4511c3e0acc4f30e2de80c468
Goal: Define the first typed, owned-value contract between Horizon and interpreted DreamMod without allowing Horizon handles to escape.
Changes:
- Added PlayerSnapshot with owned UUID, bounded player name, validated dimension identifier and finite bounded position.
- Added HostAdapter::read_player_snapshot(), which checks Enabled state and the entities.read grant before invoking the supplied Horizon read closure.
- Added tests for valid and invalid snapshots, denied callback non-invocation and malformed DTO rejection.
- Updated the Horizon bridge contract to specify the ownership and capability rules.
Verification:
- DreamMod package Actions run 44309091208: SUCCESS. Rust formatting, all package tests and Clippy succeeded on source commit 7bae3eaad6128ff815e657541dd3cd1880dca485.
- PMCS validation is triggered by the documentation updates; result pending.
Limitations:
- No concrete ServerHost closure is implemented in DreamMod Core; the DTO is not exposed to an interpreter. No production sandbox or end-to-end .dmod execution exists.
Next:
- Implement and compile the Core-side player snapshot reader against the pinned Horizon SDK. Verify the exact event and handle method signatures in CI before adding script bindings.


## S-2026-11-18-31 — Add bounded player snapshot batches
Date: 2026-11-18
Commits:
- Batch DTO and validation: 5a4c946076127857e01b2a461f21fc0a2fcaa9fc
- Capability-gated batch read: 4397de1a28d536b0ffca448b553478ecae3467fb
Goal: Keep player reads coarse-grained and bounded rather than crossing the host/interpreter boundary once per player.
Changes:
- Added PlayerSnapshotBatch with a 256-player cap and duplicate UUID rejection.
- Added HostAdapter::read_player_snapshots(), checking entities.read once before the batch callback.
- Added tests for batch bounds, duplicate IDs and denied callback non-invocation.
Verification:
- Package workflow for the latest source commit is queued; result pending.
Limitations:
- No concrete Horizon ServerHost batch collector or interpreter binding exists yet.
Next:
- Verify package CI, then implement the trusted Core-side batch collector using host.players().server_players() and owned values, with no Horizon handles escaping the hook.


## S-2026-11-18-32 — Verify bounded player snapshot batches
Date: 2026-11-18
Commit: 4397de1a28d536b0ffca448b553478ecae3467fb
Goal: Record verification of the bounded player batch DTO and capability-gated read.
Verification:
- DreamMod package Actions run 44309674771: SUCCESS.
- Rust formatting, all package tests and Clippy with -D warnings passed.
Limitations:
- The tests verify the package contract and guard only; no Horizon ServerHost collector, interpreter binding or sandbox exists.
Next:
- Build the Core-side collector against the pinned Horizon SDK, then test host access and ownership before adding any script-visible API.


## S-2026-11-18-33 — Implement the Core-side Horizon player snapshot collector
Date: 2026-11-18
Commits:
- Collector module: 7ecd981babec69a5081aecacd25f7b36629f71ea
- Core module export: b2362c6a1347390f4f330655fb0823e8563d94d8
Goal: Replace the abstract host-read closure with the first concrete, pinned-SDK Core collector while keeping Horizon handles out of the future script boundary.
Changes:
- Added a Core-local owned PlayerSnapshot and PlayerSnapshotBatch, avoiding a dependency on the package/ZIP crate.
- Used ServerHost::players().server_players() in Horizon connection order.
- Rejects a list above 256 before individual reads; copies UUID, name, dimension and position; rejects unavailable values, malformed IDs/names, invalid coordinates and duplicate UUIDs.
- Exported the collector as a Core module for later capability-gated runtime use.
Verification:
- DreamMod Core Actions run 44310547586 is building cargo-horizon-mod; the SDK compile/check and package steps are pending.
- PMCS run 44310547866 failed because implementation changes lacked the required memory updates. This entry and the companion PMCS files are the correction; rerun validation after these commits.
Limitations:
- No interpreter, script-visible binding, sandbox or per-mod capability invocation exists yet. The collector is not called on every tick and retains no Horizon handles.
Next:
- Confirm pinned SDK host/WASM compilation. Fix any actual signature/type errors reported by the checker. Then add a Core test harness or integration test for the snapshot collector and connect it only through the lifecycle capability gate.


## S-2026-11-18-34 — Require authorization before Core player reads
Date: 2026-11-18
Commit: 99929e38d5f1a815e8b17cb069a5eff8dfcc8e3e
Goal: Prevent Core player snapshot collection from touching Horizon before a per-mod entities.read authorization check.
Changes:
- Added the PlayerReadAuthorization interface and made the public player collector require it.
- Added a helper that performs authorization before running the host-read closure.
- Added regression tests for denied callback non-invocation and exactly-once authorized execution.
- Updated docs/dreammod/HORIZON_HOST_BRIDGE.md with the verified collector and explicit limitation.
Verification:
- Earlier Core workflow run 44310547586 passed the pinned SDK build, host/WASM validation and source packaging for the collector before this authorization change.
- Latest Core workflow run 44312568247 and API-only overlay run 44312568292 are running for this commit.
- PMCS run 44312568120 failed because required memory files were not updated alongside this source change; this entry and companion files correct the omission. Re-run PMCS after these docs commits.
Limitations:
- No implementation connects PlayerReadAuthorization to a concrete per-mod ModLifecycle instance yet. No interpreter, script binding or sandbox exists.
Next:
- Verify latest Core workflow. Then implement the per-mod lifecycle adapter and test it against the package lifecycle contract before exposing player data to scripts.


## S-2026-11-18-35 — Add Core snapshot validation tests
Date: 2026-11-18
Commit: ebe09145849063ade0e01f18e55a23bec18cd4e4
Goal: Ensure the Core-local DTO enforces the same input and batch constraints as the package contract.
Changes:
- Added tests for invalid Minecraft player names and malformed dimension identifiers.
- Added tests for NaN, infinite and out-of-range coordinates.
- Added tests for duplicate player UUIDs and the 256-player batch cap.
Verification:
- The source change triggers the Core workflow. No claim is made that tests executed until the workflow includes/runs cargo test; cargo-horizon-mod check is currently the known host/WASM validation command.
Limitations:
- These tests do not exercise a live ServerHost. The collector's actual pinned SDK compilation was verified by Core workflow 44310547586 before the latest authorization/test edits.
Next:
- Verify latest Core compile and investigate a safe cargo test CI step against the generated SDK.


## S-2026-11-18-36 — Correct the gated collector source regression
Date: 2026-11-18
Commit: eadd1aa545851d487da4e2ff2f37edfbec327dfb
Goal: Restore valid source structure and ensure the public player collector remains permission-gated.
Changes:
- Replaced the full malformed collector section.
- Kept the Horizon read implementation private and callable only through the public function after PlayerReadAuthorization succeeds.
Verification:
- The Core workflow for the intermediate source commit is not valid evidence because it targeted the malformed version.
- A new Core workflow is triggered by eadd1aa; its result must be checked.
Limitations:
- No live ServerHost unit test or .dmod runtime integration exists.
Next:
- Verify the corrected Core workflow and confirm test compilation/execution separately.


## S-2026-11-18-37 — Run Core formatting and unit tests in CI
Date: 2026-11-18
Commit: ffe93f053ab6a0a0df99953a1206262661e8b111
Goal: Ensure Core-local regression tests are actually executed rather than relying only on the Horizon native-mod checker.
Changes:
- Added cargo fmt --all -- --check after Horizon SDK check.
- Added cargo test after the verified SDK has been fetched into the mod project.
Verification:
- The workflow triggered by this change must confirm that the generated SDK path and unit-test harness work. No pass is claimed yet.
Limitations:
- Unit tests do not replace a live ServerHost integration test or prove per-mod runtime wiring.
Next:
- Check DreamMod Core Actions and fix any test/build errors reported.


## S-2026-11-18-38 — Optimize bounded host snapshots and add package load deadline
Date: 2026-11-18
Commits:
- Core snapshot validation: d650c58a9ee9b08a365cbe4f830a16e8510daaec
- Package bridge validation: 5e8e58a77515b1f8ec6c5aa0511005c403b44dd5
- Cooperative 60-second archive deadline: b689551994c45b603fcab8f08c5708973c9f2c3b
- Core workflow rustfmt installation: e47e8e0f858c9c9d09088db17d23a74584ae420a
Goal: Reduce avoidable batch-validation allocation overhead and bound package archive inspection without blocking the game thread.
Changes:
- Replaced BTreeSet duplicate UUID detection in both Core and package bridge with a contiguous UUID buffer, sort_unstable, and adjacent duplicate detection. The 256-player cap and original snapshot order are unchanged.
- Added MOD_LOAD_TIMEOUT (60 seconds), inspect_dmod_with_deadline(), cooperative deadline checks between archive entries and decompression chunks, and an expired-deadline regression test. inspect_dmod() starts with the default 60-second deadline.
- Fixed Core CI setup to install rustfmt after run 44313213891 showed that the pinned SDK/API check passed but formatting stopped the job before unit tests.
Verification:
- Pending fresh package and Core Actions runs. No test pass or benchmark speedup is claimed yet.
- Previous Core run 44313213891: Horizon native source/API/host-WASM check succeeded; formatting failed because rustfmt was missing; Core unit tests were skipped.
Limitations:
- The deadline currently covers package archive inspection only. The future loader must propagate the same monotonic deadline through dependency preparation, script parsing and registration, with cooperative cancellation at each stage.
- No interpreter/scheduler or ARM64 benchmark exists; no FPS improvement is measured.
Next:
- Verify both workflows, then add a benchmark for snapshot batch validation and design immutable per-dispatch batch reuse with explicit thread/invalidation rules.


## S-2026-11-18-39 — Remove temporary heap allocation from player batch validation
Date: 2026-11-18
Commits:
- Core fixed scratch buffer: b2185e8917e351dfd0408231e855efe47e6b3bfd
- Package fixed scratch buffer: f3a55af3a332073db92df24de00e347bc55a3bb8
Goal: Remove the remaining temporary heap allocation from bounded duplicate UUID validation.
Changes:
- Replaced the temporary Vec<u128> with a fixed [u128; 256] stack array in Core and package PlayerSnapshotBatch validation.
- Only the populated prefix is sorted; the original snapshot order is unchanged. The 256-player cap is checked before slicing.
- Updated Core formatting to run only on the Core manifest, avoiding generated SDK source formatting differences.
Verification:
- Package CI for the 60-second deadline passed in run 44317125357. New package run 44317885243 and Core run 44317880298 are pending for the stack-buffer implementation.
- No benchmark or FPS improvement is claimed.
Limitations:
- No real interpreter dispatcher exists yet, so per-dispatch snapshot reuse remains architectural work, not implemented behavior.
- The 60-second deadline currently bounds ZIP inspection only, not a complete mod load transaction.
Next:
- Verify both latest workflows, add batch benchmarks, then design per-dispatch immutable snapshot reuse with explicit thread and invalidation rules.


## S-2026-11-18-40 — Share immutable player snapshot batches
Date: 2026-11-18
Commits:
- Core Arc-backed batch: 3f629cc54a27992f9a7f402dfda3fcfa0505b4bf
- Package Arc-backed batch: 74310a797393ec319fb9192d931a7b860ba55ced
Goal: Avoid deep-copying snapshot records when multiple authorized consumers need the same immutable batch.
Changes:
- Changed PlayerSnapshotBatch storage from Vec<PlayerSnapshot> to Arc<[PlayerSnapshot]> in Core and package contract.
- Existing getters and ordering are unchanged. Cloning a batch shares its immutable backing allocation.
- Added tests using Arc::ptr_eq to prove clones share storage.
Verification:
- Package CI run 44318172781 and Core CI run 44318166133 are pending. No benchmark is claimed.
Limitations:
- No runtime dispatcher or cache exists yet. This storage design only makes future per-dispatch sharing cheap; each mod must pass its own capability check before receiving the batch.
Next:
- Verify both workflows, then benchmark batch clone/delivery and design the scheduler's per-dispatch ownership and invalidation rules.


## S-2026-11-18-41 — Read PMCS, verify Core CI and repair overlay route type
Date: 2026-11-18
Source HEAD at start: 741f7ff15bb22a2b16b8a5a667fe471d3ee3ddc9
Changes:
- Read AI_HANDOFF.md, PROJECT_MEMORY.md, CURRENT_STATE.md, the architecture specification, ISSUES.md, ROADMAP.md, CHANGELOG.md and the latest session log before continuing.
- Verified DreamMod Core Actions run 44322745253: pinned Horizon native/API/host-WASM check, formatting, Core unit tests and source packaging all passed.
- Verified package Arc-backed batch CI run 44318172781 and archive-deadline CI run 44317125357.
- Investigated PMCS run 44322745310. All six validator regression tests passed; freshness validation failed because CURRENT_STATE.md, ISSUES.md, ROADMAP.md, CHANGELOG.md and SESSION_LOG.md had not been updated with the latest implementation/test result.
- Investigated overlay run 44322745328. The injected test used the wrong Route namespace: native_mods::Route is the picker/UI enum, while sdk_cache::slices and prebuilt::compile require prebuilt::Route. Pinned Horizon source confirms prebuilt::Route::Api.
- Updated scripts/ci/patch_horizon_api_route.py to use prebuilt::Route::Api for both the SDK slice fetch and prebuilt compiler call.
Verification:
- Core run 44322745253: PASS.
- Package run 44318172781: PASS.
- Archive deadline package run 44317125357: PASS.
- PMCS run 44322745310: FAIL due missing documentation updates; its six validator regression tests passed. Re-run after this complete companion update.
- Overlay run 44322745328: FAIL due the route type mismatch; corrected patch awaits a fresh workflow run.
Limitations:
- The actual native host-loader test and overlay artifact inspection have not run successfully.
- No production .dmod interpreter/loader/sandbox, concrete lifecycle authorization adapter, dispatcher/cache, or ARM64 benchmark exists.
Next:
- Confirm PMCS passes on the complete documentation update.
- Rerun the API-only overlay workflow and inspect any subsequent compile/runtime failure; only then test the real Horizon native host loader and inspect the artifact.


## S-2026-11-18-42 — Match prebuilt compiler to the signed SDK
Date: 2026-11-18
Starting commit: 239f75b30cb1aec9ce47fbdd20f04ee5c0c8c67c
Changes:
- Inspected overlay run 44324363090 after the Route namespace fix. The test injection and Horizon launcher test build succeeded; the API-only prebuilt build then failed with E0514 for SDK crate mod_api compiled by an incompatible rustc, plus a generated crate::hooks lookup error.
- Audited pinned Horizon source at 0b14f8acfd97fa2c750e42795249dcc981178ca4. The SDK publish workflow reads tools/ci/rust-version, whose exact value is 1.99.1.
- Changed .github/workflows/dreammod-prebuilt.yml from Rust 1.95.0 to 1.99.1 and updated the artifact metadata compiler field (commit 96d2e74071afb75f5657b0c2fe88caf2e529adc7).
- Confirmed PMCS run 44328650216 passed all regression and freshness checks on commit 96d2e740.
Verification:
- Overlay run 44324363090: failed at API-only prebuilt compilation with E0514; host-loader and artifact steps skipped.
- PMCS run 44328650216: PASS.
- Overlay rerun 44328650675: in progress at the time of this entry; no compile/load result yet.
Limitations:
- Do not independently modify Core's Hooks declaration until the rerun establishes whether the missing crate::hooks error persists with the SDK-matching compiler.
Next:
- Inspect overlay run 44328650675 after the compile step.
- If successful, verify native host loading and inspect archive metadata/artifact.
- If it fails, capture the first diagnostics and compare with SDK's rust-version before changing code.


## S-2026-11-18-43 — Add temporary Minecraft reference smoke to engine CI
Date: 2026-11-18
Starting commit: fb8709b00ba66bb80bfffaa44d7556016595a7b0
Changes:
- Inspected current engine, testkit, DreamMod Core, package/runtime crates, workflows and project memory before adding new work.
- Confirmed the engine workspace currently has only horizontal block-to-chunk mapping plus testkit fixture schema/validation; no Java oracle or differential fixture runner exists.
- Confirmed the official server downloader and offline security tests already existed, but the workflow downloaded Minecraft only for manual dispatch and did not run engine tests alongside the reference.
- Changed the reference workflow to run on engine pushes/PRs, download official Java 26.3 into a unique temporary runner directory, verify the bundled ZIP/CRCs, run formatting/tests/Clippy, and remove all temporary files unconditionally.
- Fixed a workflow parse failure caused by using runner.temp in job-level env; runner.temp is now used only in step-level env.
- Corrected overlay metadata to report Rust 1.99.1, matching the compiler used by the signed SDK.
Verification:
- Reference/engine workflow run 44331401582: PASS. Official server download verified (62,294,556 bytes; SHA-1 33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c), ZIP CRC passed, 5 engine tests + 5 testkit tests passed, format/Clippy passed, temporary jar cleanup passed.
- PMCS run 44331401564: PASS.
- Overlay run 44328650675: API-only prebuilt compilation completed successfully but host-loader was cancelled by a newer commit; no artifact was uploaded.
- Overlay rerun 44330930078: still in progress; inspect before claiming host load or artifact success.
Limitations:
- The current reference workflow does not yet compare engine output to Minecraft. A Java 26.3 oracle and differential fixtures are still missing.
- DreamMod Core source .hmod is validated and packaged, but Hooks is empty and Core collector/runtime/script/lifecycle integration is incomplete.
- Full-game startup remains blocked on a matching licensed installation and Horizon's ownership gate; do not bypass it.
Next:
- Verify overlay run 44330930078 through host-loader and archive upload.
- Design a small Java 26.3 oracle for the existing coordinate contract.
- Plan one narrow Core/runtime bridge with concrete per-mod lifecycle authorization before expanding the engine or API surface.

## S-2026-11-18-44 — Extend engine coordinates through the HNMod bridge
Date: 2026-11-18
Starting context: engine-core contained horizontal block-to-chunk mapping; DreamMod Core exposed that mapping through a byte-synchronized source mirror.
Changes:
- Added `SectionPos` and `SectionAddress` plus `BlockPos::section_address()` for X/Y/Z Euclidean section mapping and local coordinates. The bridge now returns section position and local Y as owned values.
- Kept section geometry independent from world height, section serialization, palette storage and block-state identifiers.
- Added tests for negative coordinates and section boundaries. CI revealed rustfmt adjustments and an invalid overflow assertion; removed the impossible assertion and formatted the canonical source and HNMod mirror identically.
- Fixed the native-host CI environment after run 44335372878 showed that the real host could not resolve the sibling `libhorizon_mod_runtime.so`. The workflow now sets `LD_LIBRARY_PATH` to include the overlay directory (commit f64b016a6cf8dae17769d2ae53cd0680227f3acc).
Verification:
- Existing DreamMod Core run 44335366329 passed before the new section bridge.
- Overlay run 44335372878: API-only compilation passed; real native-host load failed on missing runtime dylib search path. Fix rerun 44341223178 is pending/in progress.
- Engine run 44341336046: official Java 26.3 download and archive checks passed; formatting failed on the new section source, so engine tests and Clippy were skipped. Rustfmt corrections are committed in 01ae7c243bfe5b8d449036c5326b9a140b3d4df6 and later commits; verify the latest run.
- PMCS run 44341395334 failed only because the implementation files changed without updating the five required memory documents; this session updates them.
Limitations:
- The section address is a coordinate primitive, not a chunk-section storage format or vanilla differential implementation.
- HNMod's `Hooks` implementation remains empty; no end-to-end .dmod execution or complete game launch exists.
Next:
- Inspect latest Actions for engine, Core, overlay and PMCS.
- If host loading passes, inspect artifact layout and upload; then separately validate the actual launcher search-path behavior.
- Add the next engine domain only with pinned Java 26.3 reference fixtures and deterministic tests.


## S-2026-11-18-45 — Verify prebuilt host load and correct Core formatting
Date: 2026-11-18
Starting commit: 447597dd6702869adde47cd010d062ff53a8d4f2
Changes:
- Inspected DreamMod Core, overlay and PMCS Actions and retrieved the failing job logs.
- Confirmed Core run 44341466411 passed pinned Horizon native/API/host-WASM validation and engine mirror parity, then failed only because rustfmt wanted the engine_bridge.rs import on one line.
- Corrected that formatting in commit 42d9ac762502ad5ef3db9158a9885c99a24ae238.
- Confirmed overlay run 44341466372 passed API-only compilation, real Horizon native-host loading, archive inspection and artifact upload.
- Updated DM-031: the CI native-host loader issue is resolved for the test process, while actual launcher search-path behavior remains unverified.
Verification:
- Overlay 44341466372: PASS all steps.
- PMCS 44341597122: PASS on the previous documented change set.
- Core 44341466411: FAIL at rustfmt only; Core tests and packaging were skipped.
- Core rerun 44361109477: pending.
- PMCS 44361109707: failed because the prior code change was not accompanied by the five required memory documents in the same compared change range; this session appends the required records.
Limitations:
- No full launcher/game startup is verified. CI's explicit LD_LIBRARY_PATH does not prove the installed launcher uses the same loader configuration.
- Core Hooks remain empty; no production .dmod interpreter, lifecycle adapter or live script execution exists.
Next:
- Verify Core rerun and PMCS after this documentation update.
- Inspect overlay artifact metadata and separately establish the launcher's runtime-library search behavior.
- Keep future engine growth fixture-driven; do not claim Minecraft parity from coordinate tests alone.


## S-2026-11-18-46 — Verify DreamMod Core section bridge and inspect Linux overlay dependencies
Date: 2026-11-18
Changes:
- Checked Core rerun 44361109477 after correcting engine_bridge.rs formatting in commit 42d9ac762502ad5ef3db9158a9885c99a24ae238.
- Verified pinned Horizon native/API/host-WASM validation, byte-identical engine/HNMod source parity, rustfmt, all 12 Core unit tests, source .hmod packaging and artifact upload.
- Downloaded and inspected overlay artifact 14445102997. Its receipt hash matched the mod library. ELF inspection showed libmod_vocab.so depends on libhorizon_mod_runtime.so but lacks RUNPATH; the mod library's own $ORIGIN RUNPATH does not cover that transitive dependency.
- Audited pinned Horizon launcher code: native_libraries loads the mod with libloading, and Linux client_command does not explicitly add the overlay directory to LD_LIBRARY_PATH.
- Updated the overlay workflow in commits 320b0bda and 4d2c910c to set $ORIGIN RUNPATH on bundled .so files, recalculate the native-mods.json library hash, and run the native-host test without inheriting an overlay LD_LIBRARY_PATH.
Verification:
- Core run 44361109477: PASS; 12 tests and source artifact upload passed.
- Overlay run 44361815335: pending at the time of this entry; the new self-contained dynamic-loader test is not yet verified.
- PMCS run 44361415616: PASS for the previous documented update.
Limitations:
- Core Hooks remain empty; the package/runtime is not wired to a live interpreter.
- No complete .dmod sandbox, launcher-installed game startup or Java 26.3 differential engine test exists.
Next:
- Verify run 44361815335 through RUNPATH patch, host loader, archive inspection and upload.
- If it passes, inspect the resulting artifact with LD_LIBRARY_PATH unset and confirm the native-mods.json hash matches.
- Keep launcher/game startup separate from native-host unit-test evidence.

## S-2026-11-18-45 — shared lifecycle authorization contract
- Added dependency-free `mods/dreammod-contract` to avoid coupling trusted Core to the package archive parser dependencies.
- Moved the `PlayerReadAuthorization` trait and canonical `entities.read` identifier into that crate. Core now consumes the shared trait; `ModLifecycle` implements it using the existing enabled-state and capability checks.
- Added regression coverage that calls the shared trait directly and verifies default-deny, explicit grant, separate enable, disable and fail-closed behavior.
- Package/Core/overlay CI is pending. The earlier PMCS run `44362787017` failed because the required memory documents were not updated after later source commits; the memory update is now being appended after the latest code changes.
- No interpreter, scheduler, actual runtime manager, script binding or end-to-end `.dmod` execution is implemented.

## S-2026-11-18-46 — Add deterministic ModManager
Date: 2026-11-18
Commit: `b3e3f76dce` (manager), `e11db6d9e9` (module export)
Goal: Centralize validated package ownership and per-mod lifecycle/dependency transitions before implementing a script runtime.
Changes:
- Added `mods/dreammod-package/src/manager.rs` and exported it from the package crate.
- Manager construction validates the complete dependency graph and retains deterministic dependency-first load order.
- Manager owns validated package contents and one `ModLifecycle` per package.
- Explicit approval remains separate from activation. A mod cannot enable until every declared dependency is Enabled.
- Disabling a dependency is rejected while a direct dependent remains Enabled.
- A mod failure cascades fail-closed to transitive dependents in reverse load order.
- Added tests for dependency order, package retention, enable ordering, disable protection, failure cascade and invalid graphs.
Verification:
- Package workflow `44363310681`: in progress at the time of the entry; formatting, tests and Clippy pending.
- PMCS workflow `44363323071`: pending.
Limitations:
- No interpreter, script scheduler, transactional resource registration, runtime quotas or concrete Horizon host integration was added.
Next:
- Verify package/PMCS workflows. Then design a registration transaction API (prepare/commit/rollback) and tests before connecting runtime scripts to Core.

### S-2026-11-18-46 follow-up — catch and fix transitive cascade order
- Inspected the exact package test log for run `44363310681`: 36 tests passed, but manager tests were absent because `pub mod manager` was committed in the subsequent `e11db6d` change.
- Code review found the manager's first failure-cascade traversal used reverse topological order, which can visit a grandchild before the intermediate dependency has been marked failed. Corrected it to walk dependency-first order in commit `bb8806d`.
- Require the next package run to compile `manager.rs` and execute its manager tests; do not cite the earlier 36-test pass as manager verification.

### S-2026-11-18-46 verification addendum
- Package run `44363502581`: SUCCESS. The exact log lists all five `manager::tests`; 41 total tests passed, then Clippy completed successfully with warnings denied. This is the first run that actually compiled and exercised the exported manager module after the cascade-order fix.
- Core run `44362777239`: SUCCESS. Pinned `cargo-horizon-mod` build, native manifest/source/API/host-WASM validation, engine mirror parity, formatting, Core unit tests, source bundle packaging and artifact upload all passed.
- Overlay run `44362776804`: still in progress at API-only compilation; no host load result yet.
- PMCS run `44363588460`: pending.

## S-2026-11-18-47 — Replace unsupported contract dependency with callback boundary
Date: 2026-11-18
Goal: Fix the real Horizon API-only overlay compile failure without weakening production validation.
Changes:
- Overlay run `44362776804` failed while staging/compiling DreamMod Core: `unresolved import dreammod_contract`. The SDK's native package staging did not include the local Cargo path dependency.
- Removed the local `dreammod-contract` crate and removed it from Core/package Cargo manifests.
- Changed Core player snapshot authorization to accept a one-shot closure, invoked before any `ServerHost` access.
- Added `ModManager::require_capability(mod_id, capability)` as the package-side callback target. It enforces both enabled state and the explicit capability grant.
- Removed the trait implementation tied to the unsupported external crate. The package retains its capability identifier and lifecycle checks.
Verification:
- Previous overlay run `44362776804` confirms the root cause; post-fix Core/package/overlay workflows are pending.
Limitations:
- The callback seam is not yet wired from a live ModManager instance into a real Horizon hook/context. No script interpreter or scheduler exists.
Next:
- Verify Core API-only prebuilt compilation and host loading, then design the runtime coordinator/hook ownership path.

## S-2026-11-18-48 — Harden callback bridge and failure propagation
Date: 2026-11-18
Commit: `259fb9b43c` (capability callback), `46ebd860ef` (package import fix), `4103668fee` (cascade hardening)
Changes:
- Added `ModManager::require_capability` as the package-side per-mod authorization callback target.
- Added tests covering denied while AwaitingApproval, denied while Ready, allowed only when Enabled with explicit `entities.read`, and denial for an ungranted capability.
- Added a regression for a transitive dependency failure where the intermediate mod was already Failed; propagation now avoids an invalid duplicate failure transition and still cascades to descendants.
Verification:
- Package run `44364113419` passed format, tests and Clippy but predates the last test/hardening commit. Post-`4103668` package run pending.
- Core run `44363904945` and overlay run `44363904976` still running at last inspection.
Limitations:
- No runtime coordinator currently passes a live ModManager callback into the Horizon host collector; only the typed closure boundary and manager API exist.

### S-2026-11-18-48 verification addendum
- Package run `44364198594` on commit `4103668fee` completed SUCCESS. Exact logs show 42 passing tests, including all seven manager tests (dependency order, callback authorization, dependency enable gate, disable protection, normal and already-failed transitive cascades, invalid graph); formatting and Clippy also passed.
- PMCS run `44364312080` completed SUCCESS with all six validator regression tests and structural/freshness validation.
- Core run `44363904945` was still building pinned `cargo-horizon-mod`; overlay run `44363904976` was still compiling the API-only route. Real native host loading remains unverified.

## S-2026-11-18-49 — Add capability-gated in-memory asset reads
Date: 2026-11-18
Commit: `f90d0b1dfed66ce134acc4710db3b93d47b9e667`
Goal: Expose `.dmod` assets to a future interpreted runtime without extracting them or leaking filesystem paths.
Changes:
- Added `ModManager::read_asset(mod_id, path)`.
- Requires the target mod to be Enabled and hold the explicit `assets.read` grant.
- Restricts reads to `assets/` and reuses the same safe archive-path validation used during package inspection.
- Returns borrowed immutable bytes from the retained validated package; no extraction or copy.
- Added tests for denied-before-enable, missing grant, successful byte read, traversal rejection and missing assets.
Verification:
- Package workflow after commit `f90d0b1` pending.
Limitations:
- No image/audio/model decoder, resource registration, cache policy, interpreter binding or per-script resource quota exists yet.
Next:
- Verify package tests and Clippy. Then define registration transactions/rollback before exposing asset bytes to actual scripts.

### S-2026-11-18-49 verification addendum
- Package run `44364618050` on commit `f90d0b1dfe` completed SUCCESS: 43 unit tests passed (including `asset_reads_require_enabled_state_and_assets_read_capability`), rustfmt and Clippy all passed.
- PMCS run `44364715327` completed the validator regression suite and structure/freshness validation successfully; post-job cleanup was still finishing at inspection.
- Core run `44363904945` passed the pinned Horizon checker build, native manifest/source/API/host-WASM validation, engine mirror parity and formatting; Core unit tests were in progress.
- Overlay run `44363904976` remained at API-only prebuilt compilation; native host loading and artifact upload remain unverified.

### S-2026-11-18-47 verification addendum
- Core run `44363904945` completed SUCCESS after removal of the unsupported local contract crate and conversion to the closure-based authorization API. Pinned Horizon checker build, native manifest/source/API/host-WASM checks, engine mirror parity, formatting, Core unit tests, source bundle packaging and upload all passed.
- Overlay run `44363904976` passed API-only prebuilt compilation and Linux `$ORIGIN` RUNPATH setup. The real Horizon native host load test without overlay environment overrides was in progress; archive inspection and upload pending.

## S-2026-11-18-50 — Add activated entrypoint source access
Date: 2026-11-18
Commits: `b085c47eeb` (initial method), `be02358531` (state-only lifecycle check), `ee903d7c76` (corrected manager gate)
Goal: Create the smallest safe source handoff from validated `.dmod` packages to a future trusted interpreter.
Changes:
- Added `ModLifecycle::ensure_enabled`, separating activation state from named capability grants.
- Added `ModManager::entrypoint_source`, returning a borrowed slice of the manifest entrypoint only after explicit activation.
- Avoided inventing a `scripts.execute` capability; privileged host operations continue to require named explicit grants.
- Added a regression test for denial before enable and success after enable.
Verification:
- Package workflow after commit `ee903d7` pending.
Limitations:
- No script parser/interpreter, sandbox, instruction/memory budget, scheduling or execution has been added.
Next:
- Verify package CI. Then design the runtime execution contract and sandbox limits before selecting an interpreter.

### S-2026-11-18-50 verification addendum
- Package run `44365010182` on commit `ee903d7c76` completed SUCCESS: all 44 tests passed, including `manager::tests::entrypoint_source_is_borrowed_only_after_activation`; rustfmt and Clippy passed.
- Overlay run `44363904976` had passed API-only prebuilt compilation and `$ORIGIN` setup; real Horizon native host load test without overlay environment overrides remained in progress.

## S-2026-11-18-51 — Fix overlay host-loader step environment
Date: 2026-11-18
Commit: `9347eb43514211d0cc64f8efaee75393058da531`
Goal: Ensure the real Horizon host-loader test receives the overlay path in its own GitHub Actions process.
Changes:
- Reviewed the in-progress overlay run `44363904976` after API-only compilation and `$ORIGIN` setup passed.
- Found the workflow exported `DREAMMOD_PREBUILT_DIR` only in the build step. GitHub Actions creates a separate process/environment for each step, so that variable was not available to the loader test.
- Re-exported `$RUNNER_TEMP/dreammod-prebuilt` in the loader step, added file-size and ELF RUNPATH diagnostics, and bounded the test to 180 seconds.
Verification:
- Previous run `44363904976` was superseded by this workflow correction during the host-test step.
- New overlay workflow run pending.
Limitations:
- This validates the CI host loader only; actual launcher runtime search-path behavior still requires a separate check.


## S-2026-11-18-53 — Verify API-only overlay in real Horizon host
Date: 2026-11-18
Commit: `f6b43915f9ef4396e5733d079b47ff445ef1d5d1`
Goal: Complete the CI proof that the API-only DreamMod native overlay can be loaded by Horizon's actual native host.
Changes:
- Fixed shell line continuations in the native dependency installation step after inspecting the failed run `44368761333` (apt had interpreted a standalone `\\` as a package name).
- The subsequent overlay run `44368880314` passed every step: pinned source/toolchain, HNMod engine mirror parity, API-only overlay compilation, `$ORIGIN` RUNPATH, real `horizon-mod-host` load without `LD_LIBRARY_PATH`, archive inspection and artifact upload.
- Artifact `14459680134` was uploaded with SHA-256 `f78e1f8bb687f6f976be9971fe9b5e51973960c7b9b60a149997db7f1b177355` (107,533,139 bytes; expiry 2026-12-02).
Verification:
- Overlay run `44368880314`: SUCCESS.
- PMCS run `44368880331`: FAILURE in regression fixtures after docs were truncated by earlier update calls; full documents restored from last known-good commit `12db63cf4d`. Rerun validator after recovery.
Limitations:
- This is an API-only overlay loaded by the real Horizon host in CI. It does not yet prove launcher runtime discovery on-device, interpreted `.dmod` execution, or full engine integration.


## S-2026-11-18-52 — Increase cold host-loader test budget
Date: 2026-11-18
Commit: `a3d3eef30b8f3a30cad5273874518b8d499839f5`
Goal: Let the real Horizon native host-loader test finish compiling on a cold GitHub Actions runner.
Changes:
- Inspected failed overlay run `44365336841`: API-only overlay build and per-library `$ORIGIN` RUNPATH setup passed.
- Host test hit exit 124 at 180 seconds while Cargo was still compiling the Horizon host dependency graph; no loader assertion had failed.
- Raised that step's timeout to 900 seconds while retaining the workflow's 90-minute job cap.
Verification:
- New overlay run `44368761333` queued on commit `a3d3eef30b`.
- Loader success, archive inspection and artifact upload are not yet verified.
Limitations:
- A successful CI host-loader test does not prove the production launcher resolves the overlay libraries in its actual runtime environment.


## S-2026-11-18-53 — Transactional resource ownership
Date: 2026-11-18
Commit: `863a2d4eb1405368a70f2362831ff41b3caa8165` (registry), `f800daeb7b326cb738ee59c07f7cd798c4f7c699` (module export), `3821c934975ce1b4a549c1e27c9c3dd4e81c3e17` (quota regression correction)
Goal: Establish deterministic, atomic ownership bookkeeping before integrating external Horizon resource registration.
Changes:
- Added a host-independent `ResourceRegistry` and `ResourceTransaction` in `mods/dreammod-package/src/resources.rs`.
- Added strict namespaced resource ID validation, duplicate detection, cross-mod ownership conflicts, per-mod/global quotas, deterministic `remove_owner`, and implicit rollback when a staged transaction is dropped.
- Exported the module from the package crate.
- Corrected the global quota test to fill the registry through multiple owners while respecting the per-mod quota.
Verification:
- Package run `44385572845` on commit `3821c934975c` was in progress during this log update; do not treat tests or Clippy as passed until its conclusion is SUCCESS.
- PMCS validator regression tests passed on run `44385572246`, but the structure/freshness step correctly flagged the five missing document updates; this session appends to all five required documents.
Limitations:
- This registry reserves IDs only. It does not call Horizon, register assets/UI/render resources, execute scripts, or automatically hook lifecycle disable/failure. Those effects require a separately verified host adapter and cleanup path.


## S-2026-11-18-54 — Resource registry lint cleanup
Date: 2026-11-18
Commit: `7edd2f44832ecc2a087c075ca7b88f1fa5bd001e`, `d1d89c64aad80e4f3fa30f13aae98d7358330b1c`
Goal: Resolve strict Clippy findings from the first resource registry validation.
Changes:
- Replaced manual checked-add/maximum fallback with `saturating_add` for global quota calculation.
- Removed an unnecessary mutable binding in the rollback test.
Verification:
- Prior run `44385572845` passed all 51 unit tests and formatting, but Clippy failed on these two findings.
- Follow-up package workflow after `d1d89c64aad8` pending; no clean CI pass claimed yet.
