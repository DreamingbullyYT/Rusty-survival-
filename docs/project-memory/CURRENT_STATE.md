# Current State

## Implemented
- DreamMod package validator, capability/lifecycle manager, resource ownership bookkeeping, entrypoint and asset access.
- Atlas declaration helper is exported; declarations are validated and filtered/sorted in a deterministic snapshot.
- PMCS documentation and validator are maintained as mandatory project continuity.

### Atlas declaration helper — 2026-11-19
- `dreammod-package::atlas` provides validated `AtlasDeclaration` and `AtlasSnapshot` filtering to enabled mod owners.
- Package CI run `44467917911` passed.
- The helper is not wired to ModManager, Core, or Horizon. Keep declarations internal and expose a direct batched snapshot to Core/host hook once the exact pinned hook contract is verified; avoid unnecessary intermediate API layers.

## Not implemented / not verified
- No interpreted .dmod runtime, sandbox, scheduler, or live Horizon registry integration.
- Atlas declarations are not published to Horizon; no live resource reload integration.
- Deep replacement of engine subsystems is a product goal, not currently implemented.

## Next actions
- Fix PMCS regression tests/required document markers.
- Integrate atlas declaration ownership into ModManager and expose one batch snapshot for enabled mods.
- Confirm exact pinned Horizon hook signature before implementing publication.
