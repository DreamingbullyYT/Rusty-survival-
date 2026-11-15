# Engine-layer inventory (read-only audit)

Repo: `<repo>` (snapshot as of 2026-11-15). Nothing under the repo was modified; no cargo commands were run.

Scripts (outside the repo): `<scratchpad>/rs-audit/count.py`, `per_crate.py`, `client2.py`, plus JSON outputs `per_crate.json`, `client2.json`, `top10.json`, `all_imports.json`.

## Method and caveats

- Engine crates are the 19 with layer `engine` in `tools/layers.json` (`crates` key). `horizon-client-lib` has no directory of its own: its manifest sets `lib.path = ../horizon-client/src/client_lib.rs`, so its sources, its `horizon-ctl` bin, its 16 examples (incl. `examples/probe/`, 121 files) and its 9 tests are physically in `crates/horizon-client/`. Those lines are attributed to `horizon-client-lib` by manifest, and `horizon-client` gets the rest.
- "src non-test" = lines before the first `#[cfg(test)] mod ...` in a file. Lines from that point to EOF count as test ("inline test"). Whole files named `tests.rs`, `*_tests.rs`, or under a `tests/` dir, and files declared `#[cfg(test)] mod x;`, count as test ("test files"). Test-line figures are therefore ESTIMATES where a `#[cfg(test)]` block is not at the end of a file.
- Module-tree reachability was resolved by parsing `mod x;` and `#[path]`. 5 orphan test-named files (821 lines) in `horizon-client/src` are not reached by any `mod` declaration.
- Dependency and importer checks were grep-based. Layer rules were checked by reading manifests; `tools/check_layers.py` was NOT run (it needs `cargo metadata`). Dev-dependencies are not checked by that tool, and I treated them the same way.
- Importer counts are grep-based on `use horizon_<crate>::` and `horizon_<crate>::`, excluding comment lines and matches inside string literals on the same line. Engine modules are also reachable through re-exports (`horizon_client::command`, `horizon_client::items`, ...), which I counted separately.

## Summary table (all engine crates)

Columns: total = all `.rs` lines attributed to the crate. src = `src/` non-test. test = inline test plus test files under `src/`. ex = `examples/`. tests = `tests/` (or manifest-declared tests). benches/other are noted when nonzero.

| Crate | Total | src non-test | src test (est.) | examples | tests | other | Largest module (non-test lines) |
|---|---:|---:|---:|---:|---:|---:|---|
| horizon-assets | 8,438 | 6,253 | 1,308 | 877 | 0 | 0 | `item_model` 2,310 |
| horizon-bench | 1,123 | 779 | 344 | 0 | 0 | 0 | crate root 379 |
| horizon-client (lib.rs + main.rs owned; game/ tree) | 413,806 | 278,689 | 133,868 | 117 | 1,093 | 39 | `game/` 276,958 |
| horizon-client-core | 9,802 | 7,450 | 2,330 | 22 | 0 | 0 | `scoreboard` 1,413 |
| horizon-client-input | 10,789 | 6,676 | 4,113 | 0 | 0 | 0 | `gamepad` 1,906 |
| horizon-client-lib (sources in horizon-client/src) | 379,718 | 213,377 | 137,850 | 24,484 | 4,007 | 0 | `mobs/` 78,670 |
| horizon-client-render | 274,857 | 155,364 | 115,702 | 2,820 | 971 | 0 | `gfx` 36,042 |
| horizon-command | 12,590 | 10,593 | 1,997 | 0 | 0 | 0 | `args` 1,867 |
| horizon-core | 25,041 | 17,256 | 7,148 | 372 | 265 | 0 | `density` 4,625 |
| horizon-hooks | 1,817 | 1,402 | 415 | 0 | 0 | 0 | `players` 456 |
| horizon-items | 12,593 | 8,238 | 4,289 | 66 | 0 | 0 | `items` 2,863 |
| horizon-narrator | 1,475 | 1,101 | 346 | 28 | 0 | 0 | `speech` 564 |
| horizon-perf-format | 3,586 | 1,586 | 128 | 0 | 1,866 | 6 | `health` 404 |
| horizon-prepare | 8,423 | 5,857 | 2,362 | 0 | 166 | 38 | crate root 1,712 |
| horizon-protocol | 52,483 | 33,489 | 18,147 | 762 | 85 | 0 | `adapter` 15,972 |
| horizon-server-files | 3,340 | 2,613 | 678 | 49 | 0 | 0 | `properties` 642 |
| horizon-system | 1,617 | 1,228 | 389 | 0 | 0 | 0 | crate root 943 |
| horizon-threads | 7,483 | 4,406 | 3,021 | 56 | 0 | 0 | `sampler` 962 |
| horizon-world | 201,670 | 131,447 | 24,137 | 41,092 | 4,926 | 68 | `worldgen` 56,804 |
| **Engine total** | **1,430,651** | **887,804** | **458,572** | **70,745** | **13,379** | **151** | |

