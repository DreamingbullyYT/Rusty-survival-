# Current Issues

### Atlas declaration integration (2026-11-19)
- Package-level declaration ownership and snapshot integration has been implemented in ModManager and awaits CI validation.
- Core's pinned atlas collect hook now re-submits its snapshot on resource reload. The .dmod runtime still does not populate the Core registry, and resource-pack/sprite validation remains Horizon's responsibility.
- API should remain direct/batched. Atlas JSON parsing, sprite validation and resource publication belong to the host hook/runtime integration.

### Atlas hook signature correction
- First Core compile exposed a mismatch: the generated Horizon collect hook accepts `RegisterTextureAtlasesEvent`, not `Vec<TextureAtlasRegistration>`. Updated implementation to event wrapper; CI is blocked by rustfmt check; compile verification remains pending.


### Runtime-format scope clarification (2026-11-19)
- Production .dmod script interpreter is not connected; Rhai is an isolated prototype.
- No DreamMod WASM runtime or player-native module loader exists. Horizon cold-cache WASM compilation is not adopted under no-player-compilation.
- Trusted native .hmod DreamMod Core is build-time integration, not player-native mod support.
- Future WASM/native player formats need explicit architecture/trust decisions, shared capability-gated Host API and verification before being advertised.

### Shared mods-directory scanner integration — 2026-11-19
- Library scanner is implemented but not connected to the launcher's actual instance `horizon-mods/` path, mod-management UI, permission prompts or runtime activation.
- Runtime test/Clippy/rustfmt verification is pending. The first PMCS attempt failed because required project-memory files were missing updates; those updates are being completed.
- Scanner uses bounded candidate/archive/retained-byte budgets and ignores Horizon `.hmod`; the common folder decision itself is accepted.

### Scanner verification status — formatting rerun (2026-11-19)
- Initial runtime workflow failed at formatting only; no compile, test or Clippy stage executed. Formatting correction has been committed; a fresh run is pending.

- Follow-up runtime run also found two remaining rustfmt-only diffs; they are corrected. The next workflow is the first attempt expected to reach test compilation, but no result is claimed yet.

### Scanner CI status update (2026-11-19)
- Verified: runtime Actions `44543680876` passed rustfmt, 20 tests and Clippy on `4abbd917`; PMCS `44543782146` passed. Remaining issue is instance-path/UI integration, not scanner validation.

### Shared-folder runtime entrypoint (2026-11-19)
- Runtime API can now scan a caller-provided `horizon-mods/` directory and pass validated `.dmod` packages into existing runtime loading. Actual launcher path discovery, permission UI and diagnostics display remain unimplemented. CI pending.

- Runtime scanner-to-loader entrypoint's initial Actions attempt stopped at rustfmt; formatting was corrected. No test or compile failure is established yet; await rerun.

- The second runtime directory-loading CI attempt also failed at rustfmt only (a trailing blank line). Removed; next run pending.
