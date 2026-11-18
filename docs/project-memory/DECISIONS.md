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


## Decision D-2026-11-18 — bounded batch validation and load deadline
- Use a contiguous UUID scratch vector plus sort_unstable for duplicate detection in bounded (maximum 256) player snapshot batches; preserve original snapshot/connection order.
- Use one monotonic 60-second deadline for a .dmod load transaction. Archive inspection polls it between entries and decompression chunks. A future loader must propagate the same deadline into script parsing, dependency preparation and registration; timeout must fail closed.
- Do not claim hard preemption: synchronous stages must cooperatively poll the deadline. Cancellation/isolation for script execution is a separate runtime requirement.
- Benchmark before claiming a measurable FPS gain.


## Decision D-2026-11-18-02 — fixed stack scratch for player batches
Use a fixed [u128; 256] stack scratch array for duplicate UUID detection in Core and package snapshot batches. Sort only the populated prefix; preserve the original player order. This is bounded by the existing 256-player cap and removes one temporary heap allocation. No performance gain is claimed until benchmarked.


## Decision D-2026-11-18-03 — share snapshot batches immutably
Use Arc<[PlayerSnapshot]> as PlayerSnapshotBatch backing storage. Cloning a batch is a cheap reference-count increment rather than a deep clone. Snapshot contents remain immutable. A future dispatcher may share a batch only after checking the receiving mod's lifecycle/capability; no cross-mod authorization bypass is allowed.

## DECISION — isolate shared authorization contracts from package inspection (2026-11-18)
- Decision: use a minimal `dreammod-contract` crate for the player-read authorization trait and capability identifier. Both Core and package/lifecycle depend on it.
- Rationale: Core must not inherit ZIP, JSON, semver or archive-inspection dependencies merely to check per-mod authorization; a shared trait avoids duplicate contracts and supports a future runtime owner.
- Consequence: this is an integration seam, not proof that a runtime manager currently connects the two crates. The package lifecycle remains separate from Horizon host handles. Revisit only if the public capability model is redesigned.

## DECISION — centralize package lifecycle ownership in ModManager (2026-11-18)
- Decision: introduce one deterministic `ModManager` owning validated package contents, dependency edges and one `ModLifecycle` per package.
- Enable is dependency-first; disabling a dependency with an enabled dependent is rejected; failure cascades transitively and fail-closed.
- Rationale: keep lifecycle decisions centralized and deterministic before adding an interpreter or host registrations.
- Consequence: manager does not execute scripts or interact with Horizon. A future runtime layer must implement transactional registration and cleanup without bypassing per-mod lifecycle authorization.

## DECISION — use a callback rather than a local Cargo contract crate for Core authorization (2026-11-18)
- Decision: Core player collection accepts a one-shot authorization closure; package `ModManager::require_capability` supplies the per-mod lifecycle/grant check.
- Evidence: overlay run `44362776804` failed because Horizon's prebuilt source staging did not include the local `dreammod-contract` Cargo dependency.
- Rationale: the HNMod source package must compile using only dependencies supported by Horizon's signed SDK and its native package format. A closure keeps the boundary typed without adding an unsupported external crate.
- Consequence: the callback is a seam, not a live integration. A future runtime coordinator must call the Core collector from a valid host context and supply the manager callback.
