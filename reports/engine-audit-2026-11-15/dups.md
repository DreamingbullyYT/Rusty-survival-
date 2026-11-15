# Duplicate audit of reports/upstream-bcb7258-audit.json (read-only)

Repo: <repo> (Horizon fork). Nothing under the repo was modified. No cargo commands were run.

## Scope, method and caveats

- Input: `reports/upstream-bcb7258-audit.json`. It holds 835 exact groups (2,446 members), 11,343 structural candidates, and 0 conceptual candidates. It contains only (path, function) pairs, so I re-extracted bodies with the scanner's own helpers (`tools/engine_audit.py`, `extract_functions`, `normalized_tokens`, `shingles`, `simhash`). I used a regex-positioned version of `mask_rust` with the same semantics, because the original slices `source[i:]` per character and does not finish in reasonable time. The re-extraction gives 41,988 functions, and its 835 exact groups match the JSON exactly (sorted member lists equal).
- The JSON is an **all-scope** scan (tests, examples, tools included). `DO_NOT_DUPLICATE.md` says the engine-only scope should run in CI, so the numbers here are not engine-only.
- Important caveat: the hash is over normalized tokens in which **identifiers and literal values are abstracted**. "Exact" therefore means "same token shape", not "same text". Many groups are shape-identical with different meanings (for example, two `name()` enum-to-string tables). Per-function verdicts below come from reading the bodies, not from the group label.
- Class rules are applied in this order; the first match wins:
  - **A test-only**: every member is under `tests/`, `examples/` or `benches/`, is a `tests.rs` file, is a module file declared `#[cfg(test)] mod x;` (including `#[path]` declarations and their children), or sits inside an inline `#[cfg(test)]` span.
  - **E generated/protocol/table-like**: any member path matches a `generated`, `gen`, `protocol`, `packets`, `adapter`, `mappings`, `tables`, or `vN_N` component; or the file header says generated/DO NOT EDIT; or literal/number tokens are at least 30% of the body.
  - **B trivial**: at most 3 top-level statements (a trailing expression counts as one). Caveat: a single `match` is one statement, so B includes large enum-to-string tables (see Pattern notes).
  - **C cross-crate**: members in two or more crates (crate = `crates/<name>`; otherwise the first path segment).
  - **D same-crate**: everything else.
- Duplicated lines = body lines × (members − 1). Raw lines = body lines × members. Body lines are non-blank lines of the body with comments removed.

## 1. Class table (all 835 exact groups)

| Class | Groups | Members | Duplicated lines | Raw lines |
|---|---:|---:|---:|---:|
| A test-only | 363 | 1,154 | 6,117 | 9,666 |
| B trivial (≤3 statements) | 431 | 1,169 | 4,479 | 7,075 |
| C cross-crate, non-trivial | 4 | 17 | 66 | 95 |
| D same-crate, non-trivial | 20 | 42 | 296 | 562 |
| E generated/protocol/table-like | 17 | 64 | 307 | 431 |
| **Total** | **835** | **2,446** | **11,265** | **17,829** |

Within B, 102 of 431 groups have bodies of 8 or more lines (2,236 of the 4,479 duplicated lines). These are mostly enum-to-string match tables that the statement count treats as trivial.

Only 24 groups fall in C or D, so the "top 25" requested is the full set. All 24 are reviewed below.

## 2. Verdicts for C and D (ranked by duplicated lines)

Verdict key: **REAL** = identical semantics, one owner proposed. **COINCIDENTAL** = same text, different meaning (including mirrors of different vanilla methods; do not merge). **UNCLEAR** = needs a vanilla or call-site check first.

"Vanilla" references come from the code's own comments or names. I did not run `tools/rea_vanilla.py`, so every vanilla-class mapping below is inferred and must be confirmed before a change.

