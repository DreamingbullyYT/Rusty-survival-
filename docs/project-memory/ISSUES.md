# Current Issues

- Metadata registry remains bookkeeping; registrations are not yet connected to live engine systems.
- The generic command host contract has been implemented, but no concrete native engine adapter exists. Latest code changes await CI.
- Package manager approval and runtime explicit activation are separate lifecycle owners and need a coordinator.
- Runtime interpreted language choice and production sandbox/security remain open; current Rhai implementation is a candidate runtime.
- No real world/entity/physics/rendering/audio/input/UI/AI/worldgen systems exist yet.
- Project-memory validator freshness should be checked after every implementation commit.
