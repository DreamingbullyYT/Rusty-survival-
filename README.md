# HNMod — Native Survival & Environment Ecosystem

HNMod is an independent, modular Rust project intended to extend Horizon with a cohesive world, climate, weather and survival simulation.

The project is kept separate from the Horizon source repository. Engine integration will be isolated behind a dedicated adapter so the core systems can be developed and tested without patching Horizon internals.

## Project layout

- `hnmod.project.toml` — HNMod development workspace descriptor. This is not the final Horizon .hnmod bundle manifest.
- `src/` — initial engine-independent core library.
- `tests/` — core behavior tests.
- `.github/workflows/ci.yml` — formatting, test and Clippy checks.

## Planned modules

1. **HNMod Core** — module metadata, dependency validation, deterministic load ordering and compatibility policy.
2. **HN Earth** — geographic projection, elevation, land/ocean masks and terrain data.
3. **HN Climate** — climate zones, temperature, seasons and vegetation response.
4. **HN Weather** — clouds, precipitation, storms and scalable atmospheric effects.
5. **HN Survival** — thirst, body temperature and environmental player effects.

## Development status

The first Core scaffold is in place. Horizon and horizon-mixins revisions are intentionally marked UNPINNED until the current upstream APIs and the native-mod template are verified together. Do not treat this scaffold as a loadable .hnmod mod yet.

## Build and test

Requires a current stable Rust toolchain.

```sh
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Design principles

- Keep the engine-independent simulation separate from Horizon-specific hooks.
- Avoid broad or invasive Horizon patches.
- Pin exact upstream revisions before writing the native adapter.
- Make module dependencies explicit and deterministic.
- Keep data acquisition and expensive work off world-generation worker threads.
- Design for desktop first, with Android/ARM64 compatibility as a later validation target.

## License

MIT for the original HNMod code in this repository. Third-party datasets and upstream integration components may have separate licenses and attribution requirements.
