# Horizon parity audit (read-only, no .local data)

Repo: <repo> (branch main, clean git status after audit)
Helper scripts: <scratchpad>/rs-audit/
  (harness_audit2.py, harness_domains.py, tsv_domains.py; outputs harnesses.json, harness_domains.json, tsv_rows.json)

## Bottom line

- The port ledger cannot be queried: `tools/port_ledger.py` needs a build from the vanilla jar and decompiled sources in `.local/` (absent). `coverage`, `gaps`, `where`, `todo`, `map-check` all print "No ledger yet". Gap kinds (seam, drift, float, callee, class) are therefore not computable from the repo alone.
- Committed evidence is the verdict map (`tools/port_map/*.tsv`, 115 files, 14,897 rows): 10,926 mapped rows (7,744 port + 3,182 inline), 423 `missing`, 3,509 `skip:*`, 39 `home` rows. 678 harnesses, 397 Java oracle files, 350 Rust probe/fixture/bench files exist on disk.
- No parity domain is verified. `project-memory/PARITY_MATRIX.md` marks every domain "Neauditat end-to-end" and states the harnesses were never executed. `docs/known-issues.md` is 4 lines with no bullet items, so it lists zero known issues.

## Command results (run from repo root)

1. `python3 tools/port_ledger.py coverage|gaps --kind seam|where KineticWeapon.damageEntities|map-check tools/port_map/game-001.tsv|todo --limit 3`
   Output: `No ledger yet: run \`python3 tools/port_ledger.py build\` first.` (map-check: usage error, needs paths). Needs `.local/minecraft/26.3/sources` or a jar. Not worked around.
2. `python3 tools/port_ledger.py --help`: runs, shows subcommands and gap kinds. Read-only.
3. `python3 tools/parity.py --list`: last line `649 of 682 harnesses would run`; 682 harness names printed.
4. `python3 tools/parity.py --status`: all 682 harnesses show `never run`.
   Side effect: both `--list` and `--status` wrote `.local/parity-runner/state.json` inside the repo. I removed `.local/` after confirming I created it (it was absent before my first call). Avoid re-running parity.py in this checkout.
5. `cat tools/port_map/*.tsv | awk -F'\t' '{print $2}' | sort | uniq -c` (verdict counts):
   port 7744, inline 3182, skip:java 1223, skip:data 934, skip:other 653, missing 423, skip:debug 300, skip:service 185, skip:ui 174, skip:server 40, home 39 (total 14,897 lines).
6. Distinct counts from the TSVs (python over the same files):
   - 3,532 distinct classes with method rows; 2,935 have at least one port/inline row; 247 have a missing row.
   - 10,853 distinct Class.method keys mapped; 845 distinct Rust file/fn targets in the `where` column.
   - 301 class-level rows (266 with no method plus 39 `home`).
   - Missing rows: 281 notes begin "minor", 19 begin "unsure" (inferred from note text).

## Per-domain table

Columns: #harnesses = harnesses assigned to the domain by name keywords (inferred). #oracles = harnesses whose import closure reaches a Java oracle class (dev.horizonlab.parity.* or a Vanilla*Oracle) or runs the vanilla jar (net.minecraft.server/client Main, shown in brackets). Mapped = TSV port+inline rows whose `where` path falls in the domain (inferred from path keywords). Missing = not attributable: missing rows have an empty `where`, so they cannot be grouped by Rust domain; the global total is 423. "gap-note" = rows whose note contains the word "gap" (text match, not ledger gap kinds).

| PARITY_MATRIX domain | #harnesses | #oracles (java / jar) | mapped methods (port+inline) | missing / gap counts | known issues | evidence |
|---|---|---|---|---|---|---|
| Version, math, RNG, coordinates, primitives | 3 | 2 / 0 | 353 | missing n/a; gap-note 10 | 0 | harness_domains.json; tsv_domains.py on `horizon-core` paths |
| World data and persistence | 32 | 24 / 3 | 419 | missing n/a; gap-note 76; 3 harnesses without probe | 0 | harnesses.json; `world`/`save`/`datafix` paths |
| Save/load and datafix (merged into row above in matrix) | (in row above) | | | | 0 | PARITY_MATRIX.md row "Save/load, datafix" |
| World generation | 39 | 38 / 1 | 842 | missing n/a; gap-note 29 | 0 | `worldgen` / `biome` / `carver` paths |
| Block states, block entities, ticks, redstone | 103 | 97 / 21 | 1,091 | missing n/a; gap-note 47; 6 harnesses with no oracle | 0 | `horizon-world/src/level`, `block_ticks`, `redstone` |
| Fluids, fire, weather, environment | 36 | 30 / 11 | 417 | missing n/a; gap-note 28 | 0 | fluid/fire/weather paths |
| Items, data components, enchantments, cooldowns | 43 | 40 / 3 | 247 | missing n/a; gap-note 45 | 0 | `horizon-items`, `item_data`, `enchant` |
| Inventory, containers, crafting, trade | 34 | 32 / 3 | 532 | missing n/a; gap-note 39 | 0 | `menu`, `container`, `crafting`, `merchant` |
| Entities, physics, collisions, projectiles | 59 | 54 / 11 | 311 | missing n/a; gap-note 84 | 0 | `entities`, `projectile`, `minecart`, `boats` |
| Mobs, AI, goals, navigation, spawning | 69 | 62 / 13 | 1,677 | missing n/a; gap-note 243 | 0 | `mobs/`, `creatures`, `natural_spawning` |
| Player movement, mining, interaction, combat | 25 | 25 / 3 | 92 | missing n/a; gap-note 18 | 0 | `melee`, `use_hand`, `combat` |
| Effects, food, damage, potions, equipment | 22 | 22 / 0 | 67 | missing n/a; gap-note 12 | 0 | `effect`, `food`, `damage` |
| Commands, selectors, functions, command blocks | 20 | 16 / 0 | 854 | missing n/a; gap-note 112; 1 harness without probe | 0 | `horizon-command`, `commands` |
| Protocol, login, packets, client/server sync | 19 | 16 / 4 | 820 | missing n/a; gap-note 158; 1 harness without probe | 0 | `horizon-protocol`, `game/remote*` |
| Integrated, dedicated server, LAN | 31 | 29 / 18 | 140 | missing n/a; gap-note 40 | 0 | `integrated`, `dedicated`, `lan` |
| Dimensions, game rules, time, weather ticks | 9 | 9 / 0 | not separable (no matching Rust group) | n/a | 0 | harness names only |
| Rendering, meshing, lighting, particles, camera | 14 | 11 / 0 | 1,416 | missing n/a; gap-note 240 | 0 | `gfx`, `mesher`, `light`, `particles` |
| Resource packs, assets, models, fonts, lang | 13 | 13 / 1 | 216 | missing n/a; gap-note 19 | 0 | `horizon-assets`, `packs`, `font` |
| UI, HUD, screens, input, audio | 11 | 11 / 0 | 458 | missing n/a; gap-note 90 | 0 | `hud`, `settings_shell`, `keybinds`, `audio` |
| Order, timing, float semantics, RNG draw order (transversal) | 0 by name (inferred) | 0 | n/a | float/drift/seam gaps need ledger build | 0 | PARITY_MATRIX.md last row |
| Unassigned by name | 96 | 95 / 10 | 394 (Rust paths in unclassified modules) | n/a | 0 | harness_domains.json "Unclassified" |