Checks: each row sums to its total (verified by script). `horizon-client` on disk (`crates/horizon-client`) is 793,524 Rust lines; of that, 379,718 is attributed to `horizon-client-lib` and 413,806 to `horizon-client`. The 820-line difference from the table total is the same orphan-file accounting noted above. Benches: none in any engine crate.

Notes on the table: `horizon-client-render` examples and tests are small, but `gfx/` alone has 52,123 test-ish lines (inline and test files, est.). `horizon-world` examples are 188 files, mostly `examples/probe/*_fixture.rs`.

---

## horizon-assets (engine)

- Total 8,438; src non-test 6,253; test ~1,308; examples 877 (`examples/`).
- Top modules: `item_model` 2,310 (item model JSON to baked models); `bake` 729 (model baking); `model` 565 (model structs); `blockstate` 561 (blockstate JSON); `math` 508 (transform math); `resolve` 486 (model reference resolution); `item_bake` 310 (item baking); `animation` 283 (model animations).
- Stated ownership (CLAUDE.md): resource decoding, block-state and item models, baking, atlases. Contents match. No Horizon extras found.
- Public surface: imported by 2 crates outside itself (per-module importer list in `all_imports.json`).

## horizon-bench (engine)

- Total 1,123; src non-test 779; test ~344; no examples/tests.
- Top modules: crate root `lib.rs` 379 (report struct and format); `compare` 211 (comparing runs); `hardware` 189 (machine section). Only 3 src files.
- Stated ownership: "the benchmark report format" (CLAUDE.md lists it in the telemetry row, `horizon-telemetry`, `horizon-telemetry-server`, `horizon-bench`). Its own doc says the telemetry upload sends it. **Mismatch:** CLAUDE.md groups it with the Horizon-layer telemetry crates, but `tools/layers.json` says `engine`.
- Imported by 3 crates outside itself (importer names in `all_imports.json`).

## horizon-client (engine by layers.json; the gameplay crate)

