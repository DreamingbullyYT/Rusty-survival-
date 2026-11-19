

## S-2026-11-19-79 — Registration API foundation
Date: 2026-11-19
- Added the `dreammod-api` crate, depending only on `dreammod-contract`, with owner-scoped registration types for command/event/system/content/provider/resource.
- Registry validates namespaces/owner and duplicates before mutating, supports atomic batches, deterministic priority ordering, feature requirements and owner cleanup.
- Added five unit tests. Replaced batch duplicate map with BTreeSet. CI is pending.
