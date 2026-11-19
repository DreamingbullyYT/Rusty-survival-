

### Atlas declaration validation groundwork — 2026-11-19
- Added and exported a host-independent atlas declaration/snapshot helper. It validates mod-owned identifiers and definition paths, filters disabled owners, sorts deterministically, and enforces the 32-declaration cap.
- DreamMod Package CI run `44467917911` passed. PMCS run `44467917951` failed because required project-memory files were not updated with this implementation; the correction is being made.
- This is validation/bookkeeping only: it does not parse atlas JSON, publish Horizon atlases, or integrate with Core hooks. User direction is to prefer a direct/batched Core-facing API and avoid needless exposed wrapper layers.