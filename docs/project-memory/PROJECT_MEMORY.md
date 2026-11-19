
- `dreammod-api` registration crate now provides owner-scoped extension declarations (command/event/system/content/provider/resource), optional required feature, atomic validation, deterministic priority/resource ordering, lookup and unregister-owner cleanup. It is engine-neutral metadata only; runtime dispatch and engine integration remain future work. CI pending.


# Mission

Build an engine-native, extensible mod platform for Rusty-survival with interpreted `.dmod` scripts, broad subsystem APIs, deterministic lifecycle and capability controls.

# Ground rules

- Avoid player-device compilation.
- Keep API engine-neutral and connect through validated adapters.
- Use owned snapshots, opaque handles, batching and capability enforcement.
- Never claim a test or live integration passed without CI evidence.

# Continuity system

This PMCS is the persistent source of project state. Read the mandatory handoff and append new session records; do not rewrite prior log entries.

## Runtime lifecycle correction

The player runtime now defaults newly loaded mods to disabled; explicit activation is required. This is not yet connected to the package ModManager approval UI or a live engine adapter. CI currently reported formatting and PMCS validator failures, and runtime tests were skipped.
