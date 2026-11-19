# Current Issues

- Metadata registry remains bookkeeping; registrations are not connected to live engine systems.
- Generic authorized-command host boundary and typed WorldBlockCommandAdapter exist, but neither is connected to authoritative engine world/chunk storage.
- Package manager approval and runtime explicit activation are separate lifecycle owners and need a coordinator.
- Runtime language decision and production sandbox/security remain open; current Rhai is a candidate runtime.
- No production world/entity/physics/rendering/audio/input/UI/AI/worldgen adapters exist yet.
- Host batch atomicity is a contract; concrete engine adapters must implement and test validation/rollback.
- Stateful provider hot switching currently changes routing only; migration/snapshot/rollback contracts are required before switching stateful subsystems.
- Script API bridge is implemented but CI is unverified. Remaining tests include provider-error/no-retry, request-count bounds and disabled/unloaded result cleanup.
- Horizon integration blocker: `ApiProvider::invoke` has no scoped host context, but Horizon APIs are hook-scoped. Design a safe engine-neutral context interface before implementing live Horizon operations; do not use global mutable host state or unsafe pointers.