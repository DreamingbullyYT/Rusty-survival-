# Issues and Risks

Status vocabulary: OPEN, INVESTIGATING, IMPLEMENTED, VERIFIED, BLOCKED.

## I-001 — Exact Minecraft Java target specification
Status: INVESTIGATING
The target version is Minecraft Java Edition 26.3. Detailed version-pinned mechanics and test fixtures are not complete.

## I-002 — Minelib license and provenance boundaries
Status: OPEN
The root project is AGPLv3; individual crates may have distinct declared licenses. Any code reuse requires file-level provenance, license compatibility review and attribution/obligations review. No Minelib source has been copied into this repository.

## I-003 — Android platform support in Minelib
Status: INVESTIGATING
Upstream README describes Android/iOS support as in preparation. Desktop-only platform assumptions must not enter the Android engine boundary.

## I-004 — Java world/chunk/block-state incompatibility
Status: OPEN
Minelib's current chunk height, block IDs, registry and generator are not Java-compatible drop-ins. Only isolated algorithms may be adapted after tests.

## I-005 — Java-compatible world generation, lighting and fluids
Status: OPEN
Custom Minelib algorithms do not establish Java parity. Implement against version-pinned reference behavior and fixtures.

## I-006 — Renderer integration on Android
Status: INVESTIGATING
Minelib renderer is an adaptation candidate, not a verified Android renderer. Vulkan surface and lifecycle support need hardware proof.

## I-007 — Asset provenance and redistribution
Status: OPEN
Do not bundle Mojang assets until lawful acquisition, redistribution basis and provenance are documented.

## I-008 — Networking protocol compatibility and security
Status: OPEN
Minelib networking requires protocol, authentication, encryption and security review before reuse.

## I-009 — Android Vulkan POC device validation
Status: INVESTIGATING
Evidence: CI run 44127492869 completed SUCCESS. ARM64 native library compilation and Gradle debug APK packaging both passed. Artifact ID 14347124661 (2,379,313 bytes; SHA-256 d4ec89aca87ae27dbd22f7505965be1017b5f522790323363581cb8ed4fafd61).
Required action: Install on a real device; verify Vulkan adapter selection, first present, pause/resume, rotation and repeated surface recreation. CI compilation/package success is not proof of runtime support.


## I-010 — Engine core test coverage and parity evidence
Status: INVESTIGATING
A minimal horizontal coordinate primitive and tests have been added. These test coordinate invariants only. No version-pinned Minecraft Java 26.3 differential fixtures or gameplay tests exist yet.
Evidence: Engine Tests run 44140955601 passed formatting, all workspace targets, and Clippy on PR head f504ba1f3629702330380c3bb71c44c4a5879150. PMCS run 44140955639 also passed.
Required action: Add version-pinned Java 26.3 reference fixtures and expand coverage in dependency order. Mark parity VERIFIED only after comparison with captured reference behavior.


## I-011 — Reference-artifact acquisition and non-publication
Status: OPEN
The Java 26.3 fixture runner and downloader are not implemented. Future CI must resolve official Mojang metadata, verify size/hash, use $RUNNER_TEMP, and delete inputs on every exit path. It must not cache or upload Mojang jars/assets. The public workflow must be transparent; hiding the download mechanism is not a compliance strategy.

## I-012 — Engine crate boundaries and staged implementation
Status: INVESTIGATING
A readable target layout is documented, but only engine-core exists. Do not create placeholder crates or implement version-specific world semantics before fixture evidence exists.


## I-013 — Fixture validation versus reference execution
Status: INVESTIGATING
The version-pinned JSON fixture contract and synthetic validator tests are implemented in `crates/testkit`. No Java 26.3 oracle capture, artifact downloader, output comparator or differential execution exists yet. Synthetic fixtures must never be counted as vanilla parity.


## I-014 — Reference artifact workflow verification
Status: INVESTIGATING
The official metadata resolver, server artifact downloader, offline Python tests and manual Actions workflow have been implemented. The workflow is intentionally artifact-only and does not claim to run a Java oracle or prove game parity. Verify a normal run and a deliberate-failure run; inspect that the unconditional cleanup step succeeds and the temporary directory is absent. No Mojang binary is uploaded, cached or committed.
