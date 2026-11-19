# Current State — DreamMod

Updated: 2026-11-19

## Implemented
- Generic API provider router integrated into DreamModRuntime: Horizon-first default, DreamMod fallback by advertised operation, per-domain preference switch, provider hot replacement/removal. Runtime host API calls require enabled mod, recognized operation, granted capability and bounded request and response sizes. The event-scoped script bridge is implemented: `.dmod` callbacks can emit `api_requests`; results are capability-checked, routed, bounded, and injected once into that mod's next event as `api_results`. Concrete Horizon provider still not implemented.
- Engine-independent DreamMod contract crate and owner-scoped metadata registry.
- .dmod package validation/scanning and interpreted Rhai runtime foundation.
- Capability filtering and response validation for commands/providers.
- Runtime package loading defaults to disabled; explicit activation is required.
- Fault-disable cleanup removes a mod's provider registrations.
- Generic EngineCommandHost boundary: dispatch produces authorized commands and submits a non-empty batch to a host; host contract requires atomic apply. Mock-host tests cover delivery and denied-command suppression.

## Verified
- Runtime CI passed on 406b8e525eb99ae276633a32336fa1615b1298c3: formatting, 26 tests, Clippy with -D warnings.
- Root offline engine test workflow passed on 4d4731573ae63a64dbf747b90084b265d43a87b6.
- Explicit activation and fault-disable provider cleanup were previously verified on 6bae66f43f6e51b9c92301beea34224e9bb03ac6.

## Verified
- Stateful test-only mock block-world host validates and stages block writes before commit. Tests cover successful writes and batch rollback behavior on invalid input. Runtime CI passed on c89b038: formatting, 27 tests, and Clippy with warnings denied.

## Not implemented / not verified
- No concrete Horizon API adapter or end-to-end Horizon calls yet.
- Latest script bridge source and regression test are committed; format/test/Clippy CI is unverified.
- Mock cross-domain host is verified, but is test-only; production adapters and authoritative world/entity state are still absent.

## Remaining
- Concrete engine host adapters and world/entity/physics/render/audio/UI/AI/worldgen subsystems.
- Runtime activation is not unified with package ModManager approvals.
- Host batch atomicity is a contract for future adapters; no real engine mutations/rollback implementation exists.
- Language decision and production sandbox remain open; Rhai is a candidate runtime.

## Next actions
1. Verify latest script bridge formatting, runtime tests, and Clippy in CI; fix any failures.
2. Implement a version-pinned Horizon adapter for a small, verifiable API surface, with translations to DreamMod contract types.
3. Build DreamMod-native adapters for operations Horizon does not expose; integrate world/block adapter with real engine storage.
4. Unify package approval and runtime activation through one coordinator; maintain PMCS and verify CI.

- Typed WorldBlockCommandAdapter is now implemented in runtime crate; it translates authorized block commands and delegates atomic commit to a native backend contract. CI pending.

- Typed WorldBlockCommandAdapter has been added and formatted; tests/CI verification pending. Native engine world/chunk storage is still absent.

- Fixed adapter unit-test import: ModId now imports from manifest (106a1c5); runtime CI rerun pending.


## Script API bridge design (2026-11-19)
- Design document added: `docs/specs/DREAMMOD_SCRIPT_API_BRIDGE.md`.
- Proposed first version is deferred/event-scoped request-response rather than synchronous Rhai callbacks; avoids shared callback context and reentrancy hazards.
- Requests/results are not implemented. Runtime API remains host-callable only; no script-facing API is available yet.
- Next acceptance gates: result isolation, one-shot delivery, disabled/unloaded cleanup, request/result bounds, provider errors without retry, and capability denials.


## Script API bridge progress
- Added typed serializable `ApiScriptRequest`, `ApiScriptResult`, and `ApiScriptResultError` envelopes plus request ID/operation validation.
- Unit tests cover valid request serialization round-trip, invalid request IDs/operation identifiers, and result error-envelope round-trip.
- These are contract primitives only. Script response parsing, dispatch routing, capability checks for script requests, per-mod queues, and event result injection are not implemented.
- CI for commit `f484db8` is not visible/confirmed; formatting, tests, and Clippy are unverified.


- Follow-up fix: `ApiOperation` now derives serde serialization/deserialization, required by the new envelopes. Latest source commit `1ffd548`; CI still unconfirmed (no statuses/workflow runs reported).


## Script API bridge implementation — 2026-11-19
- ScriptResponse now accepts `api_requests`; request validation checks envelopes and duplicate IDs.
- Each request is mapped through CommandAuthorizer and capability grants before routing; stable per-request errors cover denied/unknown/unavailable/provider-failed/size-limit cases.
- Provider results are stored for one next callback and injected as `event.api_results` for that mod only; disabling clears the queue.
- Added Rhai end-to-end test request → fake provider → next tick result → command.
- Commits: `9934644` runtime bridge; `6ee5b26` regression test. CI is not verified (status and workflow run lookups returned empty).


- Bridge hardening: aggregate pending-result serialization is capped by `max_output_bytes`; if the result envelope exceeds budget it becomes `response_too_large` or is omitted if even the error envelope cannot fit. Provider-callback fault-disable now also clears pending results.
- Added a capability-denial regression test; denial is surfaced to the script as a structured error and does not disable it. CI remains unverified.
