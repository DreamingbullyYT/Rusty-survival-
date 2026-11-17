# Project Memory

## Identity
Project: Rusty Survival Engine
Repository: DreamingbullyYT/Rusty-survival-
Primary platform: Android
Correctness target: Minecraft Java Edition 26.3 observable behavior
Inspiration: Horizon and Minelib engineering ideas only; no Horizon code copying
Strategy: independent Rust engine; optional Minelib component reuse only after audit.

## Mission
Build an independent, modular Rust vanilla engine/runtime with observable behavior matching Minecraft Java Edition 26.3. Keep simulation portable and independently testable. Use Android as the final first-class platform target.

## Scope
Current active scope is the vanilla engine only: world data, world generation, simulation, physics, persistence, rendering, resources, audio, runtime and platform integration. Modding is explicitly out of scope: do not design or implement mod loaders, mod APIs, hooks, mixins or scripting.

## Ground rules
- Correctness/parity before feature breadth or benchmark optimization.
- Horizon is inspiration only; do not copy its source, assets, code structure or implementation.
- Minelib reuse is optional; its root license is AGPLv3. Record file-level provenance and legal obligations before any source reuse. No Minelib source has been copied.
- Record provenance and redistribution status for every asset.
- No undocumented architectural coupling or cyclic dependencies.
- Keep headless simulation independent of Android, graphics APIs, audio devices, UI and filesystem paths.
- Run formatting, unit/integration tests and Clippy in CI for Rust changes.
- No fabricated test results or completion claims.
- Commit directly to main; no PRs or feature branches unless user changes this.
- Keep session history append-only.
- Do not guess version-specific Minecraft Java 26.3 behavior; use reference fixtures or mark unknowns OPEN.

## Source of truth
1. Current source and build configuration.
2. Git history and commit references.
3. Reproducible tests and captured logs.
4. Version-pinned Minecraft Java 26.3 reference fixtures and provenance.
5. Project memory as an index, updated with meaningful changes.

## Continuity system
PMCS documents live in docs/project-memory/. Specifications and audits live in docs/specs/ and docs/audits/. Read AI_HANDOFF.md first.

## Engine foundation
The current workspace contains only `rusty-survival-engine-core` with horizontal coordinate mapping. Its tests establish coordinate invariants only, not Minecraft parity. See docs/specs/ENGINE_TEST_STRATEGY.md and docs/specs/ENGINE_LAYOUT.md.

## Reference test acquisition
Future Java 26.3 oracle tests may resolve official Mojang metadata and download only required artifacts to ephemeral GitHub Actions runner storage. Verify size/hash; never commit, cache, upload, release or otherwise redistribute Mojang files. The public workflow must remain transparent about the download source and method. See docs/specs/REFERENCE_TEST_PIPELINE.md.
