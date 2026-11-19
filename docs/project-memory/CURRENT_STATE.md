# Current State — DreamMod

Updated: 2026-11-19

## Implemented
- Generic API router integrated into DreamModRuntime: Horizon-first default, provider selection by advertised operation, DreamMod fallback when the preferred provider does not advertise support, per-domain preference, provider hot-replacement/removal.
- Script API bridge: Rhai response may include `api_requests`; runtime validates IDs/operations, checks operation authorization and capability grants, routes through ApiRouter, and injects results into the next eligible callback under `event.api_results`.
- API result queues are mod-local and one-shot. Request count uses `max_commands_per_event`; per-request payload and response bytes are bounded; aggregate queued result bytes are capped by `max_output_bytes`. Disable and fault-disable clear pending results.
- Script-facing API errors are structured; authorization/provider/size failures do not themselves disable the mod.
- Scalar event payloads remain unchanged before any API result arrives. Once results exist for a scalar payload, it is wrapped under `event_payload` alongside `api_results`.
- .dmod package validation/scanning, interpreted Rhai runtime foundation, capability filtering, command/provider response validation, and explicit default-disabled activation.
- Generic `EngineCommandHost` boundary: authorized commands reach host in a batch whose contract requires atomic application; test-only mock hosts cover delivery and validation rollback.
- Typed `WorldBlockCommandAdapter` translates authorized `world.set_block` into validated mutations and delegates atomic application to a backend contract.

## Previously verified
- Runtime CI passed on `406b8e5`: format, 26 tests, Clippy with `-D warnings`.
- Runtime CI passed on `c89b038`: format, 27 tests, Clippy.
- Runtime CI passed on `fd4109a`: 27 tests, format, Clippy; root offline engine tests and Clippy passed.
- Explicit activation/fault-disable cleanup was previously verified on `6bae66f`.

## Latest verification status
- Script API bridge code and tests are committed, but current source commits have no reported GitHub combined statuses/workflow runs in the available lookup. Local Cargo/Rust toolchain is unavailable in this environment. Latest bridge CI is therefore **unverified**.
- Tests added: script request → provider → next-event result → command; capability denial delivered without disabling; two-mod result isolation using same request ID; scalar payload compatibility.
- No concrete Horizon provider/adapter or end-to-end live game calls exist.

## Remaining
- Provider invocation currently executes synchronously after script dispatch; a slow provider can block the event/tick. No async/task bridge yet.
- `ApiProvider` currently has no scoped Horizon host context. A live adapter needing `LevelHost`/`LevelWriteHost` must not use globals or pretend it has context; design a safe context-passing interface before implementing real world operations.
- No actual engine world/chunk/entity/physics/render/audio/UI/AI/worldgen adapters or authoritative storage are connected.
- Runtime activation is not unified with package manager approvals. Rhai is still a prototype and production sandbox/language decision remains open.

## Next actions
1. Verify runtime rustfmt/tests/Clippy through Actions or a Rust-capable environment; repair failures.
2. Design and implement a scoped provider execution-context interface, compatible with optional Horizon adapters and native DreamMod providers.
3. Build a small version-pinned Horizon adapter vertical slice using a documented hook/host operation; advertise only tested operations.
4. Implement DreamMod-native providers for gaps, then connect block adapter to actual engine storage.
5. Unify package approval and runtime activation; keep PMCS fresh.

## Script API bridge implementation — 2026-11-19
- Source commits: `9934644`, `6ee5b26`, `db1189d`, `4fc88f8`, `09222b5`, `39d0166`, `26d955b`, `8c2f410`, `037eaed`, `0e40e51`.
- PMCS updates follow. CI is not confirmed.