| # | Group (members) | Body lines / saved | Verdict | Proposed owner | Evidence |
|---|---|---|---|---|---|
| 1 | `sense_players`: mobs/animal_brain, warden_brain, villager_brain (3) | 23 / 46 | UNCLEAR | horizon-client `mobs/` shared sensing module | Animal and warden bodies identical. Villager differs in one line: `can_target_player(w, FOLLOW_RANGE, true)` (villager_brain.rs:663) instead of `m.attrs.follow_range`. Doc comment says `PlayerSensor` (vanilla). Confirm the villager range source against vanilla before merging. |
| 2 | Test fixtures `scratch`, `temp`, `temp_dir`, `gen_world`, `dir`, `test_dir` (10 members, 7 crates) | 4 / 36 | REAL (test-only scaffolding) | none, keep per crate (low value) | Each builds a unique `temp_dir()` path with a crate-specific prefix; all sit in `#[cfg(test)]` or test files. Consolidating would need a new dev-only helper crate, which the layer config does not provide. |
| 3 | `boat_hit` / `cart_hit` (game/boats.rs, game/minecarts.rs) | 29 / 29 | UNCLEAR | horizon-client `game/` ray-pick helper taking a bounds iterator | Same ray-vs-box loop over different entity lists (6 changed lines). Boat and minecart are separate vanilla entities (inferred), so hitbox behaviour may differ. |
| 4 | `back_to_worlds` / `back_to_players` (realms.rs) | 25 / 25 | REAL (UI flow) | realms.rs helper `back_to_configure(tab)` | Differ only in the `SubScreen` variant matched and the `ConfigureTab` value. Two and one call sites. |
| 5 | `painting_hit` / `frame_hit` (game/paintings.rs, game/frames.rs) | 24 / 24 | REAL | horizon-client `game/` shared hanging-entity pick helper | Differ only in the collection (`paintings` vs `item_frames`). Paintings and item frames are both hanging entities in vanilla (inferred from names). Keep the best-hit tie rule when merging. |
| 6 | `guest_mute_command` / `guest_tempban_command` (game/chat_filter.rs, game/tempban.rs) | 22 / 22 | REAL | game/lan_actions.rs (sole caller) | Differ in the permission constant (`MUTE_LEVEL` vs `TEMPBAN_LEVEL`) and the delegate. Both called only from lan_actions.rs:3571 and :3576. Horizon-only feature, not vanilla. |
| 7 | `sense_hurt` (mobs/monster_brain, mobs/villager_brain) | 20 / 20 | REAL | horizon-client `mobs/` shared sensing module | Bodies identical; the 1-line "diff" is whitespace. `animal_brain.rs:621` also calls it. The doc says the sensor mirrors vanilla `HurtBySensor`, which is one class, so a single owner matches vanilla's structure. |
| 8 | `closest_expiration`: realms.rs:641 (production) and realms.rs:12475 (inside `mod tests`) | 16 / 16 | REAL (test shadow copy) | realms.rs:641 (production) | The test module reimplements the production function (0 changed lines). Its asserts at realms.rs:12151 to 12156 call the copy, so they do not exercise the shipped function. Proposed change: call `super::closest_expiration` and keep the assertions unchanged. The copy's doc comment says it mirrors `InviteCodeExpiration.closestTo`, so keep the assertions as the vanilla pin. |
| 9 | `from_server_toml`: horizon-chat-filter/src/lib.rs, horizon-anticheat/src/lib.rs | 15 / 15 | REAL | horizon-server-files (engine), helper `section_only(text, key)` | Identical TOML-section extraction; only the section name differs. chat-filter already depends on server-files. anticheat (platform) may depend on engine crates under the layer rules, but has no Horizon deps today. |
| 10 | `place_configured_feature` / `place_placed_feature` (worldgen level/growth.rs) | 15 / 15 | COINCIDENTAL (mirror of different vanilla calls) | none, keep separate | Differ only in `features.place_configured` vs `features.place_placed`. Vanilla `ConfiguredFeature.place` and `PlacedFeature.place` are different methods (inferred from names). Parity-sensitive. |
| 11 | `sense_living` (animal_brain, monster_brain, villager_brain) | 7 / 14 | REAL | horizon-client `mobs/` shared sensing module | Bodies identical (0 changed lines); each calls `living_near(m, w)`. |
| 12 | `introduce` / `code_hello` (horizon-friends/src/direct.rs) | 14 / 14 | UNCLEAR | horizon-friends direct.rs, parameterised handshake | Same handshake shape but different wire `MAGIC` vs `CODE_MAGIC`, read timeouts 2s vs 4s, and error text. Different wire semantics; merge only with parameters that keep both exact. |
| 13 | `rotate_left` / `rotate_right` (horizon-world/src/java_hash_map.rs) | 11 / 11 | COINCIDENTAL (mirror of Java methods) | keep; if merged, one helper with a direction flag | Symmetric red-black rotations. The file name suggests a port of `java.util.HashMap.TreeNode` rotations (not verified). Parity-sensitive. |
| 14 | `corridor_left` / `corridor_right` (worldgen/structure/fortress.rs) | 11 / 11 | COINCIDENTAL (different vanilla pieces) | none; box lists could become data | Differ in box coordinates and fields. Vanilla `CorridorLeft` and `CorridorRight` are separate pieces (inferred from names). |
| 15 | `from_json` router / aquifer config (horizon-world worldgen/router.rs, aquifer/config.rs) | 10 / 10 | REAL (mechanics only) | horizon-world worldgen field-decode helper (where `required` lives) | Same `required(&object, field.name())` loop over two field enums. The codecs are different vanilla codecs, so only the loop should merge. |
| 16 | SplitMix64 `next`: climate/index.rs (test-only), horizon-server-files properties.rs (`SystemEntropy`), horizon-bots main.rs | 5 / 10 | REAL | horizon-core/src/random.rs: new `SplitMix64` | Same golden-gamma SplitMix step in three places. horizon-core's random.rs has only `XoroshiroRandom` (its `GOLDEN_RATIO`). world and server-files already depend on core; bots is a tool crate. Verify that properties.rs `next` is SplitMix before merging. |
| 17 | `apply` / `apply_secret` (horizon-protocol/src/rsa.rs) | 8 / 8 | COINCIDENTAL (intentional) | none; do not merge | `apply` uses `mont.pow`, `apply_secret` uses `mont.pow_secret`. The secret variant is a side-channel defence. Merging would remove it. |
| 18 | `world_open_failed` / `world_close_final` (game/native_lifecycle.rs) | 7 / 7 | REAL (small) | native_lifecycle.rs | Same "take the attempt if its state matches" shape; the state constant differs (`Opening` vs `Closing`). |
| 19 | `arrive` on `Mob` and `Boat` (game/server_travel.rs, two `Rideable` impls) | 6 / 6 | REAL | server_travel.rs, `Rideable::arrive` default method | Identical bodies. The `Minecart` impl differs by using `motion`, so it stays an override. |
| 20 | `draw_overlay_message` / `draw_title` (game/titles.rs) | 6 / 6 | COINCIDENTAL (mirror of different HUD paths) | none | Differ only in `titles.overlay_message` vs `titles.title`. Overlay and title are different vanilla HUD elements (inferred from names). Parity-sensitive. |
| 21 | `grass_color` in horizon-client item_render.rs and horizon-assets examples/item_model_fixture.rs | 5 / 5 | REAL | horizon-assets (owns the colormap); fixture must not depend on horizon-client | The example copies the production colormap index formula. Two other `grass_color` implementations exist (item_draw.rs:86, title/flat.rs:937), not compared here. |
| 22 | `draw` in gfx/core_sky.rs and gfx/stars.rs | 4 / 4 | REAL (small) | horizon-client-render gfx helper `draw_static(pass, pipeline, group, buffer, count)` | Same four-line wgpu pass sequence. The name `draw` has 312 callers in the crate, so only this pair was checked. |
| 23 | `on_alloc` / `on_free` (horizon-threads/src/alloc_count.rs) | 4 / 4 | REAL (small) | same file: one `bump` wrapper parameterised by field | Same pattern; fields differ (`allocs`/`bytes` vs `frees`/`freed`). |
| 24 | `answer_code` / `answer` (horizon-friends/src/direct.rs) | 4 / 4 | UNCLEAR | same as #12 | `CODE_MAGIC` vs `MAGIC` on the same handshake shape. Different wire semantics. |

