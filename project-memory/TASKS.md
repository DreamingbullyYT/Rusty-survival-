# Task Ledger

## Finalizate
- [x] Confirmat repository-ul de lucru și scope-ul: numai engine-ul vanilla Horizon.
- [x] Fixat baseline-ul upstream la `horizon-rust/horizon` commit `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.
- [x] Importat snapshot-ul upstream în `main` prin GitHub Actions run #8; commit rezultat `e02bf6f8e3b3f2a8725fbd3975c92a192b2456cf`.
- [x] Rulate testele de import: 967 teste OK, 9 skipped.
- [x] Identificate și păstrate instrumentele upstream canonice: `tools/layers.json`, `tools/check_layers.py`, parity harnesses și port ledger.
- [x] Creat inventarul provizoriu al celor 19 crate-uri etichetate upstream ca engine.
- [x] Creată harta dependențelor directe Cargo și matricea de domenii vanilla 26.3.
- [x] Pregătită versiunea detectorului limitată implicit la sursele de producție ale crate-urilor engine, cu opțiunea explicită `--scope all`.
- [x] Pregătite teste pentru filtrarea scope-ului engine și excluderea testelor standalone/examples/benches/platform.
- [x] Integrate în branch-ul PR #1 regulile canonice, deciziile D-005/D-006 și documentele de audit.
- [x] Confirmată în CI rularea engine-only: 7 teste ale detectorului OK și scanarea a 2.292 fișiere / 34.788 funcții fără erori.
- [x] Identificat callsite-ul de producție jigsaw care creează `WorldgenRandom<LegacyRandom>` și separat de folosirea `WorldgenRandom<XoroshiroRandom>` pentru feature decoration.

## În curs
- [ ] Obținerea raportului detaliat al candidaților engine-only; artifact-ul nu a fost publicat de run #26, deci trebuie generat și păstrat controlat într-o rulare viitoare.
- [ ] Verificarea manuală a candidaților exacți/structurali; nu se șterge nimic automat.
- [ ] Finalizarea inventarului semantic pentru `horizon-core`: RNG/math/version, apoi positions/noise/density/storage/ticks și callsites.
- [ ] Căutarea tuturor constructorilor și callsites `WorldgenRandom` din întregul workspace; verificarea relevanței `rng-trace` pentru traseul jigsaw.
- [ ] Obținerea statusului real al harness-urilor prin `tools/parity.py --status` într-un mediu cu resursele Java 26.3 legitime în `.local`.
- [ ] Confirmarea grafului complet de dependențe Cargo, inclusiv transitive și feature gates.
- [ ] Extracția gap-urilor din port ledger și corelarea lor cu matricea de paritate.

## Mai târziu
- [ ] Definirea arhitecturii țintă minimală pe baza inventarului semantic.
- [ ] Refactor incremental, fiecare schimbare legată de cazuri de paritate și teste de regresie.
- [ ] Stabilizarea și verificarea parității 1:1 pentru toate domeniile vanilla.
- [ ] Proiectarea propriului Modding API — numai după stabilizarea engine-ului.


## Progres audit horizon-core — 2026-11-15

- [x] Prima trecere de module pentru poziții, geometrie/trig și noise/density volume; responsabilitățile și harness-urile existente sunt documentate în SYSTEM_REGISTRY și PARITY_MATRIX.
- [ ] Verifică exactitatea `Vec3::rotation` față de oracle și relația dintre `jmath::atan2`, Mth.atan2 și metodele native f64.
- [ ] Inspectează implementările complete Perlin/Simplex/SmearedPerlin/NormalNoise și urmărește callsites din worldgen; nu refactoriza doar pentru a reduce linii.
- [ ] Continuă inventarul `horizon-core` cu density graph, biome, carving, storage, ticks și data structures, apoi corelează cu harness/port ledger.


## Gap audit — funcționalități vanilla lipsă identificate în sursă

- [ ] Inventariază tipurile de density function și codec-urile vanilla 26.3 care lipsesc din `DensityGraph`; corelează fiecare cu oracle/harness înainte de implementare.
- [ ] Cartografiază pipeline-ul complet de terrain generation și marchează separat BIOMES, TERRAIN, CARVERS, FEATURES, STRUCTURES și LIGHT.
- [ ] Enumeră placed-feature roots omise din execuție și identifică exact algoritmii/bindings lipsă.
- [ ] Verifică implementarea și callsites pentru structure generation, fără a presupune că jigsaw piece assembly acoperă întregul sistem de structures.
- [ ] Extinde matricea tick cu block/fluid behavior, nu doar scheduled tick queue.
- [ ] Extinde matricea carving de la mask la cave/canyon generation și mask application.
- [ ] Extinde matricea biome de la zoom la source/storage/blending/chunk integration.
- [ ] Nu începe simplificarea acestor sisteme până când gap-urile de comportament au proprietari și teste de paritate.


## Storage/palette audit

- [x] Confirmată separarea rolurilor dintre numeric PalettedContainer (core), Tag NBT PalettedContainer și BlockSection (world), PackedStorage și CarvingMask.
- [ ] Verifică dacă duplicarea read/write din `horizon-core/src/paletted.rs` poate folosi `PackedStorage` fără alocări/overhead sau pierdere de fast paths; măsoară înainte de orice schimbare.
- [ ] Mapează callsites pentru toate aceste tipuri și harness-urile de wire/NBT/save/load înainte de a propune o unificare.


## Worldgen — următoarele verificări

- [x] Corectată clasificarea structures: sistemul de starts/references/jigsaw/piece placement există; suportul rămâne parțial și trebuie listate explicit kinds unsupported.
- [x] Confirmată separarea BIOMES/TERRAIN proto generation, STRUCTURE_STARTS/REFERENCES și FEATURES decoration.
- [x] Confirmate limitele density registry: DAG only, depth 256, interval branches 256, inline noise holders unsupported.
- [ ] Extrage lista exactă de StructureKind::Unsupported și Placement::Unsupported din toate bundle-urile vanilla 26.3; verifică dacă tipurile respective sunt vanilla sau doar Horizon/modded.
- [ ] Extrage toate placed-feature roots care întorc decode errors pentru Overworld/Nether/End din bundle-ul 26.3 și clasifică fiecare algoritm/binding lipsă.
- [ ] Rulează `tools/parity.py --list` și `--status` într-un mediu cu resursele/JAR-urile legitime 26.3; nu presupune că harness-urile sunt executate.
- [ ] Rulează `tools/test_terrain_parity.py`, apoi whole-chunk `tools/test_chunk_generation_parity.py` și harness-urile de structures; păstrează output-ul complet și separă KNOWN de mismatch nou.
- [ ] Verifică feature registry datapack semantics (named holders, tags, optional dependencies, reload) față de Java 26.3; feature_catalog este explicit doar snapshot bounded.
- [ ] Nu modifica implementarea înainte de lista completă a unsupported vanilla roots și a callsites lor.


## Feature registry / placement audit

- [x] Inventariate familiile de FeatureDefinition, PlacementModifier și BlockStateProvider direct din decoder-ele sursă.
- [ ] Cu bundle-ul 26.3 legitim disponibil local, înregistrează toate feature și placed-feature roots pe dimensiune; salvează numele și decode error-ul pentru fiecare root unsupported.
- [ ] Compară toate tipurile și codec-urile prezente în bundle cu match arms din FeatureDefinition, PlacementModifier, BlockStateProvider, BlockPredicate, IntProvider, HeightProvider și tree decorators.
- [ ] Verifică dacă limitele decoderelor (depth/node/list/modifier) sunt peste toate inputurile vanilla 26.3; orice limită care respinge un asset vanilla este un gap de paritate.
- [ ] Rulează feature catalog/graph/placement/region/sort/tree parity harnesses după ce probele și resursele locale sunt configurate.