- On disk: 793,524 Rust lines (client-lib plus client). Attributed here: 413,806 total; src non-test 278,689; test ~133,868; examples 117 (`native_gui_overlay_probe.rs`, `wind_burst_hook_probe.rs`); tests 1,093 (`tests/it/`); other 39 (`build.rs`).
- Top-level modules (lib.rs-owned): `game/` 276,958 non-test (746 files); `mod_api.rs` 1,284 (mod API surface); `entry.rs` 362 (startup and entry point); `horizon_look.rs` 58.
- The `game/` tree by filename family (non-test lines; family = prefix before first `_`, or dir):
  - `native_*` (82 files): 33,217. Native-mod hook glue. Doc layers.md names `game/native_*.rs` as "still in engine crates today" that belong to the Platform layer. Includes `native_hooks.rs` at 12,510 lines.
  - `remote*` (18 files): 16,908. Vanilla multiplayer client. Engine.
  - `lan*` (20 files): 13,591. Vanilla LAN host glue. Engine.
  - `commands/` dir (25 files): 12,937. Vanilla command runners. Engine.
  - `creatures.rs`: 7,538. Mob ticking. Engine.
  - `dragon.rs`: 4,775. Engine.
  - `commands.rs`: 4,380. Engine.
  - `block*` (8 files): 3,629. Engine.
  - Platform-or-Horizon names in game/ (non-test, est.): `pad*` (controllers) about 5,700 across 15 files; `split/` (splitscreen) 3,262 across 8 files; `replay.rs` 856 and `studio.rs` 720 (replay and in-game video, Platform-layer by docs); `cosmetics.rs` + `cosmetics/` 1,840 across 5 files (Horizon, docs list it as "still in engine"); `extras.rs` + `extras/` 1,644 across 5 files (Horizon, docs list it); `mod_api_host.rs` 267 (docs list it); `friends.rs` + `friends/` 620 across 2 files (Horizon, allowed as `horizon-client -> horizon-friends`). `game/emotes.rs` is named in docs but does not exist.
- Stated ownership (CLAUDE.md): gameplay library plus thin executable, and the dedicated server. Contents match for vanilla rules. Horizon extras and platform features are in the same crate, as listed above, and docs/layers.md says they are still there today.
- Re-export use: `horizon_client::{mod_api, entry, game, native_data}` is used by 4 crates outside it (`cargo-horizon-mod`, `horizon-game`, `horizon-mod-api-gen`, `launcher`). Some `horizon_client::game` hits from `horizon-threads` and `horizon-perf` are string literals.

## horizon-client-core (engine)

- Total 9,802; src non-test 7,450; test ~2,330; examples 22.
- Top modules: `scoreboard` 1,413; `block_entity_nbt` 849; `nbt_path` 832; `text` 780; `lang` 684; `component` 513; `snbt` 398; `registries` 359.
- Stated ownership (CLAUDE.md): text, lang, snbt, nbt_path, component, scoreboard, bossbar, gamerules, block_entity_nbt, registry tables. Matches. Portable (layers.json `portable`). Normal deps: `horizon-world`, `serde_json` only. No forbidden libs.
- Imported by 5 crates (`horizon-client`, `horizon-client-render`, `horizon-command`, `horizon-items`, ...).

## horizon-client-input (engine)

- Total 10,789; src non-test 6,676; test ~4,113.
- Top modules: `gamepad` 1,906; `radial` 1,138; `window` 726; `keybinds` 532; `gameplay_guide` 521; `mouse` 366; `clipboard` 314; `key_names` 265.
- Stated ownership (CLAUDE.md): key bindings, toggles, mouse, controller settings and glyphs, clipboard, window options. **Mismatch:** `gamepad.rs` (controller settings) is Platform per layers.md Controllers section; `radial.rs` and `radial_file` (radial menu) are not named by any doc and are unclear.
- Dev-dep on `horizon-theme` (platform); dev-deps are not checked by the layer check. No normal-dep violation.

## horizon-client-lib (engine; sources physically in `crates/horizon-client/src/`)

