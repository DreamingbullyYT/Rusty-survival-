# Project scope

This repository is the working copy for the Horizon engine refactor.

## In scope
- Vanilla engine architecture and implementation.
- Engine systems, internal APIs, ownership boundaries, dependency structure and tests.
- Removing confirmed duplicate implementations only after tracing their callers, data flow and tests.

## Out of scope for this phase
- Android or desktop launcher work.
- A new custom Modding API. That is designed only after the engine refactor is stable.
- New platform/product features.

The upstream platform, Horizon product and launcher sources may remain in the repository when needed to preserve a coherent upstream snapshot and build dependencies. Their presence does not authorize changes to them. The active refactor target is the vanilla engine layer described in docs/layers.md.

At the start of each session, read project-memory/START_HERE.md and the memory files it lists.
