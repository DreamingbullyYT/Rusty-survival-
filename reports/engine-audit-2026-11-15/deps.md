# Dependency audit: rusty-survival- (read-only)

Sources: `tools/layers.json`, `docs/layers.md`, `cargo metadata --no-deps --offline` run on all four workspaces listed in `layers.json` (root, `launcher/`, `crates/horizon-telemetry-server/`, `crates/horizon-mod-registry/`), root `Cargo.lock`, `tools/check_layers.py`. Scripts are in `<scratchpad>/rs-audit/` (`graph.py`, `analyze.py`, `ext.py`, `dups.py`). Nothing under the repo was modified. No cargo build/check/test/clippy was run.

Counts: 55 workspace packages, all present in `layers.json` and vice versa (no missing, no extra). Layers: 19 engine (8 of them portable), 17 platform, 15 horizon, 4 tool (`horizon-bots`, `horizon-perf`, `horizon-mod-api-gen-tests`, `xtask`). No non-workspace path dependencies.

## 1. Engine-layer graph (19 crates)

Shipped = normal + build. "Opt" = optional, with the feature(s) that enable it. Transitive non-engine = crates outside the engine layer reached through shipped edges.

| crate | portable | shipped workspace deps (layer) | dev deps | fan-out | fan-in | transitive, non-engine |
|---|---|---|---|---|---|---|
| horizon-core | yes | none | none | 0 | 12 | none |
| horizon-hooks | no | none | none | 0 | 4 | none |
| horizon-narrator | no | none | none | 0 | 3 | none |
| horizon-perf-format | no | none | none | 0 | 5 | none |
| horizon-prepare | no | none | none | 0 | 1 | none |
| horizon-system | no | none | none | 0 | 4 | none |
| horizon-threads | yes | none | none | 0 | 5 | none |
| horizon-world | yes | horizon-core, horizon-threads | none | 2 | 9 | none |
| horizon-protocol | yes | horizon-core, horizon-world | none | 2 | 7 | none |
| horizon-items | yes | horizon-client-core, horizon-core, horizon-world | none | 3 | 3 | none |
| horizon-command | yes | horizon-client-core, horizon-core, horizon-protocol, horizon-world | none | 4 | 3 | none |
| horizon-assets | yes | horizon-core | none | 1 | 3 | none |
| horizon-client-core | yes | horizon-world | none | 1 | 6 | none |
| horizon-client-input | no | horizon-client-core | horizon-theme (platform) | 1 | 3 | none |
| horizon-server-files | no | horizon-core, horizon-protocol, horizon-world | none | 3 | 4 | none |
| horizon-bench | no | horizon-system (target-gated, non-wasm) | none | 1 | 4 | none |
| horizon-client-render | no | 18: assets, client-core, client-input, command, core, hooks, items, narrator, perf-format, protocol, system, threads, world (engine); hdr, optifine, polytone, shader-api, shaders (platform) | items, theme; horizon-perf (tool) | 18 | 2 | hdr, optifine, polytone, shader-api, shaders (5) |
| horizon-client-lib | no | 23: engine 15; platform 7 (anticheat, hdr, mcp, mods, optifine, shader-api, shaders); horizon 1 (telemetry) | client-render, items, theme; horizon-perf (tool) | 23 | 1 | 11 incl. addresses, anticheat, hdr, mcp, mods, optifine, platform, polytone, shader-api, shaders, telemetry |
| horizon-client | no | 30: engine 17; platform 10 (anticheat, chat-filter, dualsense opt `controllers`, hdr, mod-api-gen, mods, optifine, shader-api, shaders, theme); horizon 3 (cosmetics, telemetry, friends opt `friends`) | client-render, items; horizon-perf (tool) | 30 | 1 (horizon-game) | 17 incl. addresses, cosmetics, telemetry, friends, platform |

Notes on the engine table:
- Fan-in for horizon-client is 1 (horizon-game). horizon-client-lib has fan-in 1 (horizon-client).
- Engine crates depend on nothing above them except through the 26 `allowed` edges below.
- Engine-internal cycles: none.

Engine direct edges to the non-engine layers (all are in `allowed`):
- horizon-client -> anticheat, chat-filter, cosmetics, dualsense, friends, hdr, mod-api-gen, mods, optifine, shader-api, shaders, telemetry, theme
- horizon-client-lib -> anticheat, hdr, mcp, mods, optifine, shader-api, shaders, telemetry
- horizon-client-render -> hdr, optifine, polytone, shader-api, shaders

Total: 26 edges, matching `allowed` (13 + 8 + 5).

## 2. Rest of the workspace (36 crates)

