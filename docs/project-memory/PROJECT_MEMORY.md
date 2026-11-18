# Project Memory

## Identity
Project: DreamMod
Repository: DreamingbullyYT/Rusty-survival-
Target engine: Horizon
Integration: pinned HNMod/horizon-mixins build-time integration
Runtime model: interpreted .dmod source, no per-mod compilation on player devices

## Mission
Build a powerful, deeply extensible runtime modding platform for Horizon. Mods can add gameplay and engine systems, customize existing behavior, and replace designated subsystems through versioned contracts. Ordinary .dmod packages are loaded and interpreted at runtime without Rust/native compilation, bytecode compilation or a game rebuild on the player's device.

## Distribution model
1. DreamMod Core is integrated into a pinned Horizon revision during the trusted build pipeline through HNMod/horizon-mixins.
2. The resulting prebuilt Horizon distribution contains DreamMod Core.
3. The separate launcher installs and updates that prebuilt distribution.
4. Player-installed .dmod packages contain interpreted source and assets; they are dynamically loaded by DreamMod.

HNMod is not assumed to be a precompiled runtime plugin. Current documented Horizon native-mod workflow builds source against the pinned game revision.

## Ground rules
- Integrate with Horizon through pinned upstream extension points; do not copy Horizon source into this repository.
- Work directly on main; no PRs or feature branches unless the user changes this rule.
- No per-mod compilation on player devices, including hidden bytecode compilation.
- Do not finalize the script language before comparative evaluation and representative benchmarks. Rhai is a candidate only.
- Keep performance-critical loops in Rust/native engine code by default; expose batched/coarse-grained script APIs.
- Design explicit contracts for subsystem replacement, state ownership, scheduling, failure and migration.
- Deep script access must not expose arbitrary unsafe pointers or unrestricted native process privileges.
- Sandbox scripts by default. Capabilities must be explicit and narrow.
- Treat HNMod/native code as trusted build-time code with native privileges, not as a safe sandbox.
- Version every public API and record exact Horizon, mixins and hooks/API revisions.
- No fabricated test results or completion claims.
- Keep session history append-only.
- Launcher implementation is separate and out of scope.
- Existing Minelib/vanilla-engine work is historical, not the active product direction.

## Architecture
The active architecture is specified in docs/specs/DREAMMOD_ARCHITECTURE.md. The previous engine architecture and Minecraft Java 26.3 parity work are superseded for active development, but remain in Git history and existing records for traceability.

## Continuity system
PMCS documents live in docs/project-memory/. Read AI_HANDOFF.md first. Update project memory with every meaningful implementation or architecture change.
