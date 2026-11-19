# Current State

## Implemented
- Strict .dmod validation, lifecycle/dependency manager, capability gates, in-memory entrypoint/assets, resource ownership bookkeeping.
- Atlas declaration helper and ModManager batch integration: enabled mods can atomically replace their atlas declarations; snapshot is deterministic, validates namespace/IDs, filters enabled mods and enforces Horizon's 32-atlas limit.
- PMCS continuity system and regression suite are maintained as mandatory project infrastructure.

### Atlas declarations and manager integration — 2026-11-19
- `ModManager::replace_atlas_declarations` validates enabled owner, owner namespace, declarations, and global active atlas budget. Failed replacement rolls back to prior batch.
- `ModManager::atlas_snapshot` creates one deterministic immutable batch for enabled mods.
- Disable/failure removes the owner's declarations; failure cascade cleans dependent declaration batches.
- Package CI is running for this integration; do not claim pass until verified.

## Not implemented / not verified
- No interpreted .dmod runtime, sandbox, scheduler or live Horizon hook.
- Core implements the collect hook with the generated event signature; its compile is pending after a CI-caught signature correction. No .dmod interpreter is connected to the registry yet.
- Deep subsystem replacement remains a design goal, not implemented.

## Next actions
- Validate package CI and PMCS.
- Confirm exact pinned Horizon atlas hook signature and registration context.
- Verify the corrected Core hook build and confirm the event output field in the generated contract.