| crate | layer | shipped workspace deps | dev deps | fan-in |
|---|---|---|---|---|
| horizon-platform | platform | bench, core, hooks, perf-format, prepare, protocol, server-files, system, world (engine); prepare/protocol/server-files/world are optional | none | 9 |
| horizon-mods | platform | shader-api (opt `index` in horizon-mods) | none | 7 |
| horizon-shader-api | platform | none | none | 5 |
| horizon-addresses | horizon | none | none | 5 |
| horizon-telemetry | horizon | addresses, platform | none | 5 |
| horizon-friends | horizon | addresses, platform (opt `game`) | none | 4 |
| horizon-mod-api-gen | platform | none | none | 4 |
| horizon-anticheat | platform | none | none | 3 |
| horizon-hdr | platform | none | none | 3 |
| horizon-mcp | platform | none | none | 3 |
| horizon-theme | platform | none | none | 3 |
| horizon-shaders | platform | core | none | 3 |
| horizon-optifine | platform | core | system (dev) | 3 |
| horizon-release-sign | horizon | none | none | 3 |
| horizon-chat-filter | platform | server-files | none | 2 |
| horizon-cosmetics | horizon | addresses | none | 2 |
| horizon-perf-share | horizon | addresses, platform | none | 2 |
| horizon-dualsense | platform | none | none | 1 |
| horizon-polytone | platform | none | none | 1 |
| horizon-port-pack | platform | mods | none | 1 |
| horizon-discord | horizon | platform | none | 1 |
| horizon-heartbeat | horizon | platform, telemetry | none | 1 |
| horizon-game | platform | client, mods, threads | none | 1 |
| horizon-official-servers | horizon | addresses, platform | none | 1 |
| horizon-server | horizon | anticheat, chat-filter, mcp, platform, release-sign | none | 0 |
| horizon-official | horizon | discord, friends, game, heartbeat, official-servers, perf-share, platform | none | 0 |
| horizon-launcher | horizon | cosmetics, friends, mcp, mod-api-gen, mods, platform, port-pack, release-sign, shader-api, telemetry, theme | none | 0 |
| horizon-relay | horizon | friends | none | 0 |
| horizon-mod-registry (own workspace) | horizon | mods, release-sign | none | 0 |
| horizon-telemetry-server (own workspace) | horizon | telemetry | telemetry | 0 |
| cargo-horizon-mod | platform | mods | none | 0 |
| horizon-mod-deps | platform | none | none | 0 |
| horizon-bots | tool | protocol | none | 0 |
| horizon-perf | tool | bench, perf-format, perf-share | none | 0 |
| horizon-mod-api-gen-tests | tool | mod-api-gen | none | 0 |
| xtask | tool | mod-api-gen, theme | none | 0 |

Tool crates are depended on by no shipped crate (check: none). Shipped code depends on tools only through dev-deps (`horizon-perf` and `horizon-bots` are not used by shipped code).

## 3. Cycles

- All edges, including dev-deps: no strongly connected component with more than one node, and no self-loops. No cycles, no dev-dependency cycles.

## 4. Violations

`python3 tools/check_layers.py` ran read-only and PASSED: `Layer check passed: 55 crates, 26 exceptions left to move (docs/layers.md).` Exit 0. It needed no network.

My independent recount from the manifests agrees:
- Unlisted shipped edges that break `may_depend`: none.
- Shipped edges that are in `allowed`: 26 of 26. Stale `allowed` entries: none.
- Portable crates: no edge to a non-portable workspace crate; no forbidden library (arboard, cpal, gilrs, ureq, wgpu, winit). Portable external deps are: libm, log, serde, serde_json, flate2, sha1/sha2, aes, getrandom, socket2, ipconfig, lz4_flex, postcard, xxhash-rust, mimalloc, libmimalloc-sys, libc, addr2line, framehop, memmap2, object, rustc-demangle, windows-sys.

Caveat: check_layers only reads manifests (`cargo metadata`). It does not scan source for names. The check ignores dev-deps by design. Two engine crates have dev-deps on higher layers: horizon-client -> horizon-perf (tool), horizon-client-lib -> horizon-perf (tool), horizon-client-lib -> horizon-theme (platform), horizon-client-input -> horizon-theme (platform), horizon-client-render -> horizon-theme (platform). These are permitted by the docs ("tests may use anything").

## 5. External (non-workspace) dependencies of the 19 engine crates

Full table is in the script output from `ext.py`. Shortened here: 64 distinct direct external dependencies. "Locked" is the version(s) in the root `Cargo.lock`. "(multi)" means the name appears at more than one version in the root lock.

