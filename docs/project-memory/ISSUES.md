# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## DM-001 — Horizon baseline and integration pin
Status: OPEN
Need to inspect current Horizon main/release, horizon-mixins revision and hooks/API version. Pin exact revisions before writing the HNMod integration.

## DM-002 — HNMod build-time integration contract
Status: INVESTIGATING
Current documentation describes native HNMod packages as Rust source built against a pinned Horizon revision. Need verify the exact minimal integration path and ensure the player receives a prebuilt distribution, not HNMod source to compile locally.

## DM-003 — Interpreted language selection
Status: OPEN
Rhai is a candidate only. Compare true source interpretation, Android/ARM64, performance, safe API exposure, sandbox, diagnostics and maintenance. Do not accidentally introduce bytecode compilation.

## DM-004 — Deep subsystem replacement boundaries
Status: OPEN
Horizon internals cannot be assumed replaceable merely because hooks exist. Identify extension points and define complete contracts for AI, physics, rendering and other systems.

## DM-005 — Script performance in hot paths
Status: OPEN
Interpreted callbacks may be too slow at per-entity/per-vertex frequency. Use batching/coarse-grained dispatch and benchmark representative workloads before promising full replacement.

## DM-006 — Script sandbox and capabilities
Status: OPEN
Need per-mod isolation, explicit capability grants, memory/execution budgets, safe resource access, cancellation and fault containment.

## DM-007 — API stability and upstream compatibility
Status: OPEN
DreamMod must pin Horizon/mixins/hooks revisions and version its public API. Define compatibility ranges and migration rules.

## DM-008 — Package format and lifecycle
Status: OPEN
Manifest, permissions, dependency resolution, deterministic load order, enable/disable, reload and error reporting are draft requirements only.

## DM-009 — Historical engine prototype scope
Status: INVESTIGATING
Existing engine-core, testkit, Mojang artifact tooling, Minelib audits and Android Vulkan POC were built for the former independent-engine direction. Preserve them for history; do not treat them as DreamMod implementation or extend them without a new explicit scope decision.
