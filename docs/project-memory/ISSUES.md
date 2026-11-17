# Issues and Risks

## I-001 — Minelib Android readiness
Status: INVESTIGATING
Evidence: Upstream README lists Windows/Linux as available and Android/iOS as in preparation. The inspected release workflow builds Windows executables only. Platform crate uses winit and desktop-oriented keyboard/mouse handling.
Required action: Prove Android target support, lifecycle, surface creation, touch input, native dependencies, and build reproducibility.

## I-002 — Component license compatibility
Status: OPEN
Evidence: Repository LICENSE is AGPLv3; GitHub API metadata reports NOASSERTION. The p2p package manifest separately declares MIT OR Apache-2.0.
Required action: Review scope/provenance of each crate and contribution, notices, source-distribution/network-use obligations, and redistribution implications before reuse. Do not assume package metadata overrides root LICENSE.

## I-003 — Minecraft asset rights and provenance
Status: OPEN
Required action: Determine permitted asset acquisition, use, modification, and redistribution. Do not bundle Mojang assets without a documented lawful basis.

## I-004 — Minecraft Java 26.3 parity specification
Status: OPEN
Evidence: Minecraft Java Edition 26.3 is the selected behavior target. The parity matrix is currently a high-level checklist; version-specific mechanics and reference fixtures have not yet been specified.
Required action: Build a version-pinned specification for world height/sections, block states/registries, generation, lighting, fluids, collision, interaction, and observable test cases.

## I-005 — Minelib world semantics do not establish Java Edition parity
Status: INVESTIGATING
Evidence: Source inspection shows custom 16×512×16 chunks, Minelib-specific block IDs/registry, custom value-noise terrain, RGB torch lighting, and chemical torch gameplay. Fluid geometry exists but Java-compatible fluid tick/spread behavior is not demonstrated.
Required action: Independently specify and implement Minecraft Java 26.3 semantics. Treat mesh and DDA as algorithmic candidates only.


## I-006 — Android renderer lifecycle/surface proof required
Status: INVESTIGATING
Evidence: wgpu 25 lists Vulkan on Linux/Android and winit 0.30 documents Android support through android-activity glue. Minelib's pinned workspace does not enable the Android glue feature or provide an Android entrypoint, and no Android build/device test was found.
Required action: Build an isolated Android winit + wgpu 25 Vulkan clear-screen APK. Verify Resumed/Suspended, pause/resume, surface recreation, rotation, synchronization and real-device presentation before selecting the production backend.

## I-007 — Minelib embedded asset provenance
Status: BLOCKED
Evidence: minelib-assets embeds VoxelArt_Libre.zip using include_bytes! and installs it into the user's resource-pack directory. The inspected source does not establish provenance or redistribution rights.
Required action: Obtain and record author, license, permission and redistribution terms before any reuse or bundling.

## I-008 — Minelib networking is not Java protocol
Status: OPEN
Evidence: minelib-net uses a custom Postcard/Noise-over-TCP protocol with Minelib account, relay, room and gameplay message types. It is not Minecraft Java Edition 26.3 protocol 777.
Required action: If multiplayer is in initial scope, create a separate protocol 777 specification and black-box interoperability fixtures. Do not substitute the Minelib relay protocol.


## I-009 — Android Vulkan POC build and device validation
Status: INVESTIGATING
Evidence: An isolated ARM64 NativeActivity/winit/wgpu Vulkan clear-screen POC and CI packaging workflow have been added. The first CI build and real-device tests are pending.
Required action: Pass ARM64 native compilation and APK packaging; install on a real device; verify Vulkan adapter selection, first present, pause/resume, rotation and repeated surface recreation. Do not treat a CI APK build as proof of device runtime support.
