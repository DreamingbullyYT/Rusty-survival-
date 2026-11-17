# Issues and Risks

## I-001 — Minelib Android readiness
Status: INVESTIGATING
Evidence: Upstream README lists Windows/Linux as available and Android/iOS as in preparation. The inspected release workflow builds Windows executables only. Platform crate uses winit and desktop-oriented keyboard/mouse handling.
Required action: Prove Android target support, lifecycle, surface creation, touch input, native dependencies, and build reproducibility.

## I-002 — Component license compatibility
Status: OPEN
Evidence: Repository LICENSE is AGPLv3; GitHub API metadata reports NOASSERTION. The p2p package manifest separately declares MIT OR Apache-2.0.
Required action: Review the scope and provenance of each crate and contribution, notices, source-distribution/network-use obligations, and redistribution implications before reuse. Do not assume package metadata overrides root LICENSE.

## I-003 — Minecraft asset rights and provenance
Status: OPEN
Required action: Determine permitted asset acquisition, use, modification, and redistribution. Do not bundle Mojang assets without a documented lawful basis.

## I-004 — Minecraft Java parity specification
Status: OPEN
Required action: Pin the exact Java Edition target version and create behavior-specific test cases. Version not yet selected for this project.

## I-005 — Minelib world semantics may not match Java Edition
Status: INVESTIGATING
Evidence: Minelib world crate exposes a custom 16×512×16 chunk model and procedural generation. This does not establish Java Edition chunk, block, lighting, fluid, collision, or world-generation parity.
Required action: Audit source behavior and compare against the pinned Java target before reuse.
