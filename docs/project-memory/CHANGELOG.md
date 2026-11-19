

### DreamMod registration API crate — 2026-11-19
- Added `dreammod-api` to the workspace. It defines owner-scoped registration declarations for commands, event listeners, systems, content, providers and resources, including optional required-feature keys.
- Registration batch is validated atomically; IDs must be in the registering mod's namespace; duplicate IDs rejected; deterministic priority/resource-ID snapshots and owner cleanup provided. Five tests added; verification pending.


### Runtime explicit activation — 2026-11-19
- Changed newly loaded runtime mods to disabled by default and added explicit `enable_mod`. Added regression coverage that inactive mods receive no event dispatch.
- Commit: `b2d6f04ec1d9ca0ff28a7a6308f14a6956ee1b8d`; test assertion correction: `30a4edabbc2dfb237650d57f8fbffeaa65bea456`.
- Status: IMPLEMENTED; not verified. CI stopped at formatting and PMCS validator failures; Rust tests and Clippy were skipped.


### Handle type inference fix — 2026-11-19
- Fixed contract unit test to specify `Handle::<()>::new`; the default generic type was not inferred by Rust in the test.
- Root Rust core checks (format, test, Clippy) passed on commit `1fb4ed6b6272804027aecb14f40f3ae8a3621b45`.


### Fault-disable provider cleanup — 2026-11-19
- Provider registrations are cleared when a script fault threshold disables a mod, whether the fault occurs in event dispatch or provider invocation. Regression test verifies re-enable does not resurrect a stale provider. Verification pending.


### Fault cleanup formatting and CI follow-up — 2026-11-19
- Rust runtime regression-test formatting was corrected after CI flagged style-only differences. Root core checks passed; runtime-specific tests are rerunning.


### Runtime activation and fault-cleanup verification — 2026-11-19
- DreamMod Runtime CI passed 24 tests, formatting and Clippy at `6bae66f43f6e51b9c92301beea34224e9bb03ac6`.
- Root Rust core checks and offline engine test workflow also passed at that commit.


### Authorized command host boundary — 2026-11-19
- Added a generic engine host contract to DreamMod Runtime. The event helper dispatches scripts, then sends only validated/authorized commands as one batch to the host; empty command batches do not call the host.
- Added mock-host regression tests for authorized command delivery and denied-command suppression.
- Commits: 6b914d9 (runtime), 463abba (public exports). Status: IMPLEMENTED; CI verification pending.


### Host boundary rustfmt correction — 2026-11-19
- Fixed rustfmt-only layout in runtime exports and host-boundary tests after the first CI attempt stopped at formatting. Tests and Clippy did not run on that attempt.
- Corrected commits: 925137f and f349bc6. Verification pending.


### Second host-test formatting correction — 2026-11-19
- Runtime CI caught one remaining unformatted chained load call in the denied-command mock-host test. Corrected it; this attempt again stopped before tests/Clippy.
- Fix commit: 406b8e5. Awaiting rerun.


### Authorized command host boundary verified — 2026-11-19
- DreamMod Runtime CI passed on commit 406b8e525eb99ae276633a32336fa1615b1298c3: rustfmt, 26 tests (0 failures), and Clippy with warnings denied.
- Root offline engine test workflow passed on the later memory commit 4d4731573ae63a64dbf747b90084b265d43a87b6. The new host boundary is still a mock-only seam, not live engine execution.


### Mock block-world host — 2026-11-19
- Added a stateful test-only engine host for `world.set_block`: validates namespace/command, integer coordinates and bounds, block identifier, stages the full batch, then commits.
- Added a runtime-to-host integration test for two valid block mutations and an invalid second batch entry proving no partial state mutation.
- Implementation commits: 4067841 and 6d2a232. CI verification pending.


### Formatting and PMCS marker correction — 2026-11-19
- Corrected rustfmt output for the mock host and restored validator-required PMCS headings (Ground rules, Continuity system, Not implemented / not verified, Next actions).
- Implementation correction commit: c89b038. CI verification pending.


### Mock block-world host verified — 2026-11-19
- Runtime workflow passed on c89b038: rustfmt, 27 tests (0 failures), Clippy with warnings denied. This verifies the mock host test only, not a live engine adapter.

## 2026-11-19
- Added typed mock entity-health mutation to mock world host and atomic cross-domain rollback test. CI pending (b82c05b, cd78cb3).

## 2026-11-19
- Verified cross-domain test-only mock host (world block writes + entity health): DreamMod Runtime CI 27 tests, rustfmt and Clippy passed; workspace offline tests and Clippy passed (fd4109a).

## 2026-11-19
- Added typed world/block command adapter and atomic backend contract; test coverage added, verification pending.

## 2026-11-19
- Follow-up commits formatted the new world/block adapter and public exports after CI's rustfmt check; tests pending rerun.

## 2026-11-19
- Fixed test-only private ModId import in adapter tests; CI rerun pending.


### Hybrid Horizon API provider routing — 2026-11-19
- Started hybrid direction: DreamMod API routes to Horizon when a provider advertises a compatible operation, with DreamMod-native fallback for gaps and per-domain preference.
- Added provider abstraction/router, runtime replacement/removal, tests, and architecture note. This is routing scaffolding, not an integrated Horizon adapter; CI pending.
- D-015 supersedes D-014's blanket Horizon integration prohibition while retaining engine independence.


### Hybrid API runtime integration — 2026-11-19
- Fixed router source formatting/content and integrated ApiRouter into DreamModRuntime.
- Added runtime provider registration, removal and per-domain preference methods; added capability-gated API calls requiring mod enabled and recognized operation.
- Added tests for disabled mod, successful routing, missing capability and unknown operation. CI pending.
