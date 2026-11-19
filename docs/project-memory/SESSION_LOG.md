

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


## S-2026-11-19-81 — Contract test inference and CI repair
Date: 2026-11-19
- Fixed the contract test generic inference by specifying `Handle::<()>::new`; also made PMCS duplicate-ID regression test select an existing ID and import regex.
- Root Rust core workflow passed formatting, unit tests and Clippy on commit `1fb4ed6b6272804027aecb14f40f3ae8a3621b45`.
- The project-memory validator's regression tests passed, but freshness validation required these memory files to be updated for the source test change. Runtime-specific CI still needs verification.


## S-2026-11-19-82 — Fault disable cleans provider registrations
Date: 2026-11-19
- Fixed runtime fault paths to clear provider registrations when failure threshold disables a mod; applies to event callbacks, invalid response processing and provider invocation.
- Added regression test that registers a broken physics provider, trips fault-disable, re-enables the mod, and confirms the stale provider is gone.
- Verification pending; Actions was triggered for runtime CI.


## Fault-cleanup CI formatting
- [x] Correct runtime test formatting after Actions reported the exact rustfmt diff.
- [ ] Re-run DreamMod Runtime tests and Clippy.


## S-2026-11-19-84 — Lifecycle and provider cleanup verified
Date: 2026-11-19
- DreamMod Runtime CI passed: 24 tests, formatting and Clippy at commit `6bae66f43f6e51b9c92301beea34224e9bb03ac6`.
- Root Rust core format, unit tests and Clippy passed, as did the offline engine test workflow.
- Explicit activation and fault-disable provider cleanup are now verified. Next milestone is lifecycle unification and mock engine-host vertical slice; no live engine adapters yet.


## S-2026-11-19-85 — Authorized command host boundary
Date: 2026-11-19
- Added EngineCommandHost contract and dispatch_event_and_execute helper. Only the runtime's validated and capability-authorized command list is passed to the host; host API requires atomic batch execution.
- Added tests for script command reaching a mock host and denied commands never invoking the host.
- Commits: 6b914d9 and 463abba. CI verification pending; no concrete engine adapter or engine mutations yet.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-86 — Host boundary formatting correction
Date: 2026-11-19
- CI for host-boundary commit 463abba failed at rustfmt before tests or Clippy ran. Applied the formatter's exact layout corrections in runtime exports and the two tests.
- Commits: 925137f and f349bc6. Awaiting fresh runtime CI.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-87 — Final host test formatting correction
Date: 2026-11-19
- The second runtime CI attempt showed one remaining rustfmt change in the denied-command test's load call. Applied the correction in 406b8e5; runtime tests and Clippy have not run yet.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-88 — Host boundary CI verified
Date: 2026-11-19
- DreamMod Runtime CI passed on 406b8e5: rustfmt, 26 tests (0 failures), and Clippy with warnings denied.
- Root offline engine test workflow passed on 4d47315. This verifies the generic boundary and tests only; concrete engine integration remains unimplemented.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-89 — Stateful mock block-world host
Date: 2026-11-19
- Added a test-only MockBlockWorldHost with block writes, position bounds, namespace/name validation and validate-then-commit staging. A two-command test confirms writes apply; an invalid second command in a batch confirms no partial changes.
- Commits: 4067841 and 6d2a232. CI pending.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-90 — Mock host formatting and PMCS structure correction
Date: 2026-11-19
- Applied rustfmt corrections after CI reported formatting-only diffs. Restored the PMCS validator's required section markers in PROJECT_MEMORY and CURRENT_STATE.
- Implementation commit c89b038. CI pending.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-91 — Mock block-world host tests verified
Date: 2026-11-19
- DreamMod Runtime CI passed on c89b038: formatting, 27 tests (0 failed), and Clippy with warnings denied. Mock batch rollback test is verified; no production engine adapter is claimed.

Project continuity log policy: this file is chronological and append-only.


## S-2026-11-19-92 — Cross-domain mock host implementation
Date: 2026-11-19
- Extended the test-only host with entity.set_health, strict handle and health validation, and a staged mutation enum covering blocks and entities.
- Added a successful three-command script batch and a failure test proving both block and entity maps remain unchanged when the final command is invalid.
- Implementation commits b82c05b and cd78cb3. CI verification pending.


