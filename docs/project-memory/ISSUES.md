# Issues and Risks

## I-001 — Minelib Android readiness
Status: OPEN
Observation: Upstream Minelib README lists Android/iOS as in preparation; current release workflow inspected during initial audit targets Windows.
Required action: Verify current upstream state, Android target support, native dependencies, GPU surface integration, and build reproducibility before reuse.

## I-002 — Component license compatibility
Status: OPEN
Observation: Upstream Minelib repository declares AGPL-3.0; assets have separate attribution/licensing.
Required action: Review legal/redistribution implications before copying, linking, modifying, or distributing any component. Record exact source paths and license notices.

## I-003 — Minecraft asset rights and provenance
Status: OPEN
Required action: Determine permitted asset acquisition, use, modification, and redistribution. Do not bundle Mojang assets without a documented lawful basis.

## I-004 — Minecraft Java parity specification
Status: OPEN
Required action: Pin the exact Java Edition target version and create behavior-specific test cases. Version not yet selected for this project.
