# Architecture and Project Decisions

## D-001 — Hybrid Minelib strategy
Date: 2026-11-17
Status: ACCEPTED
Decision: Evaluate Minelib as a source of selectively reusable components, not as a mandatory fork base.
Reason: Preserve useful engineering while avoiding inherited architectural constraints and unsupported Android assumptions.

## D-002 — Android first
Date: 2026-11-17
Status: ACCEPTED
Decision: Android is the first-class target.
Reason: User explicitly selected this platform.

## D-003 — Minecraft Java observable parity
Date: 2026-11-17
Status: ACCEPTED
Decision: Minecraft Java Edition observable behavior is the primary correctness criterion. Exact release/version remains pending confirmation.
Reason: User selected parity as the top priority.

## D-004 — Repository reset
Date: 2026-11-17
Status: ACCEPTED
Decision: Replace the default-branch tree with a clean project bootstrap while retaining Git history.
Reason: User explicitly requested that previous repository contents be removed and the project restarted.

## D-005 — Horizon as inspiration, not code base
Date: 2026-11-17
Status: ACCEPTED
Decision: Use Horizon only as inspiration for architecture, modularity, separation of concerns, and engineering ideas. Do not copy Horizon source code, assets, or implementation.
Reason: User explicitly clarified that we want the ideas, not a copy.

## D-006 — Independent engine implementation
Date: 2026-11-17
Status: ACCEPTED
Decision: Design and implement our own engine, using audited Minelib components only where justified and independently developed systems elsewhere.
Reason: Preserve control over Android-first architecture and Minecraft Java behavior without inheriting another project's implementation.