## S-2026-11-19-93 — Cross-domain host CI verified
Date: 2026-11-19
- Fixed rustfmt issues from the first CI attempt and committed formatting in fd4109a.
- DreamMod Runtime CI passed: 27 tests, formatting, and Clippy with warnings denied. Workspace offline tests and Clippy also passed; PMCS validation passed on the docs snapshot commit before the implementation change.
- Mock host remains test-only; no production world/entity adapter exists.


## S-2026-11-19-94 — Typed world block adapter
Date: 2026-11-19
- Added public WorldBlockCommandAdapter and typed BlockId/BlockPosition/BlockMutation plus BlockWorldBackend atomic contract.
- Adapter accepts only authorized world.set_block commands, validates strict payload and identifiers, enforces horizontal world-border bounds, stages batch, and delegates atomic commit.
- Unit tests cover conversion/mod block IDs, invalid late mutation, unknown fields, out-of-bounds, and unsupported domain. Commit 47d6e6e; CI pending.


## S-2026-11-19-96 — Rustfmt corrections
- Re-formatted adapter and public exports after first CI reported rustfmt-only diffs. Commit 285f264 and 68a6c13; rerun CI pending.


## S-2026-11-19-97 — Adapter test compile correction
- Runtime test compile reported ModId import visibility; changed test to import ModId from manifest. Commit 106a1c5; verification pending.


## S-2026-11-19-98 — Hybrid Horizon API routing foundation
Date: 2026-11-19
- User changed integration direction: reuse Horizon's new documented API where compatible; DreamMod-native providers cover unsupported features. Keep unified .dmod contracts, runtime hot-swapping and engine independence.
- Added `ApiRouter`, provider descriptors/kinds, per-domain preference, operation support declarations, provider replacement/unregister, and tests for Horizon-first selection, DreamMod fallback, preference switching, failure no-retry and invalid IDs.
- Added `docs/DREAMMOD_HYBRID_API.md` with policy, planned surface map and explicit non-implementation caveats. D-014 superseded by D-015.
- Implementation commits: 66681bc (router), d9f6b6a (exports), f4ce9de/09dadb9 (format cleanup), 173cc89 (architecture doc); PMCS updates follow.
- Not yet connected to runtime dispatch; no concrete Horizon adapter. CI verification pending.


## S-2026-11-19-99 — Hybrid API connected to runtime authorization
Date: 2026-11-19
- Corrected malformed literal newline escapes in `api_router.rs`, replacing it with clean Rust source.
- Integrated ApiRouter into DreamModRuntime; providers can be registered/replaced/unregistered and per-domain preference changed through runtime methods.
- Added `call_api`: mod must be loaded and enabled, authorizer must recognize the API operation, and the mod must have the required capability before dispatch.
- Added runtime tests for disabled mods, successful Horizon provider routing, capability denial and unknown operation denial. CI not yet checked.
- Source commits: 8fd1b0e, d125032, 81232ce, 83ed487, c2694cf.


## S-2026-11-19-100 — Runtime API hardening and boundary review
Date: 2026-11-19
- Added provider descriptor operation validation and runtime operation-name validation; request payload size now bounded by configured payload limit.
- Runtime integration tests also cover oversized payload rejection. Source/formatting/CI checks are still not available from this session; treat as unverified.
- Confirmed the router is callable from the runtime host interface, not directly from Rhai scripts. Direct script bridge/result delivery remains a distinct next task.
- Follow-up commits: c146761, f9befbf, 0c663f9, 86b4ed9.

- Follow-up: also bound serialized provider responses by `max_output_bytes`; runtime test uses a reduced limit to check oversized provider results are rejected. Source commits 9aaa889 and 1f10eea.


## S-2026-11-19-101 — Script API bridge design
Date: 2026-11-19
- Inspected current Rhai interpreter and runtime/router API. API routing is host-callable but no script bindings or result path exist.
- Added `docs/specs/DREAMMOD_SCRIPT_API_BRIDGE.md` describing a deferred, event-scoped request/response bridge: script emits request envelopes; runtime authorizes and routes; correlated results are injected into the next event and consumed once.
- Chose deferred flow for the first version to avoid global synchronous callback context, reentrancy, and cross-mod permission-context hazards.
- Design and PMCS updates only; no implementation or tests claimed. Commit `05fc065` created the design document.


