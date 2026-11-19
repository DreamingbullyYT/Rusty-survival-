

### DreamMod registration API crate — 2026-11-19
- Added `dreammod-api` to the workspace. It defines owner-scoped registration declarations for commands, event listeners, systems, content, providers and resources, including optional required-feature keys.
- Registration batch is validated atomically; IDs must be in the registering mod's namespace; duplicate IDs rejected; deterministic priority/resource-ID snapshots and owner cleanup provided. Five tests added; verification pending.
