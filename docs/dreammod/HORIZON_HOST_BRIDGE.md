# Horizon host bridge contract

Status: PINNED SDK COLLECTOR IMPLEMENTED AND CORE CHECK VERIFIED; PER-MOD LIFECYCLE WIRING AND SCRIPT EXPOSURE REMAIN OPEN.

Pinned upstream source: `horizon-rust/horizon@0b14f8acfd97fa2c750e42795249dcc981178ca4`.
The SDK is fetched by the pinned `cargo-horizon-mod` workflow and is intentionally not committed under
`.horizon/sdk/`.

## Verified upstream surface

The pinned `horizon-mod-api` crate re-exports `mod_vocab` and provides:

- `Hooks`: global game moments, subscribed by implementing only the required methods in the single
  `#[mod_api::horizon_mod]` impl.
- `ServerHost` and `ClientHost`: side-typed host interfaces.
- `Host`, `LevelHost`, handles, typed registry keys, attachments and `Wire` values.
- Explicit load phases. Registration has a `RegistrationContext` and `Registrar`, but no world Host.
  World operations belong to the World phase.
- The `server_tick` hook is server-side and receives `ServerTickEvent`; its phase distinguishes
  `ServerTickStart` and `ServerTickEnd`. It is a valid candidate for a future server scheduler,
  not an instruction to execute an interpreted script once per entity.

The upstream modding guide also establishes that hook implementations are `Send + Sync`, one instance
can be called from the client and server threads, and some hooks/Parts run on worker threads. Therefore
DreamMod must not assume a single-threaded callback model or hold locks across calls that may dispatch
another hook.

## DreamMod boundary rules

1. Ordinary `.dmod` code never receives `ServerHost`, `ClientHost`, Horizon handles, raw pointers,
   references to game objects, or arbitrary Rust closures.
2. Trusted Core adapters own the Horizon host borrow and convert supported operations into bounded,
   owned DreamMod request/response values.
3. Every privileged adapter method checks the package lifecycle and the exact capability immediately
   before touching Horizon. No raw host handle may escape the adapter.
4. Batch operations are the default for entity/world queries and updates. The interpreter must not be
   called once per entity, block, vertex, or item.
5. Host calls are legal only in the Horizon phase and side documented for that operation. The adapter
   rejects unavailable context rather than manufacturing fallback data or loading chunks implicitly.
6. Hook dispatch must preserve Horizon's event ordering, cancellation, thread and re-entrancy contracts.
   DreamMod's scheduler must not silently reorder or swallow native hooks.
7. Native Core is trusted integration code. It is not itself the sandbox for untrusted `.dmod` scripts.
   The interpreter boundary still needs fuel/operation limits, memory limits, cancellation, fault
   containment and capability enforcement.

## Initial implementation sequence

- Keep the Core bootstrap minimal until the runtime interface and interpreter are selected.
- Implement one narrow adapter operation at a time, with unit tests proving denial happens before the
  Horizon operation and that owned values cannot retain host borrows.
- Add a server tick scheduler only after a concrete runtime can consume queued callbacks safely.
  Tick-start/end semantics and `server_has_time()` must be respected; expensive optional work should
  be skipped when the server has no time.
- Add integration tests against the exact pinned SDK and host loader before claiming the bridge works.

## Explicitly not implemented

No concrete Horizon host operation is currently routed through DreamMod's `HostAdapter`. The generic
callback guard in `mods/dreammod-package/src/lifecycle.rs` is a policy primitive only; callers could
still capture unsafe handles outside it. No interpreter, sandbox, script scheduler, world/entity
adapter, or end-to-end `.dmod` execution exists.


## First owned-value contract: player snapshot

The package crate now defines `bridge::PlayerSnapshot` with only owned fields:
UUID, Minecraft player name, dimension identifier and position. Construction rejects
invalid profile names, malformed dimension identifiers, non-finite coordinates and
coordinates outside the supported world bound. The type exposes read-only getters;
it contains no `PlayerRef`, `EntityRef`, `LevelRef`, host borrow or raw pointer.

`HostAdapter::read_player_snapshot()` is the policy entry point for this operation.
It requires the mod to be Enabled and to hold `entities.read` before invoking the
Horizon-side closure. The closure must resolve and read the player using the current
typed Horizon host, immediately copy the values into `PlayerSnapshot`, and return
only that owned DTO. The adapter rejects invalid snapshots.

Package CI run 44309091208 passed formatting, all package tests and Clippy on source
commit `7bae3eaad6128ff815e657541dd3cd1880dca485`.

Still not implemented: DreamMod Core does not yet supply the closure using
`ServerHost`/Horizon handles, and no interpreter can receive the DTO. The next
integration step is to compile a narrowly scoped Core bridge against the pinned SDK,
then test it through Horizon's hook/host test facilities before exposing it to scripts.


### Bounded batch reads

`PlayerSnapshotBatch` holds at most 256 unique player snapshots. Duplicate UUIDs are rejected, preserving one record per player identity. `HostAdapter::read_player_snapshots()` checks `entities.read` once before one bounded host callback. A Core collector should use Horizon's `host.players().server_players()` in connection order, copy each player's UUID, name, dimension and position while the host borrow is valid, and return the owned batch. It must not retain `PlayerRef` or `EntityRef` values or call the interpreter once per player. The 256 cap is a per-call work/memory bound, not a claim that every server can have at most 256 players; larger sets need explicit paging.


## Core collector and authorization seam (2026-11-18)

The Core module `mods/dreammod-core/src/player_snapshot.rs` now implements the
bounded collector against the pinned SDK:

- Reads `ServerHost::players().server_players()` in Horizon connection order.
- Rejects more than 256 players before per-player field reads.
- Copies UUID, name, dimension identifier and entity position into owned values.
- Rejects unavailable/stale fields, invalid names/identifiers/coordinates and duplicate UUIDs.
- Does not retain Horizon handles in the returned batch.

The collector's public entry point now requires a `PlayerReadAuthorization`
implementation. It checks `entities.read` before touching the host and runs one
bounded collection operation after authorization. Regression tests cover denied
callback non-invocation and exactly-one invocation when authorized.

Core workflow run 44310547586 passed the pinned `cargo-horizon-mod` build,
native mod manifest/source/API validation, host/WASM compilation and source
bundle packaging on commit `b2362c6a1347390f4f330655fb0823e8563d94d8`.

Important limitation: `PlayerReadAuthorization` is an interface seam, not yet
an implementation backed by a particular `ModLifecycle` instance. The runtime
must provide that adapter and call this collector per mod; do not treat the
current seam as end-to-end permission enforcement. No script binding, interpreter,
scheduler or sandbox exists yet.


Core-local tests now cover malformed player names/dimension IDs, non-finite and out-of-range coordinates, duplicate UUIDs, the 256-player limit, and authorization-before-operation behavior. The Core workflow now explicitly runs `cargo fmt --all -- --check` and `cargo test` after the pinned SDK check. The first run of this new test step is pending; source presence is not test verification.


## API performance and package-load budget (2026-11-18)

The Core and package player-batch validators now detect duplicate UUIDs using one contiguous
scratch vector and sorting, instead of allocating tree nodes. The 256-player limit and the
original Horizon connection order are unchanged. This is a bounded allocation optimization;
no benchmark or FPS improvement is claimed.

Package inspection now accepts a monotonic deadline and defaults to 60 seconds. It polls
between ZIP entries and decompression chunks. A future complete DreamMod loader must create
one deadline at load start and pass it through dependency preparation, asset/script parsing
and registration. The current deadline covers archive inspection only; it is not a complete
runtime loader timeout or hard preemption mechanism.