Counts: REAL 15 (including #2, test-only), UNCLEAR 4 (#1, #3, #12, #24), COINCIDENTAL 5 (#10, #13, #14, #17, #20).

### Parity flags

Do not merge these without a harness run against the pinned 26.3 oracle:
- #1, #7, #11: sensors that mirror vanilla `Sensor` classes. The villager `FOLLOW_RANGE` difference in #1 must be confirmed.
- #10, #13, #14, #20: mirrors of different vanilla methods or pieces.
- #12, #24: wire-level handshakes with different magic and timeouts.
- #17: intentional constant-time exponentiation.

No group proposed here removes a parity harness or a test that pins vanilla behaviour. #8 keeps the test assertions and changes only the call target.

## 3. Structural candidates (sample of 15)

Scope of the structural list: 11,343 candidates, of which 3,253 are non-test and cross-file. 2,675 of those 3,253 have score 1.0. The scorer compares token shapes with identifiers and literals abstracted, so it mostly matches enum-to-string `match` tables and tiny wrappers. Sampled pairs (highest similarity, one per crate pair):

- **Enum name / label tables** (about 9 of 15: gamepad/gameplay_guide, scoreboard/graphics_style, graphics/pvp, shelf/test_block, stages/graphics, scoreboard/gamepad, realms/test_block, command/args/graphics_ui, realms/api). The bodies are `match self { Variant => "literal", … }`. These are false positives as duplicates (for example, a visibility enum and a gamepad button enum match only on shape). The pattern to replace is hand-listed name tables: one table per type, or a derive, and for vanilla registry names, generation from the pinned data so the parity source is not hand-typed.
- **Static-table lookups** (theme `get`, datafix `lookup_str`): `iter().find(|(k,_)| …).map(…)`. One generic table-lookup helper would cover them.
- **Hook forwarding** (native_client_hooks `chat_decorate` vs native_load_hooks `loot_condition`): `if let Some(h) = HOOKS.get() { (h.field)(…) }`. A macro or one forwarder type would cover the family.
- **Recursive tree flatten** (advancements `add` vs game/commands `add`): identical DFS over `kids`. One generic helper.
- **Real duplicate across crates** (lan/forwarding `hex` vs horizon-protocol/crypto `hex`): identical hex decode. Canonical owner: horizon-protocol crypto (engine), if the layers allow the call.
- **Per-version protocol copies** (horizon-protocol adapter `v1_21_8` vs `v1_21_5` `serverbound`): identical 7-line state-to-constant matches. These follow the version adapters and fall in E. A per-version id table would collapse them, but it touches wire data, so it needs the packet harnesses.
- **Fmt forwarding** (blue_ice vs simple_block `fmt`): identical forwarding `match`es on different enums. Mechanical, low value.

## 4. Estimate (not measured)

- Raw duplicated lines in REAL verdicts (excluding the test-only #2): 182 (#4 25, #5 24, #6 22, #7 20, #8 16, #9 15, #11 14, #15 10, #16 10, #18 7, #19 6, #21 5, #22 4, #23 4).
- Estimated net saving after shared helpers, which add roughly 60 to 90 lines across about 11 helper sites: **about 90 to 120 production lines**. This is a rough estimate, not a measured count.
- UNCLEAR groups add at most 93 raw lines (#1 46, #3 29, #12 14, #24 4), and only if the vanilla or wire checks pass.
- #2 (test scaffolding, 36 lines) is excluded from the estimate.

## 5. Files

- Scripts and intermediates, all outside the repo: `<scratchpad>/rs-audit/dups/` (`extract.py`, `classify.py`, `review.py`, `groups.json`, `ranked_CD.json`, `top25.txt`, `funcs.pkl`).
- Repo files read (read-only): `tools/engine_audit.py`, `project-memory/DO_NOT_DUPLICATE.md`, `project-memory/DECISIONS.md`, `CONTRIBUTING.md` (parity section), `tools/layers.json`, `reports/upstream-bcb7258-audit.json`.
- `docs/layers.md` was not read in full; layer rules came from `tools/layers.json` and `CONTRIBUTING.md`.
