# Horizon upstream baseline

- Repository: https://github.com/horizon-rust/horizon
- Branch: main
- Imported commit: bcb7258ad101638c57e8cdd7b1e3da406440bea8
- Commit message: Merge pull request #16554 — prevent stuck damage immunity after respawning.
- Import policy: upstream source snapshot is preserved as the starting point; our changes are limited to the vanilla engine scope documented in project-memory.
- The upstream launcher and Horizon/platform crates remain present as reference/build dependencies where required, but are not targets for our refactor.
- Existing upstream architecture checks such as tools/check_layers.py and tools/layers.json are reused; do not recreate them.
