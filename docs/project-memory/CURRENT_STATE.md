# Current State — DreamMod

Updated: 2026-11-19

## Implemented
- Engine-independent DreamMod contract crate and owner-scoped metadata registry.
- .dmod package validation/scanning and interpreted Rhai runtime foundation.
- Capability filtering and response validation for commands/providers.
- Runtime package loading defaults to disabled; explicit activation is required.
- Fault-disable cleanup removes a mod's provider registrations.
- Generic EngineCommandHost boundary: dispatch produces authorized commands and submits a non-empty batch to a host; host promises atomic apply. Mock-host tests cover delivery and denied command suppression.

## Verified
- Runtime lifecycle/provider cleanup: runtime CI passed 24 tests, formatting and Clippy at 6bae66f43f6e51b9c92301beea34224e9bb03ac6.
- Root Rust core formatting/tests/Clippy and offline engine test workflow passed at that verified commit.

## Not implemented / pending verification
- Latest host-boundary commits 6b914d9 and 463abba have not yet been CI-verified.
- Concrete engine host adapters and world/entity/physics/render/audio/UI/AI/worldgen subsystems.
- Runtime activation still is not unified with package ModManager approvals.
- PMSC freshness validator has to be run on the host-boundary change.

## Next
1. Verify host-boundary formatting, tests and Clippy.
2. Build a mock engine state adapter that applies typed world/entity mutations with batch validation and rollback.
3. Unify package approval and runtime activation.
4. Define persistent mod state, save format/versioning and bounded storage policy (PMCS project continuity is separate from game mod-state persistence).
