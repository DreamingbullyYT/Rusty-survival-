# Project Goal and Non-Negotiable Rules

This file is the canonical, durable statement of the project's goal and constraints. Read it at the start of every session before planning or editing code. If conversation context is compressed or lost, this file takes precedence over guesses or reconstructed summaries. Update it only through an explicit, documented decision; never silently weaken these requirements.

## Main goal

Refactor the Horizon vanilla engine in this repository into a lean, maintainable implementation that achieves **1:1 behavioral parity with Minecraft: Java Edition**.

The pinned upstream architecture documentation names Minecraft: Java Edition 26.3 as the vanilla target. Use that as the initial parity baseline unless a later explicit project decision changes the target version.

"1:1 parity" means matching observable vanilla behavior, not copying Java source line-for-line. The target includes, as applicable to the selected Java Edition baseline:
- gameplay rules and mechanics;
- player movement, collision, physics, damage and status effects;
- blocks, block states, interactions, fluids and redstone;
- items, inventory, crafting, recipes and equipment;
- entities, mobs, AI, spawning and drops;
- world generation, dimensions, time, weather and environmental simulation;
- commands, game rules, screens/HUD behavior, audio and rendering results;
- save/load semantics, data formats and world persistence;
- client/server behavior and protocol compatibility;
- edge cases, timing, deterministic behavior where vanilla requires it, and bug-compatible quirks.

Build a parity test matrix from the exact target version and verify behavior against it. Do not claim full parity merely because the game starts, looks similar, or passes a small subset of tests. Record known gaps explicitly.

## Minimal implementation rule

Keep the codebase as small as reasonably possible **while preserving 1:1 behavior, correctness, maintainability and performance**.

- Prefer the smallest correct implementation; do not add layers, wrappers, frameworks, crates, configuration systems or abstractions without a demonstrated need.
- Reuse correct existing code and upstream facilities before implementing anything new.
- One canonical implementation per responsibility. Avoid exact, structural and conceptual duplication.
- Do not confuse fewer lines with better design: no code-golf, unreadable compression, unsafe shortcuts or removal of necessary validation.
- Optimizations are welcome only when they preserve vanilla-observable behavior. A faster approximation that changes gameplay is not parity.
- Keep dependencies and patches minimal, but never at the cost of correctness or required compatibility.
- Every proposed simplification must be checked against callers, data flow, tests and the parity matrix before acceptance.

## Mojang assets

Use the corresponding Mojang/Minecraft Java Edition assets as the visual/audio/data reference and reuse them where legally permitted and available from a legitimate Minecraft installation or other authorized source. Do not assume that possessing or referencing assets grants redistribution rights; do not commit proprietary assets into this repository unless the applicable license/permission clearly allows it.

Assets do not implement game logic. Matching textures, models, sounds or names is not evidence of behavioral parity.

## Scope and sequencing

- Current phase: vanilla Horizon engine only.
- Do not work on the launcher, Android port/product features, Horizon extras or platform-only features as refactor targets.
- Do not design or implement the project's custom Modding API yet. It comes only after the engine refactor is complete and stable.
- Keep upstream code needed for building or reference where necessary, but do not mistake its presence for permission to refactor out-of-scope layers.
- Work in this repository: `DreamingbullyYT/Rusty-survival-`.

## Required engineering workflow

Before adding, changing, moving or deleting code:
1. Search the entire repository for existing implementations and ownership of the same responsibility.
2. Inspect upstream architecture, manifests, call sites, data flow and tests.
3. Check exact, structural and conceptual duplication; treat detector output as candidates, not proof.
4. Identify the single canonical owner and dependency direction.
5. Define or update the relevant Java Edition parity cases before changing behavior.
6. Make a focused, reversible change.
7. Run relevant unit, integration, compatibility, layer and parity tests; report exactly what passed and failed.
8. Update the system registry, decisions, task ledger and test results as applicable.
9. Append a chronological entry to `project-memory/MASTER_LOG.md`.

Never delete code solely because a similarity detector reports it. Confirm semantics and all usages first. Never invent test results or claim work was completed without tool-confirmed evidence.

## Permanent project memory rules

- At the beginning of every session, read `project-memory/START_HERE.md`, this file, and the memory documents it lists.
- Check the actual repository branch, commit and working tree; memory can be stale.
- `project-memory/MASTER_LOG.md` is strictly append-only: only add new entries at the end. Never delete, replace, reorder, rewrite or summarize away existing entries.
- Keep the other memory files consistent with the real repository state. Preserve historical decisions; if a decision changes, append a superseding decision with its reason.
- After every meaningful technical operation, update the relevant memory files in the same work sequence.
- If a context summary conflicts with the repository's canonical documents, inspect history and actual code; do not silently choose whichever is convenient.

## Completion standard

The engine refactor is not complete until the target systems have been inventoried, ownership and dependencies are understood, confirmed duplication is resolved, the parity matrix has been exercised across all in-scope vanilla systems, known deviations are documented, and the build/test suite is green. Only then may the next phase (custom Modding API design) begin.
