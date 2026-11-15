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
