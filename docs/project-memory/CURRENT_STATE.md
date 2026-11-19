# Current State

## Runtime format status — source audit 2026-11-19
- Player package is .dmod with scripts/*.dream and assets. Validation, manager and lifecycle exist; scripts are not executed by a production runtime.
- Rhai 1.27 AST evaluator is a separate prototype; bytecode and optimization disabled. It is not integrated into Core or selected as the final language.
- DreamMod WASM support is absent: no WASM format/runtime/adapter. Horizon's own WASM cold-cache path compiles through child process/Wasmtime and is not adopted due to the no-player-compilation rule.
- Player native .so/DLL support is absent. DreamMod Core is trusted native Horizon .hmod build-time integration only.
- Therefore the current API is not a dual WASM/native player-mod system.

## Implemented
- Strict .dmod validation, lifecycle/dependency manager, capability gates, in-memory entrypoint/assets, resource ownership bookkeeping.
- Atlas declaration helper and ModManager batch integration: enabled mods can atomically replace their atlas declarations; snapshot is deterministic, validates namespace/IDs, filters enabled mods and enforces Horizon's 32-atlas limit.
- PMCS continuity system and regression suite are maintained as mandatory project infrastructure.

### Atlas declarations and manager integration — 2026-11-19
- `ModManager::replace_atlas_declarations` validates enabled owner, owner namespace, declarations, and global active atlas budget. Failed replacement rolls back to prior batch.
- `ModManager::atlas_snapshot` creates one deterministic immutable batch for enabled mods.
- Disable/failure removes the owner's declarations; failure cascade cleans dependent declaration batches.
- Package CI is running for this integration; do not claim pass until verified.

## Not implemented / not verified
- No interpreted .dmod runtime, sandbox, scheduler or live Horizon hook.
- Core implements the collect hook with the generated event signature. Latest Core Actions failed at rustfmt check; formatting is being corrected before compilation can be verified. No .dmod interpreter is connected to the registry yet.
- Deep subsystem replacement remains a design goal, not implemented.

## Next actions
- Validate package CI and PMCS.
- Confirm the corrected Core hook builds after rustfmt cleanup.
- Verify the event output field in the generated contract.

## Shared player mods directory decision — 2026-11-19
- Accepted D-013: player `.dmod` packages will be placed in Horizon's existing `horizon-mods/` directory alongside Horizon `.hmod` packages.
- DreamMod must dispatch only `.dmod`; Horizon continues handling `.hmod`. Do not rename, intercept or process Horizon packages. DreamMod Core is trusted native `.hmod` build integration, distinct from player `.dmod` packages.
- Directory convention is decided, but a scanner integration into the instance filesystem is not implemented or verified. Next: build a safe deterministic scanner, diagnostics and mixed-extension regression tests.

## Shared `horizon-mods/` scanner — implementation added (2026-11-19)
- Added `ModsDirectoryScanner` in `dreammod-runtime`: scans only direct children, recognizes `.dmod` case-insensitively, ignores `.hmod` (counted for diagnostics) and unrelated files, and does not recurse.
- Reports symlinked/non-regular `.dmod` candidates without following them; malformed packages become per-file issues.
- Bounded by default to 256 candidates, 256 MiB aggregate archive input, and 512 MiB aggregate retained uncompressed package content; each package still uses existing package-loader limits.
- Duplicate package IDs reject every conflicting candidate rather than choosing one by filename.
- Added tests for mixed `.dmod`/`.hmod`, unrelated files and directories, duplicate IDs, directory limits and symlink behavior. Runtime CI is triggered; results are not yet verified. This scanner is a library API and is not yet wired to the actual launcher/game instance path.

- Scanner's first two CI attempts stopped at rustfmt only; the reported remaining formatting diffs have been corrected. Fresh runtime CI is pending.

### Scanner verification — 2026-11-19
- Runtime Actions run `44543680876` passed rustfmt, 20 tests and Clippy `-D warnings` on scanner implementation commit `4abbd917`.
- PMCS run `44543782146` passed on the latest documentation HEAD. No launcher/instance path integration is claimed.

### Runtime scanner-to-loader connection — 2026-11-19
- Added `DreamModRuntime::load_from_horizon_mods_dir` and a custom-scanner variant. It scans the common folder, passes validated packages to the existing dependency/API/capability-checked `load_packages` transaction, and returns loaded IDs plus per-file scan issues and ignored `.hmod` count.
- Added a runtime integration regression test for mixed Horizon `.hmod` and malformed `.dmod` inputs. CI is pending; this remains a caller-supplied directory API, not automatic launcher instance discovery.

- Scanner-to-runtime loading entrypoint's first CI attempt failed at formatting only. Corrections applied; fresh runtime CI pending.

- Second runtime directory-loading CI attempt found one remaining rustfmt whitespace diff; removed and awaiting a clean run.

### Scanner-to-runtime verification — 2026-11-19
- Runtime Actions `44544390519` passed formatting, 21 tests and Clippy on `20940cf`, including the new directory-loading integration test. PMCS for the result documentation is pending.
- The API scans and loads a caller-supplied path. Launcher instance discovery and UI diagnostics remain unimplemented.

- Added a valid generated `.dmod` ZIP end-to-end test for the runtime directory loader, alongside ignored `.hmod` and malformed-package diagnostic assertions. CI pending.

- Valid-package integration test first CI attempt was rustfmt-only; trailing whitespace corrected and rerun pending.
