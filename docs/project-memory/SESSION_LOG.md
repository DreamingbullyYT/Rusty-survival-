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