## S-2026-11-19-102 — Script API bridge contract primitives
Date: 2026-11-19
- Added public typed `ApiScriptRequest`, `ApiScriptResult`, and `ApiScriptResultError` envelopes in api_router; request IDs and operation identifiers are validated.
- Added unit tests for valid request serialization round-trip, invalid request ID and operation rejection, and result error serialization round-trip.
- Exported types from runtime crate root.
- Commits: `d820307` (types), `8c1107b` (tests), `f484db8` (exports).
- CI status/workflow run list returned empty for `f484db8`; source verification is pending. This is not a script bridge implementation yet.


## S-2026-11-19-103 — Envelope serialization compile fix
Date: 2026-11-19
- Code review found ApiOperation was not serializable even though the new script request/result envelopes require it. Added Serialize/Deserialize derives and strict unknown-field handling.
- Commit `1ffd548`. GitHub combined status and workflow-run lookups returned no checks/runs; CI remains unverified.


## S-2026-11-19-104 — Script API request bridge implemented
Date: 2026-11-19
- Added `api_requests` to script response schema and request-count/duplicate-ID validation.
- Runtime resolves requests through the existing operation authorizer and mod capability grants before calling ApiRouter; request/result sizes are bounded and errors are returned as typed per-request envelopes.
- Results are one-shot and injected under `event.api_results` on the next eligible callback of the same mod. Disabling clears pending results.
- Added an end-to-end Rhai test: script requests `world.block_get`, fake provider returns data, next tick consumes it and emits an authorized command.
- Commits: `9934644` implementation, `6ee5b26` test; docs/spec and PMCS updated. GitHub checks/run lookups returned empty; CI remains unverified.
- Limitations: deferred (not synchronous), provider calls happen after script response parsing, and script API request batches are not transactionally coupled to command batches.


## S-2026-11-19-105 — Bridge result-budget and fault cleanup hardening
Date: 2026-11-19
- Added aggregate `max_output_bytes` accounting for queued API result envelopes, in addition to per-response size checks. Oversized results become structured response-size errors; error envelopes that cannot fit are omitted to preserve the memory budget.
- Fault-disable through provider invocation now clears pending API results too.
- Added a script-facing capability denial test; expected denial does not disable the mod.
- Commits: `db1189d` (aggregate budget), `4fc88f8` (fault cleanup), `09222b5` (authorization unwrap cleanup), `39d0166` (format cleanup), `26d955b` (denial test). CI status still unavailable.


## S-2026-11-19-106 — Script result isolation test
Date: 2026-11-19
- Added two-mod regression test issuing the same request ID in parallel to `world.block_get`, receiving distinct inputs, and consuming each mod's own result on the next tick.
- Asserted both returned command payloads contain their respective values (1 and 2), proving queues are per-mod rather than global.
- Commit `8c2f410`. GitHub status/workflow lookup still returned empty; CI pending.


## S-2026-11-19-107 — Event payload compatibility
Date: 2026-11-19
- Changed API-result injection so scalar event payloads are preserved unchanged when there are no results; when a result arrives, scalar data is wrapped under `event_payload` with `api_results`.
- Added a unit test for this compatibility behavior.
- Commits `037eaed` and `0e40e51`; CI unverified.


## S-2026-11-19-108 — Horizon adapter context research
Date: 2026-11-19
- Reviewed current official Horizon docs: code mods implement `mod_api::Hooks`; callbacks receive typed host/event contexts. Level operations are exposed through access-specific hosts (read host vs write host), not a global API object. Native API is experimental/version-pinned.
- Architectural consequence: current DreamMod `ApiProvider::invoke` lacks a scoped host context and cannot implement live world operations safely without globals or fabricated/stale snapshots. Next design should pass a scoped, engine-neutral provider context through runtime/router, with Horizon-specific optional adapter and no mandatory SDK dependency in the base runtime.
- Updated `docs/DREAMMOD_HYBRID_API.md` and normalized `CURRENT_STATE.md`. Official references: https://playhorizon.dev/docs/mod-code and https://playhorizon.dev/docs/api/stability.