Totals: 678 harnesses; 626 reach a Java oracle (by import closure), 52 have neither a Java oracle nor a jar run. Direct-only (harness file alone, no import closure): 419 reference a Java oracle, 653 reference a probe/example.

Other mapped rows not in the table: Core game loop (game.rs) 580 mapped; Horizon-only extras 0 harnesses by name.

Flagged domains:
- Zero dedicated harnesses: "Order, timing, float semantics" (transversal, by name only).
- Thin harness coverage against mapped volume: Mobs (1,677 mapped vs 69 harnesses), Rendering (1,416 vs 14), Commands (854 vs 20), Protocol (820 vs 19).
- Harnesses with no probe or fixture reference (5): macro_source_type, mob_pickup_save, orb_saved_uuid, seed_sync, stand_fall_save. These may use a game env fixture not matched by name.
- Harnesses with no Java oracle and no jar run (52), e.g. light, save_roundtrip, world_reopen, lan_integration, friends_integration, remote_fireball, remote_fireworks, spawn_loot, trident_save. Several are integration or save-roundtrip checks, not oracle-diffs. The PARITY_MATRIX acceptance rule 1 requires an independent oracle.

## Harness and oracle inventory

- Harness files: 678 `tools/test_*_parity.py` (matches PARITY_MATRIX.md).
- Java oracle files: 397 under mods/horizon/src/test/java/dev/horizonlab/parity/. Referenced by at least one harness (import closure): 369. Not referenced by any harness the closure reaches: 28 files, e.g. AllayDanceTranscript, ElderCurseTranscript, LevelEventTranscript, VanillaBlockBehaviourExporter, VanillaCreativeTabsExporter, VanillaEntityTypesExporter, VanillaLootFillOracle, VanillaSceneMotionOracle, VanillaStructurePlacementOracle, VanillaSoundRig, and others. Some are exporters or helpers, not oracles. Unreferenced may mean used by a path the closure does not follow.
- Rust probes/fixtures/benches: 350 files named *_probe, *_fixture, *_bench or under examples/probe, plus 8 game-level fixtures under crates/horizon-client/src (e.g. command_harness.rs, net_harness.rs, sculk_fixture.rs). Referenced by a harness: 323. Not referenced by name: 27, mostly benches (alloc_count_bench, audio_bench, chunk_load_bench, json_load_bench, ...) plus game fixtures reached through HORIZON_*FIXTURE env vars.
- Harnesses with a probe reference: 673; without probe and without HORIZON_*FIXTURE env: 5 (listed above).

## Known issues by domain

`docs/known-issues.md` (4 lines) holds no bullet items, and the file says bullets come from tools/release_train.py at release time. Result: 0 known issues in every domain. Open GitHub issues were not queried (network).

## Verdict-map notes (committed data)

- `missing` rows (423): 281 notes start with "minor", 19 with "unsure", the rest with a concrete reason. Examples: `UpgradeData` (block upgrades on chunk load not applied), `ClientDebugSubscriber.*` (server debug subscriptions never requested), `SelectBlockModel.*` (display-context switch missing), and many F3 debug renderers not drawn.
- Verdict `skip:java` (1,223) and `skip:data` (934) dominate the skipped rows; those are not counted as gaps.

## Not computable from the repo alone

- Ledger gap kinds (seam, client, drift, float, callee, class, stale, map, unported) and coverage percentages: need `build` from .local jar/sources.
- Harness results: `parity.py --status` reports "never run" everywhere; parity needs .local data.
- Per-package (Java package) grouping: TSV rows and harness names carry no package; packages exist only in the jar/sources. Domains above use Rust path and name keywords instead (inferred).

## Facts from memory

None used. The task was repo-only.
