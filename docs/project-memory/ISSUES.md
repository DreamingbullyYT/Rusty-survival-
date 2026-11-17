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
