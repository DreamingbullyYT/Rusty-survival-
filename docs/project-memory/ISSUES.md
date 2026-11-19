# Current Issues

- Metadata registry remains bookkeeping; registrations are not yet connected to live engine systems.
- Generic authorized-command host boundary is verified, but it is not a concrete engine adapter and no live engine mutation exists.
- Package manager approval and runtime explicit activation are separate lifecycle owners and need a coordinator.
- Runtime language decision and production sandbox/security remain open; current Rhai is a candidate runtime.
- No real world/entity/physics/rendering/audio/input/UI/AI/worldgen systems exist yet.
- Host batch atomicity is specified but future concrete adapters must implement and test validation/rollback.

- Cross-domain mock host (world blocks + entity health) is implemented and CI verified. It is test-only; no real world/entity engine adapter yet.

- World/block adapter added but CI pending; native engine storage remains unimplemented.

- Adapter implementation initially failed rustfmt checks; formatting corrected in 285f264/68a6c13, CI rerun pending.

- Corrected test-only ModId import after CI compile failure; pending re-verification.

- Runtime hybrid API routing is implemented with enabled-mod and capability checks; no concrete Horizon adapter/end-to-end calls exist yet; CI pending.
- Hot switching stateful providers is only route selection today; state migration/rollback contracts are required before switching stateful subsystems.

- API router host integration is now implemented but .dmod scripts cannot call it directly yet; a safe synchronous/asynchronous script bridge and API result handling are needed.


- Script API bridge design exists, but only as a proposal. Scripts cannot issue API requests yet. The proposed deferred response flow must be implemented and tested before claiming script-facing API access.


- Typed script API request/result envelopes have been added, but their processing pipeline remains absent; CI for `f484db8` is unconfirmed.


- Fixed a missing serde derive on ApiOperation required by serializable script request/result envelopes (commit `1ffd548`); CI verification remains pending.


- Script-facing API bridge has now been implemented as event-scoped `api_requests`/`api_results` with authorization and routing; regression test added. CI remains unverified, and bridge error/isolation/queue-limit coverage should be expanded.

- Event-scoped API bridge has per-mod one-shot results, request/result bounds and scalar payload compatibility; regression tests added for roundtrip, denial, isolation, and scalar injection. Latest CI is still not confirmed.

- Horizon integration blocker identified: ApiProvider currently has no scoped host context, while Horizon mod APIs are hook-scoped. Design a safe provider context interface before implementing real-world adapters; avoid global/unsafe host access. This keeps the Horizon SDK optional.
