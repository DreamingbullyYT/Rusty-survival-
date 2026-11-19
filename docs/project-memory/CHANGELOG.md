

### Atlas declaration validation groundwork — 2026-11-19
- Added and exported a host-independent atlas declaration/snapshot helper. It validates mod-owned identifiers and definition paths, filters disabled owners, sorts deterministically, and enforces the 32-declaration cap.
- DreamMod Package CI run `44467917911` passed. PMCS run `44467917951` failed because required project-memory files were not updated with this implementation; the correction is being made.
- This is validation/bookkeeping only: it does not parse atlas JSON, publish Horizon atlases, or integrate with Core hooks. User direction is to prefer a direct/batched Core-facing API and avoid needless exposed wrapper layers.
### Atlas declarations — ModManager batch integration (2026-11-19)
- Added atomic `replace_atlas_declarations` for enabled mods and a deterministic `atlas_snapshot` batch that filters to enabled owners.
- Enforces declaration namespace ownership and the global 32-atlas cap; rejected replacement restores the previous declaration batch.
- Disable and failure cascades clean up atlas declaration ownership.
- This is package-manager bookkeeping, not live Horizon atlas registration.

### Core atlas hook bridge — 2026-11-19
- Added a bounded owner-scoped Core atlas registry with authorization callback, namespace/path validation, atomic batch replacement, global cap and deterministic snapshot.
- DreamMod Core now consumes that snapshot in Horizon's pinned `register_texture_atlases` collect hook, re-submitting on each resource reload.
- The interpreter/mod manager is not connected to the Core registry yet; this is a trusted bridge, not end-to-end .dmod functionality.

### Atlas hook signature correction — 2026-11-19
- Corrected Core hook implementation to accept Horizon's generated `RegisterTextureAtlasesEvent` instead of a raw Vec. CI caught the contract mismatch; revalidation is pending.

### Core hook formatting cleanup — 2026-11-19
- Latest Core run `44470917203` failed at rustfmt check: formatting in `atlas_registry.rs` and import order in `lib.rs`. Hook signature correction is present; compilation is not yet confirmed. Overlay run `44470917220` passed and PMCS run `44470999045` passed.


### Runtime format status audit — 2026-11-19
- Clarified from source: .dmod validation/manager/lifecycle exist but script execution does not; Rhai is separate prototype only; no DreamMod WASM runtime or player-native loader.
- Trusted native DreamMod Core is Horizon .hmod build-time integration only. Horizon's cold-cache WASM compilation is not adopted because it conflicts with no player-side compilation.
- PMCS updated to prevent future claims of dual WASM/native player-mod support.

### Shared `horizon-mods/` discovery — 2026-11-19
- Added exported `ModsDirectoryScanner` to the player runtime: direct-child `.dmod` discovery, `.hmod` ignored for Horizon, no recursion, per-file diagnostics, duplicate-ID exclusion, and symlink/non-file rejection.
- Bounded defaults: 256 candidates, 256 MiB aggregate archive input, and 512 MiB retained uncompressed package content.
- Four scanner regression tests added. Runtime Actions/PMCS are being verified; scanner is not wired to launcher instance path or UI.

### Scanner formatting correction — 2026-11-19
- CI initially stopped at rustfmt; formatted the scanner implementation and test fixtures. Runtime tests and Clippy have not yet run on the corrected commit.

- Applied remaining rustfmt corrections after Actions identified two additional formatting diffs; latest runtime test and PMCS runs are pending.

### Shared mods scanner verification — 2026-11-19
- Runtime workflow `44543680876` passed formatting, all 20 tests and Clippy (`-D warnings`) on scanner code commit `4abbd917`.
- PMCS run `44543782146` passed on the documentation-updated HEAD. Scanner is library-level only; actual instance path/UI integration remains open.

### Scanner-to-runtime loading entrypoint — 2026-11-19
- Added runtime methods to scan a caller-provided Horizon mods directory and load validated `.dmod` packages through existing runtime dependency/API/capability checks.
- Report returns successfully loaded mod IDs, scanner issues, and count of ignored `.hmod` files. Added integration test with a Horizon `.hmod` and malformed `.dmod`; CI pending.
- Still no launcher instance-path discovery or UI connection.

- Runtime directory-loading entrypoint's first CI attempt stopped at rustfmt; formatting corrections were applied. Runtime test/Clippy verification is pending.

- Runtime entrypoint's second rustfmt check identified only a trailing blank line in the test module; removed it. Verification pending.

### Scanner-to-runtime integration verified — 2026-11-19
- Runtime Actions `44544390519` passed on `20940cf`: formatting, all 21 tests and Clippy `-D warnings`.
- The runtime now exposes a direct scan-and-load API for a supplied Horizon mods directory. It returns package load IDs, scanner diagnostics and `.hmod` ignore count. The launcher still needs to pass the actual instance path and render diagnostics.

### Valid-package directory load integration test — 2026-11-19
- Added an end-to-end runtime test that writes a valid `.dmod` ZIP into a mixed directory and confirms runtime loads it, ignores `.hmod`, and reports a malformed sibling package. Runtime CI pending.

- The valid-package integration test's initial CI attempt stopped at a trailing rustfmt blank line; removed. Test/Clippy verification remains pending.

### Valid `.dmod` loading verified — 2026-11-19
- Runtime run `44544852796` passed rustfmt, 22 tests and Clippy (`-D warnings`) on `0790dc4`. The new end-to-end test loads a valid package from the shared folder, ignores Horizon `.hmod`, and reports the malformed sibling. Instance-path/UI integration remains pending.


### Independent DreamMod API target — 2026-11-19
- Superseded the mandatory Horizon/HNMods integration decision: DreamMod is now designed as an engine-owned host for Rusty-survival, with no Horizon/.hmod/horizon-mixins dependency.
- Added comprehensive architecture spec `docs/specs/DREAMMOD_ENGINE_INDEPENDENT_API.md` and marked the previous Horizon architecture as historical.
- Added workspace crate `dreammod-contract` with validated ModId/ResourceId/CapabilityId, API versions/ranges, opaque typed handles and structured diagnostics.
- Added workspace membership and push/PR CI trigger for engine checks. Verification is pending; do not claim crate tests or Clippy passed until Actions finishes.

- Corrected an escaped newline in the contract test source; fresh workspace CI is pending.
