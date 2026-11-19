# Project Memory — DreamMod

## Mission
Build an engine-native, highly extensible mod platform for Rusty-survival with interpreted .dmod source/assets, broad subsystem APIs, deterministic lifecycle, permissions and replacement providers.

## Ground rules

## Non-negotiable decisions
- No compilation of ordinary player mods on device.
- Horizon/HNMods is legacy; independent Rusty-survival engine is active target.
- Broad API exposure through validated, engine-owned adapters; never expose raw native pointers.
- Batches and snapshots for performance, capability enforcement at host execution boundary.
- Persistent project memory (PMCS) is the source of continuity. Read AI_HANDOFF, CURRENT_STATE, DECISIONS and SESSION_LOG before continuing; append session entries.
- Do not report tests as verified without CI evidence.

## Continuity system

PMCS is append-only for session history. Read AI_HANDOFF and the current-state/decision records before continuing. Append session entries; do not rewrite previous sessions.

## Implemented and verified snapshot
Package validation/scanning, interpreted runtime foundation, owner-scoped API registration metadata, explicit runtime activation, provider fault cleanup, and generic authorized-command host boundary. Runtime CI passed on 406b8e5 (format, 26 tests, Clippy); root offline engine tests passed on 4d47315. Test-only mock block host verified on c89b038 (format, 27 tests, Clippy). Cross-domain mock host with typed entity-health command and world+entity atomic rollback was added in b82c05b and cd78cb3; verification pending CI.