
- Added `dreammod-api` registration foundation: command/event/system/content/provider/resource descriptors, required-feature metadata, namespace/owner validation, atomic batches, deterministic snapshots and owner cleanup. This is not a live engine registry or script dispatch yet; CI pending.


# Implemented

- Independent DreamMod contract and metadata registry crates exist.
- Package/runtime foundations, Rhai interpreted runtime, capability filtering, command/provider response validation and package scanning exist.
- New packages are now loaded disabled; explicit `enable_mod` activates them. Test added for no dispatch while inactive.

# Not implemented / not verified

- Live engine host adapters for world, entity, physics, renderer, input, UI, audio, AI or world generation.
- End-to-end command execution into native engine systems.
- CI verification of the latest activation change: rustfmt and PMCS checks failed; Rust test/Clippy stages skipped.

# Next actions

1. Correct formatting and PMCS validator markers.
2. Re-run Actions and fix compile/test/Clippy failures.
3. Unify package approval/lifecycle with runtime explicit activation.
4. Implement mock engine host and end-to-end `.dmod` -> interpreted response -> authorization -> adapter test.