| dependency | req (as written) | locked | engine crates that pull it |
|---|---|---|---|
| wgpu | ^30.0.1 | 30.0.1 | client, client-lib, client-render (plus cfg variants) |
| winit | ^0.30.13 | 0.30.13 | client, client-input, client-lib, client-render |
| naga | ^30.0.1 | 30.0.1 | client, client-lib, client-render (plus dev) |
| ureq | ^2 | 2.12.1 | client, client-lib, client-render, server-files |
| log | ^0.4 | 0.4.34 | 9 crates |
| serde_json | ^1.0.151 | 1.0.151 | 9 crates |
| serde | ^1 | 1.0.229 | 7 crates |
| flate2 | ^1.1.10 | 1.1.10 | client, client-lib, client-render, perf-format, protocol, world |
| png | ^0.18 | 0.18.1 | client, client-input, client-lib, client-render (dev: assets) |
| sha2 | ^0.10 | 0.10.9 | client, client-lib, client-render, prepare, protocol, world |
| sha1 | ^0.10 | 0.10.7 | client, client-lib, client-render, prepare, protocol |
| zip | ^9.1.3 | 9.1.3 | client, client-lib, client-render, prepare |
| chrono | ^0.4.42 | 0.4.45 | client, client-lib, client-render |
| bytemuck | ^1.24 | 1.25.2 | client, client-lib, client-render |
| glam | ^0.30 | 0.30.10 | client, client-lib, client-render |
| pollster | ^0.4 | 0.4.0 | client, client-lib, client-render |
| ab_glyph | ^0.2.32 | 0.2.32 | client, client-lib, client-render |
| unicode-bidi | ^0.3.18 | 0.3.18 | client, client-lib, client-render |
| miniz_oxide | ^0.8 | 0.8.9 (+0.9.1) (multi) | client, client-lib, client-render |
| crc32fast | ^1 | 1.5.2 | client, client-lib, prepare |
| fs4 | ^1.1.0 | 1.1.0 | client, client-lib |
| toml | ^1.1 | 1.1.6+spec-1.1.0 | client |
| gilrs | ^0.11 | 0.11.2 | client (opt controllers), client-lib (opt) |
| cpal | ^0.16 | 0.16.0 | client-render (opt) |
| arboard | ^3.6.1 | 3.7.0 | client-input |
| rayon | ^1.10 | 1.12.0 | client-lib |
| mio | ^1.2.4 | 1.2.4 | client-lib |
| xxhash-rust | ^0.8.19 | 0.8.19 | client-lib, world |
| open | ^5.4.5 | 5.4.5 | client-lib (cfg windows) |
| dunce | ^1.0.5 | 1.0.5 | client-lib (cfg windows) |
| base64 | ^0.22 | 0.22.1 (+0.23.1) (multi) | client-render, server-files |
| getrandom | ^0.3 | 0.3.4 (+0.2.17, 0.4.3) (multi) | client-lib, protocol, server-files |
| half | ^2 | 2.7.1 | client-render |
| image-webp | ^0.2 | 0.2.4 | client-render |
| lz4_flex | ^0.14 | 0.14.0 | client-render, world |
| smallvec | ^1 | 1.16.2 | client-render |
| symphonia | ^0.5 | 0.5.5 | client-render |
| zune-jpeg | ^0.4 | 0.4.21 | client-render |
| prost | ^0.14 | 0.14.4 | perf-format |
| prost-build / prost-types | ^0.14 | 0.14.4 | perf-format (build / dev) |
| protox | ^0.9 | 0.9.1 | perf-format (build / dev) |
| regex | ^1 | 1.13.1 | prepare |
| libc | ^0.2 | 0.2.190 | prepare (cfg), threads (cfg) |
| libm | =0.2.16 | 0.2.16 | core, world |
| postcard | =1.1.3 | 1.1.3 | world |
| aes | ^0.8.4 | 0.8.4 | protocol |
| socket2 | ^0.5 | 0.5.10 (+0.6.5) (multi) | protocol |
| ipconfig | ^0.3.4 | 0.3.4 | protocol (cfg windows) |
| libmimalloc-sys | ^0.1.49 | 0.1.49 | threads |
| mimalloc | ^0.1.52 | 0.1.52 | threads |
| addr2line | ^0.26 | 0.26.1 | threads (cfg linux) |
| framehop | ^0.16 | 0.16.0 | threads (cfg linux) |
| memmap2 | ^0.9 | 0.9.11 | threads (cfg linux) |
| object | ^0.39 | 0.39.1 | threads (cfg linux) |
| rustc-demangle | ^0.1 | 0.1.28 | threads (cfg linux) |
| windows-sys | ^0.61 | 0.61.2 (+0.45.0, 0.52.0, 0.59.0) (multi) | threads (cfg windows) |
| libloading | ^0.8.9 | 0.8.9 | narrator (cfg linux) |
| objc2 | ^0.6 / ^0.6.5 | 0.6.5 (+0.5.3) (multi) | client, client-lib, narrator (cfg macos) |
| objc2-app-kit | ^0.3 / ^0.3.2 | 0.3.2 (+0.2.2) (multi) | client, client-lib, narrator (cfg macos) |
| objc2-foundation | ^0.3 / ^0.3.2 | 0.3.2 (+0.2.2) (multi) | client, client-lib, narrator (cfg macos) |
| windows | ^0.62.2 | 0.62.2 (+0.54.0) (multi) | narrator (cfg windows) |
| winresource | ^0.1 | 0.1.34 | client (build) |
| rustix | ^1 | 1.1.5 (+0.38.44) (multi) | client, client-lib, client-render (cfg unix) |

