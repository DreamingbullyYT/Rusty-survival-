# DreamMod

A deeply extensible, interpreted runtime modding platform for Horizon.

- Player-installed `.dmod` packages execute as interpreted source.
- No per-mod Rust/native compilation, bytecode build, or game rebuild on player devices.
- DreamMod Core is built once as a trusted native Horizon mod and distributed prebuilt.
- Reuse Horizon's typed mod API and native host; extend only where required.
- Add systems and replace designated subsystems through versioned contracts.
- Sandbox scripts by default with explicit capabilities and execution budgets.

This repository owns DreamMod Core, runtime, mod-facing APIs, package format, tests and documentation. The launcher is separate.

Read [AI_HANDOFF.md](docs/project-memory/AI_HANDOFF.md), [architecture](docs/specs/DREAMMOD_ARCHITECTURE.md) and [Horizon platform audit](docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md).

**Status:** minimal native .hmod Core bootstrap exists; pinned-source CI compile is pending. No interpreter, .dmod loader, prebuilt native distribution or full DreamMod API is verified.
