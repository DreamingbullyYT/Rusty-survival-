

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
