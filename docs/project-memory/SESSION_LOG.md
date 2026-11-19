

## S-2026-11-19-79 — Registration API foundation
Date: 2026-11-19
- Added the `dreammod-api` crate, depending only on `dreammod-contract`, with owner-scoped registration types for command/event/system/content/provider/resource.
- Registry validates namespaces/owner and duplicates before mutating, supports atomic batches, deterministic priority ordering, feature requirements and owner cleanup.
- Added five unit tests. Replaced batch duplicate map with BTreeSet. CI is pending.


## S-2026-11-19-80 — Runtime activation is explicit
Date: 2026-11-19
- Audit confirmed `DreamModRuntime::load_packages` set mods enabled automatically, contradicting the separate default-disabled lifecycle.
- Added default-disabled runtime packages and public `enable_mod`; dispatch/provider behavior remains gated by enabled state. Added test for disabled event dispatch and updated affected tests to activate explicitly.
- CI surfaced rustfmt failures (runtime plus contract crate files) and PMCS marker validation failures. Runtime tests and Clippy were skipped; this change is IMPLEMENTED, not VERIFIED.
- Next: formatting and PMCS repair, then rerun Actions and unify package approval with runtime activation.


Project continuity log policy: this file is chronological and append-only.


This file is chronological and append-only.


## S-2026-11-19-81 — Contract test inference and CI repair
Date: 2026-11-19
- Fixed the contract test generic inference by specifying `Handle::<()>::new`; also made PMCS duplicate-ID regression test select an existing ID and import regex.
- Root Rust core workflow passed formatting, unit tests and Clippy on commit `1fb4ed6b6272804027aecb14f40f3ae8a3621b45`.
- The project-memory validator's regression tests passed, but freshness validation required these memory files to be updated for the source test change. Runtime-specific CI still needs verification.


## S-2026-11-19-82 — Fault disable cleans provider registrations
Date: 2026-11-19
- Fixed runtime fault paths to clear provider registrations when failure threshold disables a mod; applies to event callbacks, invalid response processing and provider invocation.
- Added regression test that registers a broken physics provider, trips fault-disable, re-enables the mod, and confirms the stale provider is gone.
- Verification pending; Actions was triggered for runtime CI.


## Fault-cleanup CI formatting
- [x] Correct runtime test formatting after Actions reported the exact rustfmt diff.
- [ ] Re-run DreamMod Runtime tests and Clippy.


## S-2026-11-19-84 — Lifecycle and provider cleanup verified
Date: 2026-11-19
- DreamMod Runtime CI passed: 24 tests, formatting and Clippy at commit `6bae66f43f6e51b9c92301beea34224e9bb03ac6`.
- Root Rust core format, unit tests and Clippy passed, as did the offline engine test workflow.
- Explicit activation and fault-disable provider cleanup are now verified. Next milestone is lifecycle unification and mock engine-host vertical slice; no live engine adapters yet.


## S-2026-11-19-85 — Authorized command host boundary
Date: 2026-11-19
- Added EngineCommandHost contract and dispatch_event_and_execute helper. Only the runtime's validated and capability-authorized command list is passed to the host; host API requires atomic batch execution.
- Added tests for script command reaching a mock host and denied commands never invoking the host.
- Commits: 6b914d9 and 463abba. CI verification pending; no concrete engine adapter or engine mutations yet.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-86 — Host boundary formatting correction
Date: 2026-11-19
- CI for host-boundary commit 463abba failed at rustfmt before tests or Clippy ran. Applied the formatter's exact layout corrections in runtime exports and the two tests.
- Commits: 925137f and f349bc6. Awaiting fresh runtime CI.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-87 — Final host test formatting correction
Date: 2026-11-19
- The second runtime CI attempt showed one remaining rustfmt change in the denied-command test's load call. Applied the correction in 406b8e5; runtime tests and Clippy have not run yet.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-88 — Host boundary CI verified
Date: 2026-11-19
- DreamMod Runtime CI passed on 406b8e5: rustfmt, 26 tests (0 failures), and Clippy with warnings denied.
- Root offline engine test workflow passed on 4d47315. This verifies the generic boundary and tests only; concrete engine integration remains unimplemented.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-89 — Stateful mock block-world host
Date: 2026-11-19
- Added a test-only MockBlockWorldHost with block writes, position bounds, namespace/name validation and validate-then-commit staging. A two-command test confirms writes apply; an invalid second command in a batch confirms no partial changes.
- Commits: 4067841 and 6d2a232. CI pending.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-90 — Mock host formatting and PMCS structure correction
Date: 2026-11-19
- Applied rustfmt corrections after CI reported formatting-only diffs. Restored the PMCS validator's required section markers in PROJECT_MEMORY and CURRENT_STATE.
- Implementation commit c89b038. CI pending.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-91 — Mock block-world host tests verified
Date: 2026-11-19
- DreamMod Runtime CI passed on c89b038: formatting, 27 tests (0 failed), and Clippy with warnings denied. Mock batch rollback test is verified; no production engine adapter is claimed.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-92 — Cross-domain mock host implementation
Date: 2026-11-19
- Extended the test-only host with entity.set_health, strict handle and health validation, and a staged mutation enum covering blocks and entities.
- Added a successful three-command script batch and a failure test proving both block and entity maps remain unchanged when the final command is invalid.
- Implementation commits b82c05b and cd78cb3. CI verification pending.
