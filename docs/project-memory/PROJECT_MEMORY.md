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
Package validation/scanning, interpreted runtime foundation, owner-scoped API registration metadata, explicit runtime activation, provider fault cleanup, and generic authorized-command host boundary. Runtime CI passed on 406b8e5 (format, 26 tests, Clippy); root offline engine tests passed on 4d47315. Stateful block mock passed on c89b038 (format, 27 tests, Clippy). Cross-domain mock (world block writes + entity health) and atomic rollback passed CI on fd4109a: DreamMod Runtime 27 tests, formatting and Clippy; workspace offline tests and workspace Clippy also passed.

## World/block adapter implementation
- Added typed block ID/position/mutation and WorldBlockCommandAdapter in runtime crate (47d6e6e). Converts authorized world.set_block envelopes; validates schema, namespaced IDs, X/Z world-border bounds; stages complete batch before invoking native backend atomic contract. Tests cover successful batch, mod namespaces, invalid final command, unknown payload, bounds, and unsupported commands. CI pending.


## Current unverified work (2026-11-19)
- Hybrid API router and event-scoped script API bridge are implemented in source: `.dmod` requests route through capability authorization and provider selection; one-shot per-mod results arrive as `event.api_results`. Regression tests cover roundtrip, denial, isolation and scalar payload compatibility. CI is not verified.
- Horizon's native mod API is hook-scoped and version-pinned; DreamMod provider calls currently have no scoped host-context argument. Design that context before implementing a real Horizon adapter; keep SDK optional.
- See CURRENT_STATE.md and AI_HANDOFF.md for the current source commit list and exact next steps.
