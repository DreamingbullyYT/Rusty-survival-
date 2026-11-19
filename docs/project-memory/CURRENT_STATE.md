

### Atlas declaration helper — 2026-11-19
- `dreammod-package::atlas` is exported and provides validated `AtlasDeclaration` plus deterministic `AtlasSnapshot` filtering to enabled mod owners.
- Package CI run `44467917911` passed. PMCS run `44467917951` failed due missing required memory updates for the code commit; docs are being amended.
- The helper is not wired to ModManager, Core, or Horizon. Current direction: keep declarations internal and provide a direct batch snapshot to Core/host hook when the exact pinned hook contract is confirmed; avoid unnecessary intermediate API layers.