- Attributed total 379,718: src non-test 213,377; test ~137,850 (inline 113,808 + test files 23,302 + bin 740); examples 24,484 (15 declared files 2,857 + `examples/probe/` 21,627); tests 4,007; bin `horizon-ctl` 3,026 (of which 2,286 non-test).
- Top modules (client_lib.rs-reachable, non-test lines): `mobs/` 78,670 (158 files); `lan/` 28,048 (53); `realms/` 14,982 (5); `menu/` 5,572 (5); `entity_nbt/` 4,696 (12); `motion/` 4,088 (4); `studio/` 3,529 (9); `title/` 3,344 (10).
- Stated ownership (CLAUDE.md): "Reusable client systems, `horizon-ctl`, probes and their tests ... It never depends on gameplay." **Mismatch:** `mobs/` (mob AI, spawn, brains), `arrows.rs`, `projectiles.rs`, `crafting.rs`, `raid.rs`, `animate_tick.rs`, `item_sync.rs` are gameplay, and `layers.json` plus docs/layers.md put gameplay in the engine. So the CLAUDE.md sentence is stale against the code.
- **Horizon/platform items in this crate:** `studio/` (its header says "Horizon In-Game Video Studio", 3,529 lines); `mod_content.rs` 3,271 (content-mod loader; check against docs/mods.md); `server_perf/` 1,169 (perf capture glue, `.hzperf` writer side).
- Manifest deps: allowed exceptions (`horizon-mcp`, `horizon-optifine`, `horizon-hdr`, `horizon-anticheat`, `horizon-shaders`, `horizon-shader-api`, `horizon-mods`, `horizon-telemetry`) are all in `layers.json` `allowed`. Dev-deps `horizon-theme` and `horizon-perf` are not checked.

## horizon-client-render (engine)

- Total 274,857; src non-test 155,364; test ~115,702; examples 2,820; tests 971.
- Top modules (non-test): `gfx/` 36,042 (renderer; test ~52,123 inline and files); `particles` 7,571; `lod` 5,347; `options_ui` 5,266; `hud` 4,506; `block_entity_render` 3,653; `streaming` 3,574; `settings_shell` 2,850.
- Stated ownership (CLAUDE.md): world data, rendering and screens, "Nothing in it reaches gameplay." Renderer and screens match.
- **Horizon extras inside (per layers.md, or by name, non-test lines):** `pvp.rs` 1,514 (docs list it as Horizon); `pvp_hud.rs` 530; `bluemap.rs` 1,536 + `bluemap_detect.rs` 257 (Horizon far-terrain from a third-party web map); `minimap.rs` 402; `social.rs` 345; `mod_browser.rs` 134; `mod_list.rs` 160; `extras.rs` 862 + `extras_ui.rs` 279 (docs list both); `studio_ui.rs` 479 (Horizon studio UI); `pack_browser.rs` 508 (built-in `https://api.modrinth.com/v2`, listed as an exception in `tools/addresses.json`). Total about 6,400 lines of Horizon or product code.
- **Platform inside (per layers.md):** `shader_ui.rs` 671 and `hdr_calibration.rs` 371 (docs list both as still in engine); `controller_ui.rs` 585, `pad_glyph.rs`, `pad_cursor.rs` (controllers); `split_ui.rs` 1,005, `split_layout.rs`, `split_map.rs`, `split_profiles.rs` (splitscreen); `android_gamepad.rs` 267, `android_runtime/` 158, `touch.rs` + `touch/` 1,386+626, `keypad.rs` 314 (Android and touch; the header says "Adapted from Starling", a third-party project).
- Imported by 1 crate outside itself (`horizon-client`, which includes `horizon-client-lib`).

## horizon-command (engine)

- Total 12,590; src non-test 10,593; test ~1,997.
- Top modules: `args` 1,867; crate root 1,255; `snbt` 983; `selector` 816; `vanilla` 682 (vanilla tree, `set_horizon_commands` toggles Horizon commands on the tree); `item` 673; `packrat` 643; `codec` 608.
- Stated ownership (CLAUDE.md): Brigadier parsing, vanilla tree, selectors, suggestions, network form. Matches. Portable. Its `vanilla.rs` has a Horizon-commands switch (`horizon_commands`, `set_horizon_commands`), a small Horizon footprint.
- Importers: 0 by `horizon_command::` import; it is reached via `horizon_client::command` (re-export).

## horizon-core (engine)

- Total 25,041; src non-test 17,256; test ~7,148; examples 372; tests 265.
- Top modules: `density` 4,625; `geometry` 2,916; `bin/` 2,719 (src/bin); `noise` 1,537; `random` 1,080 (Java `Random`); `jmath` 1,069; `paletted` 867; `ticks` 552.
- Stated ownership (CLAUDE.md): simulation primitives, jmath, positions, noise, density, paletted storage, `version.rs`. Matches.
- Most-imported engine crate: 11 importing crates; `random` (6), `prepared_files` (6), `version` (6), `position`, `jmath` (5 each).

