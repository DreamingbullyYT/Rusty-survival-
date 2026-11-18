# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## DM-001 — Horizon baseline and integration pin
Status: IMPLEMENTED
Current Core baseline: Horizon Nightly 2026-11-18 `0b14f8acfd97fa2c750e42795249dcc981178ca4`, matching SDK `sdk-0b14f8acfd97fa2c750e42795249dcc981178ca4`, horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, mod API `0.2`, game `26.3`. Re-audit before every upstream compatibility update.

## DM-002 — .hmod build-time integration and prebuilt distribution
Status: INVESTIGATING
The current Core manifest is pinned to the published Nightly and uses the required `capabilities = ["registries"]` field from Horizon's official native template; it has no source mixins or internals. The official cargo-horizon-mod check workflow passed in run 44285877944 and uploaded a source .hmod artifact, which was downloaded and inspected. The stock launcher CLI requires prepared Minecraft 26.3 assets and refuses without an authenticated owning account; do not bypass this. The current CI-only harness calls the exact internal prebuilt::compile(..., Route::Api, ...) with signed SDK client runtime/API slices and validates the output through the real host loader. Run 44288437986 failed because the step overrode HOME and hid Rustup; fixed in 67bc587. Run 44288685182 then failed because cargo was invoked from the Horizon root workspace instead of the separate launcher workspace; fixed in 7a7cdb0. Current run 44289897914 is in progress. Remaining work: pass the overlay build and host-loader test, inspect the artifact, then combine it with a matching licensed base installation.

## DM-003 — Interpreted language selection
Status: INVESTIGATING
Added an isolated Rhai 1.27.0 AST-evaluator prototype with Grain bytecode disabled and AST optimization disabled. It is not the production runtime and not the final language decision. Starlark was rejected because its evaluator compiles AST to bytecode. Sprite is a research-only tiny tree-walker reference, too limited to adopt directly. Rhai tests/Clippy passed in 44284795812; a comparable 10k-item batch benchmark was 462.94× slower than native on x86_64 CI. New array/map/string/variable/function budgets and the array-growth test passed in 44285379104. Keep Rhai provisional. Do not add bytecode, JIT or player-side native compilation.

## DM-004 — Deep subsystem replacement boundaries
Status: OPEN
Horizon internals cannot be assumed replaceable merely because hooks exist. Identify extension points and define complete contracts for AI, physics, rendering and other systems.

## DM-005 — Script performance in hot paths
Status: OPEN
Interpreted callbacks are 462.94× slower than Rust in the current synthetic 10k-item Rhai benchmark on x86_64 CI. Do not promise arbitrary per-entity/per-vertex script loops for physics/rendering. Use coarse provider dispatch with Rust-owned batch iteration. A CI-prebuilt native kernel tier could be considered later, but is not implemented and would be separate from interpreted `.dmod` scripts.

## DM-006 — Script sandbox and capabilities
Status: OPEN
Need per-mod isolation, explicit capability grants, exact memory accounting, safe resource access, cancellation and fault containment. The Rhai prototype tests operation/call-depth limits, per-array/per-map/per-string caps, and variable/function counts. These are per-value/script limits, not aggregate per-mod heap accounting; production enforcement remains open.

## DM-007 — API stability and upstream compatibility
Status: OPEN
DreamMod must pin Horizon/mixins/hooks revisions and version its public API. Define compatibility ranges and migration rules.

## DM-008 — Package format and lifecycle
Status: OPEN
Manifest, permissions, dependency resolution, deterministic load order, enable/disable, reload and error reporting are draft requirements only.

## DM-009 — Historical engine prototype scope
Status: INVESTIGATING
Existing engine-core, testkit, Mojang artifact tooling, Minelib audits and Android Vulkan POC were built for the former independent-engine direction. Preserve them for history; do not treat them as DreamMod implementation or extend them without a new explicit scope decision.


