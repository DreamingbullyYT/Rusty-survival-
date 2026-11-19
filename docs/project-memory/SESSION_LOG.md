## S-2026-11-19-58 — Atlas declarations helper and PMCS repair
Date: 2026-11-19
- Verified `mods/dreammod-package/src/lib.rs` exports `atlas`; helper validates owner/identifier/path, filters disabled owners, rejects duplicates and over-limit batches, and sorts deterministically.
- Package CI `44467917911` passed. PMCS run `44467917951` failed because the required memory docs and regression fixtures were out of sync.
- Direction confirmed: atlas declarations feed a direct/batched Core API, without needless wrapper layers. No Core/Horizon integration or actual atlas publication exists yet.

## S-2026-11-19-59 — PMCS regression repair and ModManager atlas integration planning
Date: 2026-11-19
- Restored required PMCS structural markers and expanded roadmap/current state with truthful DreamMod implementation status.
- PMCS regression fixtures are being corrected: the append-only test must mutate a real marker, and legacy-ID test must reuse an ID present in baseline.
- PMCS remains the mandatory memory system: read AI_HANDOFF, update state/decisions/issues/roadmap/changelog, append session log, and validate in Actions after every substantive change.

This file is chronological and append-only.

## S-2026-11-19-60 — ModManager atlas batches
Date: 2026-11-19
- Added atomic per-enabled-mod atlas declaration replacement and one deterministic enabled-mod snapshot.
- Namespace and atlas-count validation happen before commit; invalid replacement rolls back. Disable and dependency failure remove declarations.
- Package CI and PMCS validation are pending; no live Horizon atlas publication is claimed.

## S-2026-11-19-61 — Core atlas registration hook bridge
Date: 2026-11-19
- Added a trusted Core atlas registry: per-owner atomic batches, authorization callback, namespace/path validation, duplicate prevention, global 32-entry limit and sorted snapshot.
- Core now implements Horizon's pinned collect hook and re-submits the snapshot each time the hook runs (initial load/resource reload).
- Package CI passed for manager integration (62 tests); Core compile and PMCS for this commit are pending. No script runtime populates the registry yet.

## S-2026-11-19-62 — Correct atlas collect event signature
Date: 2026-11-19
- Actions failure showed generated hook expects `RegisterTextureAtlasesEvent`, not raw Vec. Updated the Core implementation to use the generated event wrapper and its atlas output field.
- The new commit triggers Core, overlay and PMCS checks; no pass is claimed yet.

## S-2026-11-19-63 — Core formatting cleanup
Date: 2026-11-19
- Actions review: PMCS `44470999045` passed; API-only overlay `44470917220` passed; Core `44470917203` failed on rustfmt differences only in the shown log tail.
- Corrected hook signature remains in source; compile is not yet verified because formatting check stops the job. Formatting-only cleanup and PMCS updates are next.


## S-2026-11-19-64 — Current modding runtime formats audited
Date: 2026-11-19
- User asked what runtime formats are actually present, not just planned.
- Inspected main source: .dmod validation, ModManager/lifecycle, capability/resource/atlas bookkeeping exist; production script execution is not implemented. Rhai 1.27 AST-walking prototype is separate, unselected and not connected to Core.
- DreamMod has no WASM runtime/format/adapter and no player-native .so/DLL loader. DreamMod Core is trusted native Horizon .hmod build-time integration only.
- Horizon's existing WASM cold-cache route compiles via child process/Wasmtime; not adopted because it conflicts with no-player-compilation.
- Updated PMCS/spec to record distinctions. WASM evaluation must require precompiled modules and the shared permission-checked Host API; native player mods need explicit trust/security decision.
- No implementation code changed; PMCS CI validation pending.

## S-2026-11-19-65 — DreamMod scanner for Horizon's shared mods directory
Date: 2026-11-19
- User confirmed the common folder convention: player `.dmod` files go in Horizon `horizon-mods/`, alongside Horizon `.hmod` files, while format loaders remain separate.
- Implemented `ModsDirectoryScanner` in dreammod-runtime: direct-child only; recognizes `.dmod`; counts and ignores `.hmod`; ignores unrelated files; reports symlink/non-regular/malformed candidates; rejects all duplicate-ID packages; deterministic output.
- Default budgets: 256 `.dmod` candidates, 256 MiB aggregate archive bytes, 512 MiB aggregate retained uncompressed bytes. Added regression tests for mixed extensions, directories, duplicate IDs, budgets and symlink handling.
- Exported scanner API. Not integrated with launcher/game filesystem or UI. Rust toolchain unavailable in this environment; Actions verification pending. PMCS first run failed because required CHANGELOG/ISSUES/SESSION_LOG updates were omitted; corrected docs are now committed.

## S-2026-11-19-66 — Scanner CI formatting correction
Date: 2026-11-19
- Inspected Actions output: the initial scanner runtime workflow failed at `cargo fmt --check`, before tests/Clippy, with formatting-only diffs.
- Applied rustfmt corrections to scanner imports, package-size fold and test fixture/assertions. No functional change intended. Re-triggered validation; results pending.

## S-2026-11-19-67 — Remaining rustfmt corrections
Date: 2026-11-19
- A second format check identified two remaining style differences in the aggregate-retained-bytes fold and limit-test assertions. Applied exact rustfmt output; runtime test rerun is pending.

## S-2026-11-19-68 — Shared mods scanner CI verified
Date: 2026-11-19
- Runtime Actions run `44543680876` on `4abbd917` passed rustfmt, 20 tests (including the scanner tests), and Clippy with warnings denied.
- PMCS run `44543782146` passed on the updated docs HEAD.
- Confirmed the scanner only discovers/validates packages; it is not yet wired into the launcher's instance folder or UI. Continue with path integration and diagnostics UI design.

## S-2026-11-19-69 — Connect scanner to runtime load path
Date: 2026-11-19
- Added `load_from_horizon_mods_dir` plus a scanner-configurable variant to DreamModRuntime. Scan diagnostics and `.hmod` count are retained in the report while validated `.dmod` packages go through existing load/dependency/API/permission checks.
- Added regression test mixing an ignored `.hmod` and a malformed `.dmod`; CI is pending. Caller must still supply the game's instance path; UI integration is not implemented.

## S-2026-11-19-70 — Runtime entrypoint formatting correction
Date: 2026-11-19
- Actions found rustfmt-only diffs in the public re-export ordering, scanner error propagation and test-module trailing whitespace. Corrected these; new runtime workflow is pending.

## S-2026-11-19-71 — Final runtime rustfmt whitespace correction
Date: 2026-11-19
- Third runtime CI attempt showed only one trailing blank line before test-module close. Removed it; new CI run pending.
