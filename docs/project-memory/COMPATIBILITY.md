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

## Shared authorization contract (2026-11-18)
`dreammod-contract` is a local path dependency shared by the standalone Core and package crates. It intentionally has no third-party dependencies and uses Rust 2024 syntax compatible with the pinned Core toolchain 1.99.1. Core continues to compile against the exact pinned Horizon SDK; package validation uses stable Rust. Verify both workflows before treating the new dependency edge as compatible.

## ModManager (2026-11-18)
The manager is implemented in the standalone `dreammod-package` crate and uses the already-pinned SemVer dependency/resolver. It introduces no Horizon SDK dependency and no native host ABI. It uses standard-library `BTreeMap`/`BTreeSet` for deterministic ordering. Package CI run `44363310681` is pending; confirm stable Rust formatting, unit tests and Clippy before marking verified.

## HNMod authorization callback compatibility (2026-11-18)
The pinned Horizon prebuilt SDK stages the HNMod package source but does not stage an arbitrary local Cargo path dependency. A real overlay attempt exposed this as `unresolved import dreammod_contract` (run `44362776804`). The local contract crate was removed. Core now uses a generic one-shot authorization closure, and package ModManager exposes a capability-check method that can be passed as that closure. This avoids adding a non-SDK crate to the native mod package.

### Closure-based authorization bridge (2026-11-18)
Core no longer imports any non-SDK crate for authorization; it accepts a generic one-shot closure. Package ModManager exposes `require_capability` and can supply that closure. The callback signature uses ordinary Rust `FnOnce() -> Result<(), E>` and introduces no ABI or Horizon SDK dependency. The latest extra manager test is in commit `4103668`; post-commit package verification pending.

### Latest package/PMCS verification
- Package run `44364198594` passed on commit `4103668fee`: rustfmt, 42 unit tests, Clippy `-D warnings`.
- PMCS run `44364312080` passed the updated memory set.
- Closure-based Core integration is still pending the active Core and overlay builds.

## In-memory asset reads (2026-11-18)
Asset access is implemented in the standalone package crate using only Rust standard library data structures and the existing archive path validator. `read_asset` returns a borrowed `&[u8]` from the package retained by ModManager; it does not create a file or add Horizon SDK dependencies. Package CI after `f90d0b1` pending.

### Core callback verification — 2026-11-18
The closure-based authorization callback compiled and passed all Core checks against the pinned Horizon SDK in run `44363904945`. The API-only prebuilt overlay also compiled and had Linux `$ORIGIN` RUNPATH setup pass in run `44363904976`; real native host loading remains in progress.

## Entrypoint source bridge (2026-11-18)
`ensure_enabled` and `entrypoint_source` add no third-party dependencies or Horizon API use. They use standard borrowed slices over the validated in-memory package. Package CI after `ee903d7` pending.

### Entrypoint bridge verification
Package run `44365010182` passed all 44 tests, rustfmt and Clippy after adding the activation-gated entrypoint source API. No Horizon SDK dependency was introduced.
