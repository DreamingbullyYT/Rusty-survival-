
- New `dreammod-api` registration foundation is in the workspace. Registry is metadata/bookkeeping only; it does not yet dispatch events, execute systems, call engine services, or load interpreted scripts. Tests/Clippy pending.


- Runtime lifecycle mismatch was confirmed: runtime loading auto-enabled packages despite explicit lifecycle policy. Fixed to default-disabled with explicit activation, but activation is not yet unified with package approval and no live engine adapter exists. Latest CI failure is formatting/PMCS, so compile/test status is unknown.
