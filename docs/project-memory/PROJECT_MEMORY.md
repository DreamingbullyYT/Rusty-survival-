# Project Memory — DreamMod

## Mission
Build an engine-native, highly extensible mod platform for Rusty-survival with interpreted .dmod source/assets, wide subsystem APIs, deterministic lifecycle, permissions and replacement providers.

## Non-negotiable decisions
- No compilation of ordinary player mods on device.
- Horizon/HNMods is legacy; independent Rusty-survival engine is active target.
- Broad API exposure through validated, engine-owned adapters; never expose raw native pointers.
- Batches and snapshots for performance, capability enforcement at host execution boundary.
- Persistent project memory (PMCS) is the source of continuity. Read AI_HANDOFF, CURRENT_STATE, DECISIONS and SESSION_LOG before continuing; append session entries.
- Do not report tests as verified without CI evidence.

## Implemented snapshot
Package validation/scanning, interpreted runtime foundation, owner-scoped API registration metadata, explicit runtime activation, provider fault cleanup, and the generic authorized-command host boundary. The latest host boundary commits 6b914d9 and 463abba are awaiting CI verification. No concrete world/entity/physics adapter exists yet.
