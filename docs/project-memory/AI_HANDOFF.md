# AI Handoff — Mandatory Entry Point

Last updated: 2026-11-18
Project: DreamMod
Repository: DreamingbullyYT/Rusty-survival-
Target engine: Horizon (integrate through pinned upstream extension points; do not copy Horizon source)
Product: deeply extensible interpreted runtime modding API

## Mandatory onboarding
1. Read this file, then PROJECT_MEMORY.md, CURRENT_STATE.md, ARCHITECTURE.md, DECISIONS.md, ISSUES.md, ROADMAP.md, COMPATIBILITY.md, PARITY_MATRIX.md, REUSE_MATRIX.md, CHANGELOG.md, SESSION_LOG.md and CHANGE_TEMPLATE.md.
2. Read docs/specs/DREAMMOD_ARCHITECTURE.md before implementation.
3. Inspect current branch, HEAD commit, working tree and recent history. Repository state is authoritative over stale prose.
4. Work directly on `main`; do not create feature branches or pull requests unless the user explicitly changes this rule.
5. Read the latest session-log entry and unresolved issues before proposing work.
6. Treat source, Git history, reproducible tests, pinned upstream references and build logs as ground truth. Never invent progress.
7. Horizon is the target engine. Integrate DreamMod through the current .hmod/native mod route and horizon-mixins where required and pinned supported interfaces; do not copy Horizon source into this repository.
8. Current Horizon docs describe .hmod native packages, typed API injection points and native-only mixins/bridges/loop slots. The launcher normally compiles native mod source; the native host also loads prebuilt adjacent libraries with strict identity/hash checks. Verify the prebuilt distribution route.
   - Historical direct Core API check 44249522292 passed against the earlier source pin; it was cargo check only, not a native build or game launch.
   - Current Core package manifest is pinned to Horizon Nightly 2026-11-18 (`0b14f8acfd97fa2c750e42795249dcc981178ca4`) and horizon-mixins `e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, with `api = 0.2`, the required `capabilities = ["registries"]` (required by Horizon's official native template/manifest validator), and no native mixin/internals declarations. Treat `registries` as a real permission in the trust model. The pinned cargo-horizon-mod check and source .hmod package workflow passed in run 44285877944. The generated artifact was downloaded and inspected: it contains horizon-mod.toml, Cargo.toml, src/lib.rs and uses.json, with the expected Nightly pin. The prebuilt workflow now installs the launcher and exact Nightly successfully. Client `native-build --instance` then requires `prepare`, which refuses without an authenticated account owning Minecraft 26.3. Do not bypass this. The server export also required prepared assets. The current workflow uses a test-only module appended to the pinned Horizon launcher checkout to call its exact internal `prebuilt::compile(..., Route::Api, ...)` with the signed SDK client runtime/API slices, then a second test calls the real `horizon-mod-host::native_libraries::load`. This creates a client overlay without Mojang assets or account credentials; it does not bypass the ownership check. Run 44288437986 is pending.
   - An isolated Rhai 1.27.0 AST-evaluator candidate exists in mods/dreammod-runtime-prototype; Grain bytecode and AST optimization are disabled. It is not the final language choice or part of DreamMod Core. CI run 44283982705 passed formatting, 4 behavior/budget tests and Clippy. The first corrected benchmark exposed a large interpreter/native gap; the newest run adds equal input cloning and observable output checks, and its result is pending.
   - Starlark is rejected for the strict no-bytecode requirement: its evaluator compiles the parsed AST to bytecode. Sprite is a research-only tree-walking interpreter reference; its current feature set is too small to be adopted directly.
   - Pinned Horizon source at the current Nightly also confirms Route::Api for API-only native mods: mod dylibs are built against the prebuilt runtime/API slices and the game executable is returned unchanged. Current DreamMod Core qualifies because it uses Hooks only and declares no native source patch/internals dependency. A CI-built prebuilt game distribution and clean startup are still required.
9. The launcher distributes a prebuilt Horizon build with DreamMod Core already integrated. Launcher implementation is out of scope here.
10. Ordinary .dmod packages must run as interpreted source at runtime. No per-mod Rust/native compilation, bytecode compilation, or game rebuild on the player's device.
11. Do not select a scripting language until the evaluation and representative benchmarks justify it. Rhai and Starlark are candidates; the Rhai prototype is evaluation-only.
12. Design broad, versioned extension contracts. Deep access does not mean unsafe raw pointers or unrestricted native process access for scripts.
13. Scripts are sandboxed by default; capabilities, execution budgets, memory limits and fault isolation are mandatory.
14. Before implementation, state intended change, affected modules, risks and verification plan.
15. After every meaningful change, update project memory in the same logical change set. Append to SESSION_LOG; never rewrite old entries.
16. Run PMCS validation and relevant tests. For Rust changes run cargo fmt, cargo test --workspace --all-targets and cargo clippy --workspace --all-targets -- -D warnings in CI.
17. Distinguish IMPLEMENTED from VERIFIED. IMPLEMENTED is not VERIFIED.

## Non-negotiable product direction
- DreamMod is an advanced modding runtime/API for Horizon, not a standalone Minecraft engine.
- DreamMod Core is integrated once into a pinned Horizon build via the current .hmod/native mod route and horizon-mixins where required at build time.
- Launcher ships the prebuilt game distribution containing DreamMod Core.
- .dmod scripts are interpreted at runtime; player devices do not compile mods.
- Support adding systems and replacing designated subsystems through explicit contracts.
- Performance matters: keep hot loops native, use coarse-grained/batched script calls, benchmark replacement implementations.
- Script sandbox and explicit capabilities are mandatory.
- DreamMod Core's prebuilt native library is trusted build-time code, not an untrusted script sandbox.
- No launcher implementation in this repository.

## Historical work
The previous independent Minelib/vanilla-engine and Minecraft Java 26.3 parity direction is superseded by the user's 2026-11-18 DreamMod pivot. Existing engine/testkit/Android POC work remains historical and must not be presented as DreamMod functionality. Do not delete historical files or claim they are part of DreamMod without an explicit migration decision.

## Change record requirements
Every substantial change records date, commit, files, motivation, implementation, exact tests/results, risks and follow-up. Use docs/project-memory/CHANGE_TEMPLATE.md.

## Status vocabulary
OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## Handoff rule
At the end of a work session, update CURRENT_STATE.md, ISSUES.md, ROADMAP.md, CHANGELOG.md and this handoff when relevant. Append a dated SESSION_LOG.md entry with exact work and verification. Clearly state what was and was not tested.
