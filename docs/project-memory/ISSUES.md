# Current Issues

- Metadata registry remains bookkeeping; registrations are not connected to live engine systems.
- Generic authorized-command host boundary and typed WorldBlockCommandAdapter exist, but neither is connected to authoritative engine world/chunk storage.
- Package manager approval and runtime explicit activation are separate lifecycle owners and need a coordinator.
- Runtime language decision and production sandbox/security remain open; current Rhai is a candidate runtime.
- No production world/entity/physics/rendering/audio/input/UI/AI/worldgen adapters exist yet.
- Host batch atomicity is a contract; concrete engine adapters must implement and test validation/rollback.
- Stateful provider hot switching currently changes routing only; migration/snapshot/rollback contracts are required before switching stateful subsystems.
- Script API bridge is implemented but CI is unverified. Remaining test is unload cleanup; request-count overflow, disable/re-enable cleanup and provider-error/no-retry regressions are covered.
- Horizon integration blocker: `ApiProvider::invoke` has no scoped host context, but Horizon APIs are hook-scoped. A router-level operation-scoped engine-neutral context contract is implemented in source, but runtime dispatch/host API integration and CI verification remain pending. See `docs/specs/DREAMMOD_SCOPED_API_CONTEXT.md`. Implement/test integration before live Horizon operations. Do not use global mutable host state or unsafe pointers.