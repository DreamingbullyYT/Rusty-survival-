

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
