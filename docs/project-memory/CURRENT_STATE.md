# Current State — DreamMod

Updated: 2026-11-19

## Implemented
- Generic API provider router: Horizon-first default, DreamMod fallback by supported operation, per-domain preference switch, provider hot replacement/removal. Generic contract only; not connected to runtime dispatch.
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

## Verified
- Stateful test-only mock block-world host validates and stages block writes before commit. Tests cover successful writes and batch rollback behavior on invalid input. Runtime CI passed on c89b038: formatting, 27 tests, and Clippy with warnings denied.

## Not implemented / not verified
- No concrete Horizon API adapter or end-to-end Horizon API calls yet; router tests and formatting/CI pending.
- Mock cross-domain host is verified, but is test-only; production adapters and authoritative world/entity state are still absent.

## Remaining
- Concrete engine host adapters and world/entity/physics/render/audio/UI/AI/worldgen subsystems.
- Runtime activation is not unified with package ModManager approvals.
- Host batch atomicity is a contract for future adapters; no real engine mutations/rollback implementation exists.
- Language decision and production sandbox remain open; Rhai is a candidate runtime.

## Next actions
1. Connect ApiRouter to runtime command/provider dispatch and add capability checks at that boundary.
2. Implement a version-pinned Horizon adapter for a small, verifiable API surface, with translations to DreamMod contract types.
3. Keep DreamMod-native adapters for operations Horizon does not expose; then integrate the world/block adapter with real engine storage.
4. Unify package approval and runtime activation through one coordinator; maintain PMCS and verify CI.

- Typed WorldBlockCommandAdapter is now implemented in runtime crate; it translates authorized block commands and delegates atomic commit to a native backend contract. CI pending.

- Typed WorldBlockCommandAdapter has been added and formatted; tests/CI verification pending. Native engine world/chunk storage is still absent.

- Fixed adapter unit-test import: ModId now imports from manifest (106a1c5); runtime CI rerun pending.
