# Engine Test Strategy

Status: INITIAL TEST FOUNDATION IMPLEMENTED; full parity harness OPEN.
Target: Minecraft Java Edition 26.3 observable behavior.

## Principles

1. Headless simulation tests must run without Android, a GPU, or a window.
2. Every version-specific behavior test must identify its exact 26.3 reference source and capture method.
3. A passing unit test proves the tested invariant only; it does not prove Minecraft parity.
4. Unknown version-specific constants and mechanics stay OPEN until reference evidence exists.
5. Deterministic tests record seeds, initial state, ordered actions, tick counts, and expected output.
6. Platform, renderer, persistence, and networking adapters receive separate contract/integration tests.

## Test layers

- Unit tests: small deterministic algorithms and value types.
- Domain tests: block states, inventory, physics, fluids, lighting, and tick scheduling against version-pinned fixtures.
- Differential fixtures: compare engine outputs with captured Minecraft Java 26.3 reference outputs.
- Persistence tests: round-trip, migration, corruption recovery, and compatibility tests using legally obtained reference data.
- Platform tests: Android lifecycle, input, storage, audio, and GPU behavior on actual devices.
- Performance tests: repeatable chunk-generation, tick, memory, and frame-time workloads; performance never substitutes for correctness.

## First implemented contract

The rusty-survival-engine-core crate currently implements only horizontal block-to-chunk coordinate mapping:
- Chunk width is 16 blocks.
- Euclidean division/remainder keeps local X/Z in the range 0 through 15 for negative positions.
- Absolute Y is preserved and no dimension or section height is assumed.
- Round-trip and integer-boundary behavior are tested.

These are foundational coordinate invariants, not a complete world/chunk model and not a captured Java 26.3 differential fixture.

## Commands

- cargo fmt --all -- --check
- cargo test --workspace --all-targets
- cargo clippy --workspace --all-targets -- -D warnings

GitHub Actions runs the same checks in .github/workflows/engine-tests.yml.


## Temporary Mojang reference artifact CI

`.github/workflows/reference-artifact-smoke.yml` runs on engine pushes and pull requests. It downloads the official Minecraft Java 26.3 server jar into a unique GitHub Actions runner temporary directory, verifies the metadata SHA-1 and byte count, checks the bundled ZIP CRCs, runs the engine workspace tests and Clippy, and removes the entire temporary directory with an unconditional cleanup step.

Successful smoke run 44331401582 verified a 62,294,556-byte server artifact with SHA-1 `33680f5f2ac32864d6d7cf5e56a705fdb3e05f4c`. The engine workspace currently passed five coordinate tests and the testkit passed five fixture-contract tests.

**This is not a differential parity harness.** The server jar is currently verified as a reference artifact but is not launched or queried by the Rust engine tests. No Java 26.3 behavior is captured or compared yet. A future oracle driver must use a legally obtained official artifact, record the exact version/hash and capture provenance, and compare normalized outputs to deterministic engine fixtures. Never upload the Mojang jar or include it in repository artifacts.