## horizon-hooks (engine)

- Total 1,817; src non-test 1,402; test ~415.
- Top modules: `players` 456; crate root `lib.rs` 376 (the `Hooks` trait, `register`); `pages` 243; `settings` 137; `hud` 57; `tunnel` 50; `registry` 41; `lang` 31.
- Stated ownership: extension points, names no type from above. Matches by design. `tunnel.rs` mentions Play with Friends invite codes (a Horizon crate) in docs, not in code. `perf.rs` is a hook for saved-capture open and share.
- Imported by 3 crates (`horizon-client`, `horizon-client-core`, `horizon-platform`...).

## horizon-items (engine)

- Total 12,593; src non-test 8,238; test ~4,289; examples 66.
- Top modules: `items` 2,863; `item_data` 1,554; `enchant` 1,536; `blocks` 912; `bundle` 278; `recipe_filters` 274; `random_sequences` 196; `item_nbt` 135.
- Stated ownership (CLAUDE.md): blocks and items below gameplay. Matches.
- Imported by 2 crates outside itself.

## horizon-narrator (engine)

- Total 1,475; src non-test 1,101; test ~346; examples 28.
- Top modules: `speech` 564 (text-to-speech); crate root 273; `screen` 264 (screen narration).
- Stated ownership: vanilla narrator. Matches. No Horizon extras found.

## horizon-perf-format (engine)

- Total 3,586; src non-test 1,586; test ~128; tests 1,866; other 6 (`build.rs`).
- Top modules: `health` 404; `writer` 279; `merge` 259; `counters` 199; `files` 133; `slow` 107; `brief` 96; `timing` 41.
- Stated ownership (CLAUDE.md, docs/perf-platform.md): the `.hzperf` capture contract and its writer. Matches. Its `tests/` (1,866 lines) are a large share.

## horizon-prepare (engine)

- Total 8,423; src non-test 5,857; test ~2,362; tests 166; other 38 (`build.rs`).
- Top modules: crate root `lib.rs` 1,712; `artifacts` 1,459; `pyjson` 491; `java` 445; `sounds` 398; `main.rs` 334; `exporters` 181; `inventory` 177.
- Stated ownership: builds `.local/horizon-client/26.3` from the player's own install. Matches. Its build.rs and output-versions.txt are not Rust lines.
- No importers (binary crate).

## horizon-protocol (engine)

- Total 52,483; src non-test 33,489; test ~18,147; examples 762; tests 85.
- Top modules: `adapter` 15,972 (version adapters, `translate/`); `item` 6,431; `packets` 3,895; `rsa` 1,009; `connection` 731; `buf` 705; `secure_chat` 677; `entity_data` 591.
- Stated ownership: protocol 777 codecs, framing, encryption, packets. Matches.
- **Horizon items:** `mod_list.rs` (the Horizon mod list compared on join, docs/mods.md) and `menu_extra.rs` (native-mod menu type data, docs/native-mods.md). Small (about 160 and 60 lines non-test) but they are Horizon-layer protocol additions in the vanilla protocol crate. Note the mismatch in the CLAUDE.md row 90 ("network protocol 777").
- Imported by 6 crates. Note the top-10 list.

## horizon-server-files (engine)

- Total 3,340; src non-test 2,613; test ~678; examples 49.
- Top modules: `properties` 642; `user_lists` 504; `json` 411; `java_properties` 385; `user_cache` 231; `profiles` 148; `chat_guard` 137 (chat-filter glue, see layers.md engine list); `duration` 59.
- Stated ownership (CLAUDE.md): `server.properties`, eula, ops, whitelist, bans, usercache. Matches.

## horizon-system (engine)

