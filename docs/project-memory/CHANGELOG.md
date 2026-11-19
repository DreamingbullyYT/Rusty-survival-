

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
