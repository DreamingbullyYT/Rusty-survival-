# Architecture and Project Decisions

## D-001 — Hybrid Minelib strategy (SUPERSEDED)
Date: 2026-11-17
Status: SUPERSEDED
Historical decision to evaluate Minelib as a source of reusable engine components. It does not govern active DreamMod work.

## D-002 — Android first (SUPERSEDED)
Date: 2026-11-17
Status: SUPERSEDED
Historical target for the independent engine prototype. DreamMod follows the supported Horizon runtime/build targets.

## D-003 — Minecraft Java observable parity (SUPERSEDED)
Date: 2026-11-17
Status: SUPERSEDED
Historical correctness target for the independent engine prototype; not the active DreamMod mission.

## D-004 — Repository reset (HISTORICAL)
Date: 2026-11-17
Status: HISTORICAL
The repository was reset to a clean project bootstrap while retaining Git history.

## D-005 — Horizon integration
Date: 2026-11-18
Status: ACCEPTED
Decision: Use Horizon as the target engine and integrate DreamMod Core through a pinned HNMod/horizon-mixins build-time integration. Do not copy Horizon source into this repository.
Reason: User explicitly selected Horizon as the engine base and approved the prebuilt-distribution architecture.

## D-006 — No compilation for player-installed mods
Date: 2026-11-18
Status: ACCEPTED
Decision: Ordinary .dmod packages contain interpreted source and are loaded at runtime. No Rust/native compilation, bytecode compilation or game rebuild is required on player devices.
Reason: User requires mods to work without compilation on the player's machine.

## D-007 — Prebuilt Horizon distribution
Date: 2026-11-18
Status: ACCEPTED
Decision: Integrate DreamMod Core once during trusted build time and distribute a prebuilt Horizon package containing it. The separate launcher handles installation and updates.
Reason: This avoids player-side HNMod compilation.

## D-008 — Deep, contract-based extensibility
Date: 2026-11-18
Status: ACCEPTED
Decision: DreamMod must support adding systems and replacing designated subsystems through explicit, versioned contracts, with AI, physics and rendering among the intended advanced domains.
Reason: User selected maximum extensibility while preserving runtime safety and performance.

## D-009 — Script language remains open
Date: 2026-11-18
Status: ACCEPTED
Decision: Do not lock Rhai or another runtime until no-compilation semantics, API ergonomics, Android support, sandboxing and representative benchmarks are evaluated.
Reason: The choice affects the entire runtime and should be evidence-based.
