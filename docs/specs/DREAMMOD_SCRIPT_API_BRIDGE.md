# DreamMod Script API Bridge — Design

Status: event-scoped bridge implemented in runtime source; CI verification pending.

## Goal
Allow interpreted `.dmod` scripts to request operations from the shared API router without depending on Horizon or a specific provider.

## Initial interface: event-scoped request/response
Scripts return `api_requests` alongside commands and provider registrations. Each request has:
- `request_id`: mod-local correlation ID, validated and length-limited.
- `domain` and `name`: validated API operation identifiers.
- `safety`: `read_only` or `mutating` metadata.
- `input`: JSON value, subject to the runtime request size limit.

The runtime routes requests after script execution and returns a response envelope containing the same request ID, operation, and either a JSON result or a structured error. Results become available in a later event payload under `api_results`. The script must not assume a synchronous return in the first version.

## Execution pipeline
1. Confirm mod is loaded, enabled, on the correct runtime side, and subscribed to the event.
2. Supply the mod's queued API results in event data; clear the consumed queue.
3. Execute Rhai under its operation/call/depth/collection limits.
4. Validate total script output and request count, IDs, operation names, and input size.
5. Authorizer maps the operation to a capability. Unknown operations and missing grants are denied.
6. Router selects a provider by advertised support, domain preference, and priority.
7. Bound response size. Store a result envelope for the next eligible callback of that mod; inject under `event.api_results` and consume once.
8. Provider errors are returned as errors; do not retry another provider automatically, because a mutating provider may have partially applied an operation.

## Security and isolation
- Request IDs are scoped to one mod; no global ID namespace.
- Results are queued only for the requesting mod and cleared when consumed.
- Disable/unload/fault-disable clears pending API results and registrations.
- API requests are never treated as commands unless the host explicitly supports that operation.
- Capability authorization occurs for each request, not just when a package is loaded.
- Safety metadata is descriptive and validated; the authorizer remains the authority.
- Enforce maximum API requests per event, request input bytes, and result bytes. Request count currently shares `max_commands_per_event`; the per-event pending result vector is bounded by that request count.
- Malformed request output fails the callback under the existing fault threshold. Authorization/provider/size denials are returned as per-request error envelopes and do not disable the mod by themselves.
- No raw pointers, direct filesystem/network access, native code, or provider internals are exposed to scripts.

## Why deferred responses first
A synchronous Rhai native callback would need a per-invocation authorization context and mutable router access while a script instance is executing. A global callback/context risks cross-mod context bleed and lock/reentrancy problems. A deferred event-scoped request/response path avoids those hazards, fits the current event-driven runtime, and can later be replaced by a safe async/task API without changing operation IDs or provider contracts.

## Current implementation

- Rhai script output accepts `api_requests` alongside commands/providers.
- Runtime validates request envelopes and duplicate IDs, applies operation authorizer/capability grants, routes via ApiRouter, bounds input/result size and returns stable error envelopes.
- Completed results are injected into the next matching mod callback under `event.api_results`; queue is one-shot and mod-local. Disabling clears pending results.
- Automated test covers a script requesting `world.block_get` through a fake Horizon provider, then consuming the returned data on the next tick to emit a command. Test/format/Clippy CI is not yet verified.

## Acceptance tests
- valid request routes to the preferred provider and result is correlated by request ID.
- unknown operation and missing capability are denied.
- disabled/unloaded mods cannot issue calls and cannot retain results.
- oversized input/output and too many requests are rejected.
- provider error is returned without retry.
- results are isolated between mods and consumed once.
- malformed envelope is rejected.
- request/result queue limits prevent memory growth.

## Next implementation steps
1. Add typed script API request/result envelopes and strict schema validation.
2. Add per-mod pending-result queues and event injection.
3. Process requests through the same authorization and router path as host calls.
4. Add tests for routing, denial, correlation, one-shot result delivery, cleanup and bounds.
5. Only then build the Horizon provider adapter and test a real operation end-to-end.