Duplicated locked versions: 10 names matter for engine direct deps (the rest of the 50+ duplicate names in the root lock are transitive only). Who pulls the other version, from the root lock:
- base64 0.23.1: pem (transitive).
- getrandom 0.2.17: rand_core, ring. 0.4.3: jobserver, quinn-proto, rand, zip.
- miniz_oxide 0.9.1: flate2 (the engine's own `flate2 ^1.1.10` brings it in).
- objc2 0.5.3: block2, objc2-app-kit (old family), objc2-cloud-kit, etc. objc2-app-kit 0.2.2 and objc2-foundation 0.2.2: winit and objc2-link-presentation (old family).
- rustix 0.38.44: calloop, smithay-client-toolkit, winit.
- socket2 0.6.5: friends, ipconfig, mysql, quinn, redis, tokio.
- windows 0.54.0: cpal (the optional client-render dep).
- windows-sys 0.45.0: jni. 0.52.0: ring, socket2, winit. 0.59.0: rustix.

Other lockfiles (not the root workspace) also carry duplicates. For example, `launcher/Cargo.lock` has 60+ duplicated names, and `crates/horizon-mod-registry/Cargo.lock` has `miniz_oxide` 0.8.9 + 0.9.1 and `syn` 2.0.119 + 3.0.9. Those are separate workspaces and not part of the engine graph.

## 6. Discrepancies between docs/layers.md or layers.json and the manifests

1. **"Every Horizon feature sits behind `horizon-extras`" (docs/layers.md, Horizon section, "Rules")** is not true of the manifests. `crates/horizon-client/Cargo.toml` line 14 has `horizon-cosmetics = { path = "../horizon-cosmetics" }` with no `optional`, and line 75 has `horizon-telemetry = { path = "../horizon-telemetry", default-features = false }`, also not optional. `crates/horizon-client-lib/Cargo.toml` line 77 has `horizon-telemetry` unconditional. Only `horizon-friends` (line 45, optional, `friends`) is gated, and `horizon-extras = [..., "friends"]` reaches it. So a build with `--no-default-features` still links cosmetics and telemetry into the engine crate `horizon-client`. The claim that "horizon-plain builds without it" is not supported by the manifests as written.

2. **Engine must not depend on platform or Horizon (docs/layers.md, rule table) versus 26 `allowed` exceptions.** Consistent with the file's own "exceptions left to move" note, but the transitive effect is larger than the direct list: `horizon-client` reaches 17 non-engine crates, including `horizon-addresses` (the Horizon address list) and `horizon-telemetry`. This is not a contradiction of the docs, but it is worth knowing when the Horizon layer moves.

3. **Tools list (docs/layers.md, "Tools")** names `xtask`, `horizon-bots`, `horizon-mod-api-gen-tests`. `tools/layers.json` and the manifests also make `horizon-perf` a tool crate (workspace member, layer `tool`). The docs omit it.

4. **Names that appear in docs/layers.md but are not packages in the manifests:**
   - `horizon-ctl`: a `[[bin]]` target in `crates/horizon-client-lib/Cargo.toml` (line 145, path `../horizon-client/src/bin/horizon-ctl/main.rs`). Not a crate. Consistent as a binary name.
   - `horizon-extras`: a feature of `horizon-client` and `horizon-client-render`, not a crate. Consistent.
   - `horizon-mod-runtime`: appears in docs/layers.md, but no package with that name in any workspace manifest. Stale or planned name.
   - `horizon-engine`: docs/layers.md and `crates/horizon-client/Cargo.toml` mention it as a released server-build name, not a crate. Consistent.

5. **Portable crate list** in `layers.json` (8 crates) matches the list in docs/layers.md. All 8 are engine layer. The forbidden list (`arboard`, `cpal`, `gilrs`, `ureq`, `wgpu`, `winit`) is respected. No discrepancy found.

6. **Layer policy for `horizon-platform`**: docs say the platform never depends on Horizon. Manifests agree: `horizon-platform` ships only on engine crates and platform-free crates, and no platform crate depends on a horizon-layer crate. No discrepancy.

7. **Docs say the Horizon layer "uses only the platform"**. Manifests agree: all 15 horizon-layer crates depend only on platform or horizon crates. No engine dependency is found in the horizon layer. No discrepancy.

8. **`launcher/` and the two side workspaces** are declared in `layers.json` `workspaces`. All three manifests are read and all their packages are in `layers.json`. No discrepancy.