- Total 1,617; src non-test 1,228; test ~389.
- Top modules: crate root 943; `scrub` 285 (strips user name, paths, IPs and UUIDs from text before it is kept or sent, for telemetry and crash reports).
- Stated ownership: machine facts for the crash report, GPU info and telemetry. Matches. `scrub.rs` is telemetry-privacy glue that feeds Horizon-layer telemetry; flag as a minor Horizon-adjacent item.
- Imported by 4 crates.

## horizon-threads (engine)

- Total 7,483; src non-test 4,406; test ~3,021; examples 56.
- Top modules: `sampler` 962; `profile` 951; `heap_sample` 524; `jobs` 505; `heap_census` 261; `boot_paint` 234; `work` 174; `alloc_count` 173.
- Stated ownership (CLAUDE.md): "thread priorities". **Mismatch:** sampling profiler (`sampler`, `profile`, `heap_sample`, `heap_census`, `alloc_count`) is about 2,871 non-test lines of profiling and heap-census tooling for `horizon-ctl memory` and the perf platform. Only `work`, `jobs` and `system_menu` fit the stated ownership. `boot_paint` is the window boot screen.
- Imported by 4 crates.

## horizon-world (engine)

- Total 201,670; src non-test 131,447; test ~24,137; examples 41,092 (188 files, mostly `examples/probe/*_fixture.rs`); tests 4,926; other 68 (`build.rs`).
- Top modules: `worldgen` 56,804; `level` 21,926; `datafix` 17,955; `loot` 2,991; `block_survival` 1,246; `attributes` 1,198; `nbt` 1,037; `bin/` 915 (worldgen parity bin).
- Stated ownership (CLAUDE.md): "Vanilla world data with no gameplay loop: NBT, regions, chunks, datafix/, worldgen/, level/ ... entity physics, items, effects, food, damage." **Mismatch with wording:** the crate holds gameplay rules: `living_*` (living_damage, living_fall, living_health...), `player_*` (player_health, player_food, player_inventory, player_travel...), `server_player_combat.rs`, `server_player_damage.rs`, `item_use*`, `piston_*`, `block_survival.rs`. These are rules, not a loop, and layers.md says the engine holds gameplay, so this is a doc-wording mismatch rather than a layer breach.
- Most imported engine crate after horizon-core: 8 importing crates (`nbt` 7; `datafix` 5; `snbt_text` 4; `compression` 4).

---

## Ownership flags (cross-crate summary)

1. Horizon extras in engine crates, per docs/layers.md "Still in engine crates today" and by name: `horizon-client` `game/cosmetics.rs`, `game/extras.rs`, `game/extras/`, `game/friends*`, `game/mod_api_host.rs`, `game/native_*.rs` (82 files, 33,217 non-test lines); `horizon-client-render` `pvp.rs`, `pvp_hud.rs`, `bluemap*.rs`, `minimap.rs`, `social.rs`, `mod_browser.rs`, `extras*.rs`, `studio_ui.rs`, `pack_browser.rs`; `horizon-client-lib` `studio/`.
2. Platform features in engine crates (controllers, splitscreen, replays, Android/touch): `horizon-client` `game/pad*`, `game/split/`, `game/replay.rs`; `horizon-client-render` `split_*`, `controller_ui.rs`, `pad_*`, `touch*`, `android_*`, `keypad.rs`; `horizon-client-input` `gamepad.rs`.
3. Built-in address in an engine crate: `pack_browser.rs` contacts `https://api.modrinth.com/v2` (also `tools/addresses.json` `allowed`). docs/layers.md says engine code may hold only vanilla's endpoints.
4. Doc contradictions: docs/layers.md lists controllers, splitscreen, replays, HUD framework, UI pages, anticheat, chat filter, OptiFine, Polytone, HDR under both the Engine "Belongs" and the Platform "Belongs"; CLAUDE.md says `horizon-client-lib` "never depends on gameplay" (it contains `mobs/`); CLAUDE.md says `horizon-world` has "no gameplay loop" (it has player and combat rules); CLAUDE.md says `horizon-threads` is "thread priorities" (about 2,871 lines of profiler); CLAUDE.md groups `horizon-bench` with telemetry (Horizon) while layers.json says engine.
5. Layer-check status: all normal (non-dev) path deps from engine crates to non-engine crates are in `layers.json` `allowed` (25 entries, all from horizon-client, horizon-client-lib, horizon-client-render). Dev-dependency edges (`horizon-theme`, `horizon-perf`) are not checked by the tool. The portable crates (`horizon-assets`, `horizon-client-core`, `horizon-command`, `horizon-core`, `horizon-items`, `horizon-protocol`, `horizon-threads`, `horizon-world`) have only portable deps and none of the forbidden libraries (checked by reading manifests).

