# Compatibility and Verification Matrix

The former Minecraft Java Edition 26.3 parity matrix is historical and is not the active DreamMod test target.

## Active DreamMod verification domains

| Domain | Status | Required evidence |
|---|---|---|
| Pinned Horizon integration | NOT_STARTED | Reproducible HNMod build against exact Horizon/mixins/hooks revisions |
| Prebuilt distribution | NOT_STARTED | Build artifact contains DreamMod Core and boots successfully |
| No player-side compilation | NOT_STARTED | Install/run trace shows no Rust/native/bytecode compilation for .dmod |
| Package validation | NOT_STARTED | Valid/invalid manifest and archive tests |
| Interpreted runtime | NOT_STARTED | Source .dmod executes without compilation |
| Sandbox/capabilities | NOT_STARTED | Denied capability and resource-budget tests |
| API lifecycle/events | NOT_STARTED | Deterministic integration tests |
| System registration | NOT_STARTED | Ordering, dependency and failure tests |
| Subsystem replacement | NOT_STARTED | Provider contract, fallback, state and performance tests |
| Upstream compatibility | NOT_STARTED | Test matrix across explicitly pinned Horizon revisions |

No domain may be marked VERIFIED without reproducible tests and recorded revision metadata.
