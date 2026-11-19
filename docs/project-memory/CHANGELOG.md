

### DreamMod registration API crate — 2026-11-19
- Added `dreammod-api` to the workspace. It defines owner-scoped registration declarations for commands, event listeners, systems, content, providers and resources, including optional required-feature keys.
- Registration batch is validated atomically; IDs must be in the registering mod's namespace; duplicate IDs rejected; deterministic priority/resource-ID snapshots and owner cleanup provided. Five tests added; verification pending.


### Runtime explicit activation — 2026-11-19
- Changed newly loaded runtime mods to disabled by default and added explicit `enable_mod`. Added regression coverage that inactive mods receive no event dispatch.
- Commit: `b2d6f04ec1d9ca0ff28a7a6308f14a6956ee1b8d`; test assertion correction: `30a4edabbc2dfb237650d57f8fbffeaa65bea456`.
- Status: IMPLEMENTED; not verified. CI stopped at formatting and PMCS validator failures; Rust tests and Clippy were skipped.
