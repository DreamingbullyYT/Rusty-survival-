# Matrice de paritate vanilla — Minecraft Java Edition 26.3

## Scop și surse canonice

Ținta este comportamentul observabil 1:1 al Minecraft: Java Edition 26.3. Această matrice organizează domeniile de verificare pentru proiect; **nu înlocuiește și nu copiază** infrastructura upstream.

Sursele canonice existente la commitul upstream fixat bcb7258ad101638c57e8cdd7b1e3da406440bea8:

- docs/rust-parity.md — starea generală și sursele live pentru paritate.
- docs/port-ledger.md și tools/port_ledger.py — maparea metodelor Java ↔ funcții Rust, metode lipsă, seam/drift/float/callee gaps și verdict map.
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

## Detaliere — horizon-core RNG și matematică

Acoperirea existentă identificată în sursă include:
- worldgen random adapter, seeds, draw counters, forks, positional factories, bounds și legacy/xoroshiro modes (`tools/test_worldgen_random_parity.py`);
- random tick/scheduled tick state transitions și RNG fingerprint (`tools/test_random_tick_parity.py`);
- anumite fishing RNG paths, cu UUID-to-seed încă explicit neacoperit (`tools/test_fishing_rng_parity.py`);
- Float LogSampler comparat cu oracle-urile Java, cu mod exhaustiv opțional pentru toate cele 2^32 intrări float (`tools/test_float_log_parity.py`);
- noise și geometry au harness-uri separate.

Acestea sunt cazuri și harness-uri existente, nu rezultate de execuție în sesiunea curentă. Harness-ul worldgen documentează încă lipsuri pentru Gaussian samples, concurrent access și full chunk decoration. Paritatea double-precision Math.log, toate funcțiile Math pe toate arhitecturile și toate traseele RNG nu sunt dovedite de testele enumerate. Statusul fiecărui harness trebuie obținut din runner-ul `tools/parity.py --status` pe un mediu cu fișierele Minecraft 26.3 legitime disponibile.


## Detaliere — poziții, geometrie și noise

Harness-urile identificate în sursă, dar nerulate în această sesiune:

- `tools/test_geometry_parity.py`: corpus determinist cu vectori și AABB, valori speciale IEEE-754, coordonate de margine, trig table (65.536 intrări), rotații/look direction, traversal, voxel shapes, joins optimizate/neoptimizate, coliziuni și block iteration. Raportul harness-ului exclude explicit entități live, efecte reale de velocity/grounding, fluids și block behavior.
- `tools/test_noise_parity.py`: compară cu oracle Java gradient offsets/permutation tables, consumul RNG la construcție, Perlin 2D/3D și derivate, Simplex cu ambele moduri de offset, SmearedPerlin, NormalNoise octave construction, modern/legacy seeding, density volumes și noise stacks. Nu verifică grafurile serializate complete de density functions sau terenul generat complet.
- Testele unitare din `position.rs` acoperă negative coordinate floor semantics, distinct packet/storage layouts, packed signed limits și overflow.
- Testele unitare din `geometry/trig.rs` disting Mth.atan2 table-driven de fdlibm Math.atan2 și verifică vectori salvați pentru Java.
- Existența acestor harness-uri nu este rezultat de execuție; nu se marchează domeniile ca verificate până nu există status real și comparație cu oracle.


## Gap-uri de implementare cunoscute din comentariile sursă

Statusul de mai jos este mai precis decât eticheta generală „neauditat” și trebuie păstrat până la implementare + comparație cu oracle:

| Zonă | Limitare explicită în sursă/harness | Consecință pentru paritate |
|---|---|---|
| Density graph | Codec-urile serializate și tipurile rămase de funcții sunt pending | Nu se poate declara suport complet pentru definițiile vanilla de density |
| Terrain generation | `horizon-core::density` spune că generarea completă de terrain este pending | Noise/density sampler parity nu echivalează cu world terrain parity |
| Structures | `horizon-world::worldgen::generator.rs` spune că structurile nu sunt generate | Worldgen rămâne incomplet chiar dacă biome/noise/carving trec separat |
| Placed features | Algoritmii/bindings neimplementați sunt omiși din execuție, păstrând doar locul în ordering | Lipsesc efecte observabile ale unor features vanilla; seed-urile celorlalte sunt protejate |
| Scheduled ticks | Harness-ul `test_tick_parity.py` exclude block/fluid behavior | Tick queue parity nu dovedește block/fluid tick parity |
| Carving | Harness-ul carving mask exclude cave/canyon generation și mask application | Mask storage parity nu dovedește carving parity |
| Biomes | Harness-ul biome zoom exclude biome source/storage/blending/chunk integration | Quart lookup parity nu dovedește biome pipeline parity |

Acestea sunt gap-uri de comportament, nu oportunități de ștergere. Înainte de optimizare trebuie cartografiate toate callsites și dependențele fiecărui sistem lipsă.


### Palette/storage parity detail

- `tools/test_paletted_container_parity.py` compares `horizon-core::paletted::PalettedContainer` against the pinned Java client oracle for block and biome strategies. It exercises single/list/hash-map/global palette thresholds, palette growth, scripted edits, network write/read, canonical pack/unpack and reload hashes.
- This harness does **not** compare `horizon-world::chunk::PalettedContainer` (generic Tag/NBT adapter), `BlockSection` counts/free-slot reuse, or every `PackedStorage` consumer. Those require their own chunk/NBT/save-load coverage.