## Public surface (top 10 engine modules by distinct importing crates, code only)

Counts are distinct importing crates; brace-grouped `use horizon_x::{a, b}` are included. Full list in `top10.json` and `all_imports.json`.

| # | Module | Importers | Importing crates |
|---|---|---:|---|
| 1 | `horizon-world::nbt` | 7 | horizon-client, horizon-client-core, horizon-client-render, horizon-command, horizon-items, horizon-platform, horizon-protocol |
| 2 | `horizon-core::random` | 6 | horizon-assets, horizon-client, horizon-client-render, horizon-items, horizon-server-files, horizon-world |
| 3 | `horizon-core::prepared_files` | 6 | horizon-client, horizon-client-render, horizon-items, horizon-optifine, horizon-platform, horizon-world |
| 4 | `horizon-core::version` | 6 | horizon-client, horizon-client-render, horizon-platform, horizon-protocol, horizon-shaders, horizon-world |
| 5 | `horizon-protocol::crypto` | 5 | horizon-bots, horizon-client, horizon-client-render, horizon-platform, horizon-server-files |
| 6 | `horizon-world::datafix` | 5 | horizon-client, horizon-client-core, horizon-client-render, horizon-items, horizon-protocol |
| 7 | `horizon-core::position` | 5 | horizon-client, horizon-client-render, horizon-items, horizon-platform, horizon-world |
| 8 | `horizon-core::jmath` | 5 | horizon-client, horizon-client-render, horizon-command, horizon-protocol, horizon-world |
| 9 | `horizon-protocol::buf` | 4 | horizon-bots, horizon-client, horizon-command, horizon-server-files |
| 10 | `horizon-protocol::packets` | 4 | horizon-bots, horizon-client, horizon-client-render, horizon-command |

Notes: `horizon-platform`, `horizon-optifine`, `horizon-shaders`, `horizon-bots`, `horizon-game` and `launcher` are outside the engine and import engine modules; that is allowed (platform may depend on engine). Only the `horizon-core` and `horizon-world` modules are used from many crates; `horizon-client` is used from `launcher`, `horizon-game`, `horizon-mod-api-gen`, `cargo-horizon-mod` through `horizon_client::{mod_api, entry, native_data}`.

Crates importing each engine crate (distinct importers, code only): horizon-core 11, horizon-world 8, horizon-protocol 6, horizon-client 5, horizon-client-core 5, horizon-threads 4, horizon-system 4, horizon-bench 3, horizon-hooks 3, horizon-perf-format 3, horizon-assets 2, horizon-client-input 2, horizon-items 2, horizon-narrator 2, horizon-server-files 2, horizon-client-render 1, horizon-client-lib 1, horizon-command 0 (reached via `horizon_client::command`), horizon-prepare 0 (binary).

## Claims I did not verify (not claimed as findings)

- Whether any engine module is unused. I did not grep-verify callers for every module, so no module is called unused or duplicated.
- Whether `horizon-client-render` "reaches gameplay" (CLAUDE.md claim): not checked beyond the manifest.
- `game/emotes.rs`, `horizon-client/src/extras.rs`, `store.rs`, `upload.rs` and `entity_skin.rs` are named in my initial probe list but do not exist. They were not counted.
