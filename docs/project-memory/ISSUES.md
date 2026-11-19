
- New `dreammod-api` registration foundation is in the workspace. Registry is metadata/bookkeeping only; it does not yet dispatch events, execute systems, call engine services, or load interpreted scripts. Tests/Clippy pending.


- Runtime lifecycle mismatch was confirmed: runtime loading auto-enabled packages despite explicit lifecycle policy. Fixed to default-disabled with explicit activation, but activation is not yet unified with package approval and no live engine adapter exists. Latest CI failure is formatting/PMCS, so compile/test status is unknown.


- Contract unit-test compilation error (`Handle<T>` inference) was fixed with an explicit unit marker type. Root Rust core workflow passed format, tests and Clippy at `1fb4ed6b6272804027aecb14f40f3ae8a3621b45`. PMCS freshness must be refreshed for this source test edit.


- Runtime fault-disable previously left provider registrations stored. Call paths still gated by enabled state, but stale provider state could be revived by explicit re-enable. Fix now clears owner providers when threshold trips. CI pending.
