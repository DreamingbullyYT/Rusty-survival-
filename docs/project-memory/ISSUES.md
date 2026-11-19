# Current Issues

### Atlas declaration integration (2026-11-19)
- Package-level declaration ownership and snapshot integration has been implemented in ModManager and awaits CI validation.
- No Core/Horizon integration yet: the Core hook is empty and the Horizon reload registration contract still needs exact source/API verification.
- API should remain direct/batched. Atlas JSON parsing, sprite validation and resource publication belong to the host hook/runtime integration.
