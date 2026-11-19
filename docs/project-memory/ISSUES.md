# Current Issues

- Metadata registry remains bookkeeping; registrations are not yet connected to live engine systems.
- Generic authorized-command host boundary is verified, but it is not a concrete engine adapter and no live engine mutation exists.
- Package manager approval and runtime explicit activation are separate lifecycle owners and need a coordinator.
- Runtime language decision and production sandbox/security remain open; current Rhai is a candidate runtime.
- No real world/entity/physics/rendering/audio/input/UI/AI/worldgen systems exist yet.
- Host batch atomicity is specified but future concrete adapters must implement and test validation/rollback.

- Cross-domain mock host (world blocks + entity health) is implemented and CI verified. It is test-only; no real world/entity engine adapter yet.

- World/block adapter added but CI pending; native engine storage remains unimplemented.
