# Current State — DreamMod

Updated: 2026-11-19

## Implemented
- Engine-independent DreamMod contract crate and owner-scoped metadata registry.
- .dmod package validation/scanning and interpreted Rhai runtime foundation.
- Capability filtering and response validation for commands/providers.
- Runtime package loading defaults to disabled; explicit activation is required.
- Fault-disable cleanup removes a mod's provider registrations.
- Generic EngineCommandHost boundary: dispatch produces authorized commands and submits a non-empty batch to a host; host contract requires atomic apply. Mock-host tests cover delivery and denied-command suppression.

## Verified
- Runtime CI passed on 406b8e525eb99ae276633a32336fa1615b1298c3: formatting, 26 tests, Clippy with -D warnings.
- Root offline engine test workflow passed on 4d4731573ae63a64dbf747b90084b265d43a87b6.
- Explicit activation and fault-disable provider cleanup were previously verified on 6bae66f43f6e51b9c92301beea34224e9bb03ac6.

## In progress / pending verification
- Stateful test-only mock block-world host now validates and stages block writes before commit. A test covers successful writes and batch rollback behavior on invalid input. Latest implementation commits 4067841 and 6d2a232 await CI.

## Remaining
- Concrete engine host adapters and world/entity/physics/render/audio/UI/AI/worldgen subsystems.
- Runtime activation is not unified with package ModManager approvals.
- Host batch atomicity is a contract for future adapters; no real engine mutations/rollback implementation exists.
- Language decision and production sandbox remain open; Rhai is a candidate runtime.

## Next
1. Verify the mock block-world host tests, then extend the mock to typed entity commands and cross-domain rollback.
2. Unify package approval and runtime activation through one coordinator.
3. Add the first concrete domain adapter, preferably world/block commands, then entity and physics.
4. Keep PMCS fresh and append the session log for each substantial implementation change.
