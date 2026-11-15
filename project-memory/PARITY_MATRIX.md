# Matrice de paritate vanilla — Minecraft Java Edition 26.3

## Scop și surse canonice

Ținta este comportamentul observabil 1:1 al Minecraft: Java Edition 26.3. Această matrice organizează domeniile de verificare pentru proiect; **nu înlocuiește și nu copiază** infrastructura upstream.

Sursele canonice existente la commitul upstream fixat bcb7258ad101638c57e8cdd7b1e3da406440bea8:

- docs/rust-parity.md — starea generală și sursele live pentru paritate.
- docs/port-ledger.md și tools/port_ledger.py — maparea metode Java ↔ funcții Rust, metode lipsă, seam/drift/float/callee gaps și verdict map.
- tools/parity.py — enumerarea și starea harness-urilor.
- tools/test_*_parity.py — comparații diferențiale cu oracle Java.
- mods/horizon/src/test/java/dev/horizonlab/parity/ — oracle-urile Java.
- crates/*/examples/*_probe.rs și fixture-urile jocului — probe Rust.
- docs/known-issues.md — probleme observabile cunoscute.

Inventar verificat în arborele upstream fixat: **678 harness-uri** cu sufixul _parity.py, **397 fișiere Java oracle** și **115 fișiere TSV** în port map. Aceste numere descriu infrastructura existentă, nu demonstrează că toate cazurile sunt acoperite sau că paritatea este completă.

Upstream declară explicit că paritatea 1:1 nu a fost atinsă peste tot. Jocul este jucabil, dar niciun domeniu nu va fi marcat complet doar pe baza faptului că pornește sau că există un harness.

## Matrice la nivel de domeniu

| Domeniu vanilla | Zone principale indicate de upstream | Dovezi/teste canonice | Status în proiect |
|---|---|---|---|
| Versiune, matematică, RNG, coordonate și primitive | horizon-core: version, random, jmath, positions, noise, density, paletted storage | Port ledger; harness-urile existente pentru RNG/noise/math | Inventar preliminar; acoperirea exactă se extrage după import |
| World data și persistență | horizon-world: NBT, regions, chunks, datafix; horizon-client save/load | Save roundtrip, world compare și port ledger | Neauditat end-to-end |
| World generation | horizon-world/worldgen: biomes, noise, density, carvers, features, structures, placement | Harness-urile de worldgen/biome/carver/feature și oracle Java | Neauditat end-to-end |
| Block states, block entities, ticks și redstone | horizon-world/level și gameplay în horizon-client/game | Harness-urile block/fluid/redstone/tick și port ledger | Neauditat end-to-end |
| Fluide, foc, vreme și mediu | horizon-world + gameplay/rendering | Harness-urile fluid/weather și testele de world state | Neauditat end-to-end |
| Items, data components, enchantments și cooldowns | horizon-items + horizon-client/game | Harness-urile item/use/crafting și port ledger | Neauditat end-to-end |
| Inventar, meniuri, containere, crafting și trade | horizon-client-render pentru ecrane; horizon-client/game pentru acțiuni; horizon-items | Harness-uri de container/menu/crafting și oracle Java | Neauditat end-to-end |
| Entități, fizică, coliziuni și proiectile | horizon-world + horizon-client/game | Harness-uri entity/movement/collision/projectile și port ledger | Neauditat end-to-end |
| Mobs, AI, goals, navigation și spawning | horizon-client/mobs + gameplay; world data în horizon-world | Harness-uri mob_ai/spawning și blame din port ledger | Neauditat end-to-end |
| Player movement, mining, interaction și combat | horizon-client/game + horizon-client-input | Harness-uri de movement/interaction/use/attack și port ledger | Neauditat end-to-end |
| Effects, food, damage, potions și equipment | horizon-world + horizon-items + horizon-client/game | Harness-uri dedicate și port ledger | Neauditat end-to-end |
| Commands, selectors, functions și command blocks | horizon-command pentru parsing/tree; horizon-client/game pentru dispatch și efecte | Harness-uri command și port ledger | Neauditat end-to-end |
| Protocol, login, packets și sincronizare client/server | horizon-protocol + horizon-client/game remote/LAN/server | Harness-uri wire/protocol și lan_e2e | Neauditat end-to-end |
| Integrated server, dedicated server și LAN | horizon-client/game/integrated + lan + dedicated; horizon-server-files | Harness-uri dedicated_server/server_thread/LAN | Neauditat end-to-end |
| Dimensiuni, game rules, time și weather ticks | horizon-world + horizon-client-core + gameplay | Harness-uri world/tick/rule/time/weather | Neauditat end-to-end |
| Rendering, meshing, lighting, particles și camera | horizon-client-render + partea de randare din horizon-client/game | Harness-uri render/meshing/light/fog și comparații vizuale/structurale existente | Neauditat end-to-end |
| UI vanilla, HUD, screens, input și audio | horizon-client-render, horizon-client-input, horizon-narrator și gameplay | Harness-urile UI/input/audio existente; port ledger | Neauditat end-to-end |
| Resource packs, assets, models, fonts și localization | horizon-assets + horizon-client-render + horizon-client-core | Harness-uri de assets/model/resource pack și oracle-uri relevante | Neauditat end-to-end |
| Save/load, datafix și compatibilitate de versiune | horizon-world + horizon-client/game + horizon-server-files | Save roundtrip, reopen și port ledger | Neauditat end-to-end |
| Ordine, timing, float semantics și random draw order | Toate sistemele care reproduc metode Java | Port ledger gaps: seam, drift, float, callee; parity traces | Neauditat transversal |

## Reguli de acceptare

Un domeniu poate fi marcat verificat numai când:

1. Harness-ul compară aceleași intrări și condiții cu oracle-ul Java pentru versiunea 26.3.
2. Rezultatele observabile coincid, inclusiv valorile numerice, ordinea evenimentelor, consumul RNG, starea serializată și comportamentul pe mai multe tick-uri.
3. Sunt acoperite cazurile normale, limitele, interacțiunile dintre sisteme și scenariile client/server relevante.
4. Port ledger-ul nu lasă gap-uri relevante nerezolvate sau explicate printr-un verdict justificat.
5. Testele de regresie trec după orice modificare.
6. Un simplu boot, screenshot sau test happy-path nu este considerat dovadă de paritate.

Performanța poate fi optimizată intern numai dacă rezultatul observabil rămâne identic. Nu schimbăm ordinea random draws, matematica Java, float widening, ordinea de iterație sau timing-ul când acestea pot fi observate în joc.

## Cum se actualizează

- După import: rulează tools/parity.py --list și --status; înregistrează rezultatele și maparea harness → domeniu.
- Construiește/actualizează port ledger-ul cu tools/port_ledger.py build folosind instalarea legitimă Minecraft 26.3 a utilizatorului; fișierele Mojang rămân locale în .local și nu se comit.
- Folosește tools/port_ledger.py coverage, gaps și todo pentru a identifica metodele fără răspuns.
- Nu crea un al doilea ledger și nu copia lista de metode într-un document paralel.
- Adaugă harness nou numai când o responsabilitate vanilla nu are deja acoperire potrivită; fiecare harness trebuie să compare cu oracle Java independent.
- Nu șterge sau slăbi harness-uri existente doar pentru a obține CI verde.

## Limitări curente

- Aceasta este o matrice de domenii, nu un raport de acoperire măsurat.
- Nu am executat încă harness-urile: ele necesită jar-urile și resursele Minecraft ale utilizatorului în .local.
- Nu declarăm niciun domeniu complet până când statusul harness-urilor, port ledger gaps și testele end-to-end nu sunt analizate.
- Codul platform/Horizon aflat în crate-uri etichetate engine se exclude din refactorul vanilla, chiar dacă dependența lui trebuie păstrată pentru build.


## Pregătirea resurselor pentru CI

Procedura planificată pentru testele care au nevoie de oracle-ul Java este documentată în docs/private-vanilla-26-3-parity.md. Aceasta folosește numai surse oficiale Mojang, hash-urile din research-lock.json, spațiul temporar al unui runner efemer și harness-urile existente. Nu introduce un al doilea parity runner și nu publică JAR-uri/assets. Până când workflow-ul este implementat și executat, harness-urile rămân nevalidate în CI pe baza acestui mecanism.


## Pregătirea resurselor în CI — implementare

Workflow-ul manual .github/workflows/private-vanilla-parity.yml pregătește resursele prin tools/ci/vanilla_data.py și le importă cu horizon-prepare. Testele folosesc harness-urile canonice tools/parity.py; nu există un al doilea runner. Starea și criteriile sunt documentate în docs/private-vanilla-26-3-parity.md. Niciun caz nu este marcat PASS până la o execuție reală.


## Evidență disponibilă fără date Mojang (2026-11-15)

Sursa: fișiere comise în repository și comenzi read-only. Raport complet: `reports/engine-audit-2026-11-15/parity.md`. Niciun domeniu nu devine „verificat” prin aceste cifre: ele arată doar ce infrastructură există.

- `tools/port_ledger.py` nu poate fi interogat: are nevoie de `.local/` (jar și surse decompilate). `coverage`, `gaps`, `where`, `todo` răspund „No ledger yet”. Tipurile de gap (seam, drift, float, callee, class) nu sunt calculabile din repo.
- Harta de verdicte comisă (`tools/port_map/*.tsv`, 115 fișiere, 14.897 rânduri): 10.926 mapate (7.744 `port`, 3.182 `inline`), 423 `missing`, 3.509 `skip:*`, 39 `home`.
- 678 fișiere `tools/test_*_parity.py`; 397 fișiere Java oracle; 350 fișiere probe/fixture/bench Rust. `tools/parity.py --list` enumerează 682 de nume; `--status` arată toate 682 ca „never run”.
- 626 de harness-uri ajung la un oracle Java sau rulează jar-ul vanilla (după închiderea importurilor); 52 nu au nici oracle Java, nici rulare a jar-ului (de exemplu light, save_roundtrip, world_reopen, lan_integration). Regula 1 de acceptare cere oracle independent.
- 5 harness-uri fără probe: macro_source_type, mob_pickup_save, orb_saved_uuid, seed_sync, stand_fall_save (pot folosi o fixture prin variabilă de mediu, nepotrivită după nume).
- 28 de fișiere Java oracle nu sunt atinse de închiderea importurilor niciunui harness (unele sunt exportatoare sau ajutoare).
- `docs/known-issues.md` are 4 linii și nicio intrare (lista se generează la release); 0 probleme cunoscute nu înseamnă 0 gap-uri.
- Domenii cu acoperire subțire față de volumul mapat: mobs (1.677 metode mapate, 69 harness-uri), rendering (1.416, 14), commands (854, 20), protocol (820, 19). Domeniul transversal „order/timing/float” nu are niciun harness după nume.

Atenție: `tools/parity.py --list` și `--status` scriu `.local/parity-runner/state.json` în repo. `.local/` nu se comite; verifică `git status` după ce le rulezi.

Tabel pe domenii (asocierea cu domeniile se face după cuvinte-cheie din nume și căi: inferat):

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

