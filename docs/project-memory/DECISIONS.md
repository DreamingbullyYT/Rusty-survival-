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

## D-005 — Horizon integration (SUPERSEDED)
Date: 2026-11-18
Status: SUPERSEDED by D-014
Historical decision: Use Horizon as the target engine and integrate DreamMod Core through pinned HNMod/horizon-mixins build-time integration. This no longer governs the new independent Rusty-survival engine.

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

## DECISION — expose package assets as borrowed in-memory bytes (2026-11-18)
- Decision: add a capability-gated `ModManager::read_asset` that returns `&[u8]` from the retained validated package.
- Rationale: `.dmod` assets should be usable without extraction, duplicate allocation or filesystem path exposure.
- Security: require Enabled + `assets.read`, restrict to `assets/`, reuse archive path validation, and return immutable borrowed bytes.
- Consequence: this is raw package-byte access only; resource decoding, caching, texture registration and per-script budgets remain future host/runtime responsibilities.

## DECISION — gate a mod's own entrypoint by activation, not a synthetic capability (2026-11-18)
- Decision: provide `ModLifecycle::ensure_enabled` and gate `ModManager::entrypoint_source` on Enabled state.
- Rationale: a mod's own entrypoint is the code being activated; it should not require an artificial user-grantable `scripts.execute` permission that is absent from the capability allowlist.
- Privileged host calls remain independently gated by explicit manifest capabilities. Parsing, sandboxing and execution budgets remain future work.


## D-010 — Atlas registration uses one manager-owned batch
Date: 2026-11-19
Status: ACCEPTED
Decision: Store atlas declarations by mod in ModManager and expose a deterministic snapshot for enabled owners. Replacement is atomic and checked against the combined active-atlas cap; disable/failure removes owner declarations. Core should consume this batch directly rather than introducing redundant public wrapper layers.
Reason: Atlas declarations are small, bounded metadata and resource reload needs a complete deterministic collection. This avoids per-declaration bridge calls while preserving ownership and rollback.
Consequence: This remains package bookkeeping until the pinned Horizon hook is verified and connected; no live registration claim.

## D-011 — Core publishes atlases from one bounded snapshot
Date: 2026-11-19
Status: ACCEPTED
Decision: The Core `register_texture_atlases` collect hook consumes one sorted registry snapshot and resubmits it on initial registration/resource reload. Script adapters must authorize before batch replacement.
Reason: Horizon requires atlas declarations on every reload; batch snapshot avoids per-resource bridge calls and keeps validation/ownership at Core boundary.
Consequence: This is not end-to-end .dmod support until the interpreter and ModManager call the registry; no native handles or unrestricted access are exposed.


## D-012 — Runtime formats are not assumed to be dual-mode
Date: 2026-11-19
Status: ACCEPTED
Decision: The current player-package path remains intended interpreted-source .dmod. Rhai is only an isolated prototype and not selected. DreamMod does not implement WASM or player-native module execution. DreamMod Core is trusted native Horizon .hmod build-time integration. Do not promise dual WASM/native player-mod support until separate runtimes and security boundaries exist and are verified.
Reason: Source inspection found no DreamMod WASM runtime or player-native loader. Horizon's existing WASM cold-cache compilation path conflicts with the no-player-compilation rule.
Consequence: WASM needs a compliant precompiled module/runtime design and Host API adapter; player-native mods remain out of scope pending explicit trust-model approval.


## D-013 — DreamMod packages share Horizon's mods directory (LEGACY HORIZON TARGET)
Date: 2026-11-19
Status: ACCEPTED
Decision: DreamMod player packages (`.dmod`) are placed directly in Horizon's existing `horizon-mods/` directory. Do not create a separate `dreammod-mods/` directory. Keep the extensions and loaders distinct: Horizon owns `.hmod`; DreamMod owns `.dmod`. DreamMod's scanner must recognize only `.dmod` packages and must not alter, rename, consume, or intercept `.hmod` files. DreamMod Core remains trusted native Horizon `.hmod` build-time integration and is not a player `.dmod`.
Rationale: One familiar mods folder is simpler for players, while extension-based dispatch preserves clear format identity and compatibility with Horizon's loader.
Consequences: A library scanner is now implemented in dreammod-runtime for direct-child `.dmod` discovery with bounded package/archive/retained-memory budgets, duplicate-ID rejection, and per-file diagnostics. It ignores `.hmod` for Horizon, does not recurse, extract archives or intentionally follow symlinks. Launcher/game path and UI integration remain pending; CI verification is pending.


## D-014 — DreamMod is independent of Horizon/HNMods
Date: 2026-11-19
Status: ACCEPTED
Decision: The new Rusty-survival engine will host DreamMod directly through engine-owned adapter contracts. DreamMod must not require Horizon, HNMods/.hmod, horizon-mixins, or an external mod host. The .dmod package and no-player-compilation requirement remain. Horizon integration is optional legacy/reference work and must not block the new engine.
Reason: The new engine's architecture does not want HNMods; the mod API must be native to the engine rather than layered on a rejected mod host.
Consequences: Design an engine-independent contract crate and a separate engine adapter. API promises are capability-negotiated and implemented only when the engine subsystem exists. Avoid a full engine patch/mixin system. Build/test a vertical slice with the new engine. Existing Horizon Core and shared-folder scanner are retained as legacy experiments, not the integration target.


## D-2026-11-19-05 — Authorized command batches cross one engine host boundary
- Decision: Runtime forwards validated, capability-authorized commands to the engine through an EngineCommandHost batch method; the host contract requires atomic validation/apply. Empty batches do not call the host.
- Rationale: keep scripts isolated from engine objects and reduce host crossings; a host cannot observe commands before authorization.
- Consequence: current contract and mock tests are not a concrete engine adapter. Domain-specific validation, rollback and authoritative state checks remain mandatory in engine implementations.