## DM-010 — Reuse existing Horizon mod platform and prebuilt native loading
Status: INVESTIGATING
Horizon already provides a typed mod API, native host, content/native/WASM package routes, hooks, Parts, attachments, capabilities, API injection points and native mixins/bridges/loop slots. Do not duplicate these. The existing WASM runtime uses a child-process Wasmtime compilation path on the player's machine when the module cache is cold, so it does not meet strict interpreted-source/no-compilation requirements. Native .hmod source is normally compiled by the launcher, but the native host also loads adjacent prebuilt libraries with SHA-256 and build identity checks. A CI-built DreamMod Core native library is being built via the exact internal Route::Api compiler with signed SDK runtime/API slices. The overlay build and real host-loader proof are pending in Actions run 44288685182.
Evidence: docs/audits/HORIZON_MOD_PLATFORM_AUDIT.md (source route traced at Horizon `1c45cf...` and rechecked at current Nightly `0b14f8acfd97fa2c750e42795249dcc981178ca4`; horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`).
Required action: complete the current CI-only signed-SDK API-route harness, inspect the generated client overlay and pass the real native-host loader test. Then verify that the user's launcher can apply the overlay to a matching licensed base install. A full gameplay launch remains a later gate. The stock native-build CLI requires prepared Minecraft assets; do not bypass the ownership check.


## DM-011 — Compile and package the first DreamMod Core bootstrap against pinned Horizon
Status: VERIFIED
The official pinned cargo-horizon-mod check and source .hmod build passed in run 44285877944. The artifact was downloaded and inspected; it contains the expected manifest, Cargo source and uses.json, pinned to Horizon 0b14f8a and API 0.2. This verifies source/API package compilation, not native dylib packaging or game startup.


## DM-012 — Validate the Rhai interpreted-source candidate
Status: INVESTIGATING
Added `mods/dreammod-runtime-prototype` and a dedicated workflow. Rhai 1.27.0 AST evaluation uses `no_optimize` and excludes Grain bytecode. Formatting, behavior tests, Clippy and per-value resource-limit tests passed (runs 44284795812 and 44285379104). Provider batch validation was then added; run 44289702282 caught rustfmt-only differences, fixed in 99be124. Run 44289964423 then caught an ambiguous EvalAltResult-to-Box conversion in the new type-validation path; fixed with Box::new in 5bdd485. Follow-up workflow is pending. The comparable 10k-item benchmark was 462.94× slower than native on x86_64 CI. Starlark was rejected because it compiles AST to bytecode; Sprite remains research-only due to its tiny feature set. Keep Rhai provisional, design coarse provider APIs and benchmark ARM64 before any language decision.


## DM-013 — Validate script provider batch boundaries
Status: IMPLEMENTED
The Rhai evaluation prototype now rejects mismatched position/velocity lengths, batches above 100,000 items, non-finite inputs, wrong-sized provider output, non-numeric output and non-finite output. Focused regression tests were added in commit 33639ee0d89627b7ad3342d694a2d26c1ff43fd5. Actions result pending inspection. This is prototype boundary validation, not production sandboxing or aggregate memory accounting.


## DM-014 — Versioned .dmod manifest contract
Status: VERIFIED
A standalone `mods/dreammod-package` crate parses a strict version-1 JSON manifest, rejects unknown fields/schema versions, validates reverse-DNS-style mod IDs, semantic versions and API compatibility ranges, restricts entrypoints to relative `scripts/*.dream` paths, allowlists and de-duplicates capabilities, validates dependency IDs/ranges, and caps manifest bytes and collection counts. The dedicated package workflow passed in Actions run 44299317538 (commit 4e983c0). ZIP inspection and dependency planning are tracked separately; this manifest parser does not grant permissions or execute scripts.


## DM-015 — Safe .dmod archive ingestion
Status: VERIFIED
Added a metadata preflight validator and integrated it with the in-memory ZIP reader in `mods/dreammod-package`. It applies entry-count, per-file and aggregate expanded-size limits; compression-ratio limits; strict portable relative paths; case-insensitive duplicate detection; rejects symlinks, unsupported types and encrypted entries; limits file roots to `manifest.json`, `scripts/` and `assets/`; and requires one manifest plus the declared script entrypoint. The validator is called before decompression by `inspect_dmod()`. It does not extract files. Package CI passed in run 44299317538; actual decompressed output is bounded and CRC-checked. This is not a claim of complete protection against every ZIP parser vulnerability.


## DM-016 — Deterministic dependency planning
Status: VERIFIED
Added a bounded, side-effect-free dependency planner to `dreammod-package`. It validates each manifest, rejects duplicate installed IDs, missing dependencies and semver mismatches, detects cycles, and emits a stable dependency-first order with lexicographic tie-breaking. It does not install, execute or grant capabilities. Unit tests cover order, missing/incompatible dependencies, cycles and duplicate IDs. Package CI passed in run 44299317538.


## DM-017 — Real in-memory .dmod ZIP inspection
Status: VERIFIED
Added an exact-pinned ZIP reader dependency and inspect_dmod(). It bounds compressed input, validates archive header consistency before member access, accepts only Stored/Deflate, performs metadata preflight before decompression, reads and parses the manifest, validates the declared script entrypoint, then streams all regular members to EOF in fixed-size chunks for actual-size and CRC validation. No files are extracted. Tests cover real valid ZIPs, traversal, truncated/non-ZIP input and payload corruption. GitHub Actions run 44299317538 passed on commit 4e983c04ea8f3a4085aa2e13b6b53d691c307564: formatting, all 19 tests, and Clippy with `-D warnings`. The initial test failure was a malformed JSON fixture for a Windows path; after correcting its JSON escaping, tests passed. Clippy then identified a needless boolean branch in the path validator; it was simplified and the complete workflow passed.


## DM-018 — Legacy duplicate session IDs in append-only PMCS journal
Status: VERIFIED
The historical SESSION_LOG contains duplicate IDs from earlier entries. Rewriting those headings would violate the append-only rule. The validator now permits legacy duplicates but rejects newly appended IDs that duplicate another new entry or reuse any prior ID. Regression tests cover both cases; PMCS Actions run 44299837834 passed the new regressions, and final run 44300145443 passed the full structural/freshness validation.


## DM-019 — Retain validated .dmod contents in memory
Status: VERIFIED
inspect_dmod() now returns a ValidatedPackage containing parsed manifest metadata, archive inventory and validated PackageFile records for scripts/assets. Every file is streamed to EOF and CRC-checked before its bytes are retained. Aggregate and per-file uncompressed limits bound retained data; manifest.json is parsed separately and not duplicated. No filesystem extraction, installation, permission grant or execution occurs. Fixture assertions verify exact retained script/asset bytes. Actions run 44300015439 passed formatting, all 19 tests and Clippy on commit 2dd019b.


## DM-020 — Explicit capability grant planning
Status: VERIFIED
Added plan_capability_grants(manifest, approved) as a pure deterministic planner. Empty approvals grant nothing; requested but unapproved capabilities remain denied; unknown, unrequested and duplicate approvals are rejected. The returned CapabilityPlan is not runtime authority. Added CapabilityGate with private granted state and a require() check; it must still be wired into every Horizon host API boundary before enforcement can be claimed. No launcher permission UI, sandbox or live capability enforcement is implemented. Regression tests passed; package Actions run 44300324687 passed all 21 tests and Clippy on commit d83b3491b747bc880b657e4cf17e259a8af2ff62.


## DM-021 — Host capability gate primitive
Status: VERIFIED
Added CapabilityGate::from_approvals(), allows() and require(). It can only be constructed through a validated manifest and explicit approvals; its granted set is private, and unknown/unapproved capabilities are denied. This is not yet connected to Horizon's host API or the runtime, so no end-to-end enforcement claim is made. Actions run 44300879584 passed formatting, all 22 tests and Clippy on commit 5085971c93c3dbc2a0089e136e5f9a7aecc21a49.




## DM-022 — Owned player snapshot bridge contract
Status: VERIFIED
Added `bridge::PlayerSnapshot`, an owned DTO containing UUID, validated player name, dimension identifier and finite bounded position. The type contains no Horizon references or handles. `HostAdapter::read_player_snapshot()` checks the `entities.read` grant and enabled lifecycle state before running the supplied host-read closure, then rejects invalid DTOs. Regression tests cover DTO validation and prove denied calls do not invoke the read closure. This is not yet a concrete Horizon `ServerHost` adapter, runtime script binding, sandbox or end-to-end player API. Source commits: `3e8baf1`, `8845d96`, `7bae3ea`; package Actions run `44309091208` passed formatting, all tests and Clippy.


## DM-023 — Concrete Core-side online-player collector
Status: IMPLEMENTED — SDK verification pending
Added a Core-local owned snapshot DTO and bounded collector using the pinned `ServerHost::players().server_players()` service. The collector rejects batches above 256 before reading, preserves connection order, reads each player's name/dimension/entity position while the host borrow is valid, and copies only owned values. It rejects unavailable handles/fields, invalid names/dimensions/coordinates and duplicate UUIDs. No Horizon handle escapes. The collector is not yet called by the .dmod runtime and does not itself enforce per-mod capabilities; the runtime must call it only after an `entities.read` grant is checked. Source commits: `7ecd981`, `b2362c6`. Core SDK check pending in Actions run 44310547586.


## DM-024 — Bind Core player-read authorization to per-mod lifecycle
Status: OPEN
The Core collector now refuses to touch `ServerHost` until a `PlayerReadAuthorization` implementation grants `entities.read`. Regression tests verify callback non-invocation on denial and one invocation on success. This is only an interface seam: no implementation currently connects it to a particular package `ModLifecycle` instance. The future runtime must implement the trait using the mod's Enabled state and approved capability set, then ensure every script-triggered read passes through it. Source commit: `99929e3`. Core and overlay Actions are running; verify before claiming the change is compiled.
