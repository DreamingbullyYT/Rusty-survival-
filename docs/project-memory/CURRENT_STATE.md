# Current State

## Runtime format status — source audit 2026-11-19
- Player package is .dmod with scripts/*.dream and assets. Validation, manager and lifecycle exist; scripts are not executed by a production runtime.
- Rhai 1.27 AST evaluator is a separate prototype; bytecode and optimization disabled. It is not integrated into Core or selected as the final language.
- DreamMod WASM support is absent: no WASM format/runtime/adapter. Horizon's own WASM cold-cache path compiles through child process/Wasmtime and is not adopted due to the no-player-compilation rule.
- Player native .so/DLL support is absent. DreamMod Core is trusted native Horizon .hmod build-time integration only.
- Therefore the current API is not a dual WASM/native player-mod system.

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
- Core implements the collect hook with the generated event signature. Latest Core Actions failed at rustfmt check; formatting is being corrected before compilation can be verified. No .dmod interpreter is connected to the registry yet.
- Deep subsystem replacement remains a design goal, not implemented.

## Next actions
- Validate package CI and PMCS.
- Confirm the corrected Core hook builds after rustfmt cleanup.
- Verify the event output field in the generated contract.
