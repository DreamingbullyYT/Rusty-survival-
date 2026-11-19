# Current Issues

### Atlas declaration integration (2026-11-19)
- Package-level declaration ownership and snapshot integration has been implemented in ModManager and awaits CI validation.
- Core's pinned atlas collect hook now re-submits its snapshot on resource reload. The .dmod runtime still does not populate the Core registry, and resource-pack/sprite validation remains Horizon's responsibility.
- API should remain direct/batched. Atlas JSON parsing, sprite validation and resource publication belong to the host hook/runtime integration.
