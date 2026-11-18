# Horizon host bridge contract

Status: API SURFACE INSPECTED AT THE PINNED REVISION; DREAMMOD BRIDGE NOT IMPLEMENTED.

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
