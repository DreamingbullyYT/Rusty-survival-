# Compatibility Policy

## Active target
DreamMod integrates with Horizon. Compatibility is defined by exact pinned revisions, not a broad claim that all Horizon versions work.

Current baseline (2026-11-18):
- Horizon Nightly: `0b14f8acfd97fa2c750e42795249dcc981178ca4` (Nightly 2026-11-18; published SDK `sdk-0b14f8acfd97fa2c750e42795249dcc981178ca4`).
- horizon-mixins: `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`.
- Game: `26.3`.
- Rust toolchain: `1.95.0`.
- Core native manifest: API `0.2`, no extra capabilities, API-only (no mixins/internals/bridge/loop declarations).

Every Core build must record:
- Horizon commit/release
- horizon-mixins commit
- hooks/mod API revision
- Rust toolchain and target
- DreamMod Core version
- supported .dmod API range

## .dmod compatibility
Each package declares a DreamMod API range. The loader rejects incompatible packages before activation and reports the reason. Stable APIs are versioned and deprecated APIs receive a migration window.

## Distribution compatibility
DreamMod Core is integrated at build time. A Core native integration update requires a new prebuilt Horizon distribution. Ordinary interpreted .dmod script/data updates do not require rebuilding Horizon.

## Historical compatibility records
The former Minecraft Java Edition 26.3, protocol 777, data version 5023 and Android-first standalone-engine records describe a superseded prototype direction. They are not DreamMod compatibility claims.


## Performance/load compatibility notes (2026-11-18)
- The snapshot optimization does not alter the public DTO, field values, ordering, batch maximum or Horizon API signatures.
- The 60-second load deadline is a package-inspection policy, not a change to Horizon's own .hmod loader. The future DreamMod loader must propagate one monotonic deadline across all .dmod load stages.
- No script language/runtime compatibility change and no measured performance claim is made.


## Fixed-buffer snapshot implementation (2026-11-18)
The Core/package duplicate UUID check uses a fixed stack array sized to MAX_PLAYER_SNAPSHOT_BATCH (256). This does not change public DTO fields, ordering, capability names, Horizon API signatures or package compatibility. It relies on the existing hard batch bound; any future increase must revisit stack usage and the scratch buffer size.


## Arc-backed snapshot batches (2026-11-18)
PlayerSnapshotBatch retains the same public getters, item ordering, field values and 256-player bound. Its internal storage changes from Vec to Arc<[PlayerSnapshot]>. Clone now shares immutable storage; no mutable access or Horizon handle is exposed. This is an internal Rust contract change, not a change to the .dmod manifest or script language.
