# DreamMod

DreamMod is a deeply extensible, interpreted runtime modding platform integrated into a pinned Horizon build.

## Core goals
- Player-installed `.dmod` packages execute as interpreted source at runtime.
- No Rust/native compilation, bytecode build, or game rebuild is required for ordinary mod installation or updates.
- DreamMod Core is integrated into Horizon once at build time through HNMod/horizon-mixins; the launcher distributes the prebuilt game package.
- Expose broad, versioned engine APIs and explicit replaceable subsystem contracts, including advanced domains such as AI, physics and rendering.
- Keep scripts sandboxed by default, with explicit capabilities and execution budgets.

## Repository scope
This repository owns DreamMod Core integration source, interpreted runtime, mod-facing APIs, package format, compatibility tests and build documentation. The launcher is separate.

## Project memory
Read [AI_HANDOFF.md](docs/project-memory/AI_HANDOFF.md) first. The PMCS documents under `docs/project-memory/` are mandatory for every contributor and AI session.

## Architecture
See [DreamMod Architecture](docs/specs/DREAMMOD_ARCHITECTURE.md).

## Status
Architecture proposal only. No runtime, HNMod integration, loader, API or .dmod package has been implemented or verified.
