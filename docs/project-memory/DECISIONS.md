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
Reason: It is the user's explicit platform choice.

## D-003 — Parity first
Date: 2026-11-17
Status: ACCEPTED
Decision: Minecraft Java observable behavior is the primary correctness criterion.
Reason: User selected parity as the top priority.

## D-004 — Repository reset
Date: 2026-11-17
Status: ACCEPTED
Decision: Replace the current default-branch tree with a clean project bootstrap while retaining Git history.
Reason: User explicitly requested that all previous repository contents be removed and the project restarted.
