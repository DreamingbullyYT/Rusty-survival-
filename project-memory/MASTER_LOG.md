# Master Log — append-only

> REGULĂ CRITICĂ: acest fișier este jurnal cronologic imuabil. Nu șterge, nu rescrie, nu reformula și nu înlocui intrări existente. Adaugă fiecare înregistrare nouă la final.

## 2026-11-14 — Inițializarea spațiului de lucru

- Repository confirmat: `DreamingbullyYT/Rusty-survival-`, branch `main`.
- Scopul repo-ului schimbat: spațiu de lucru pentru refacerea engine-ului Horizon.
- Scope: numai engine-ul; launcherul și Modding API-ul propriu sunt amânate până după refacerea engine-ului.
- Scheletul HNMod existent urmează să fie înlocuit în starea curentă a branch-ului; istoricul Git se păstrează.
- Au fost stabilite regulile anti-duplicare și memoria permanentă în `project-memory/`.
- A fost introdus detectorul inițial `tools/engine_audit.py`; capabilitățile și limitele sale sunt descrise în documentație.
- Sursa Horizon upstream nu a fost încă importată. Nu există încă un inventar verificat al sistemelor engine-ului.
- Nicio componentă de engine nu a fost încă rescrisă.

## 2026-11-14 — Reset și fundația de audit

- Confirmare explicită primită pentru folosirea repository-ului ca spațiu de lucru al engine-ului.
- Conținutul vechi HNMod a fost eliminat din arborele curent: `Cargo.toml`, `hnmod.project.toml`, `src/`, `tests/core_tests.rs` și vechiul workflow CI. Istoricul Git a fost păstrat.
- Commit-ul de reset și creare a memoriei: `c72dbe94fb8f00e2b9aa94ec4a58da7d12d307df`.
- Au fost create fișierele de memorie din `project-memory/`; `MASTER_LOG.md` rămâne append-only.
- A fost adăugat `tools/engine_audit.py`: scanare read-only pentru duplicări exacte, candidați structurali pe bază de tokeni normalizați și suprapuneri conceptuale declarate în registru.
- Detectorul folosește generare de candidați SimHash/LSH pentru a evita compararea exhaustivă a tuturor perechilor de funcții. Rezultatele sunt euristice, nu dovadă semantică.
- Au fost adăugate teste standard-library în `tests/test_engine_audit.py` și workflow-ul `.github/workflows/engine-audit.yml`.
- GitHub Actions run #1: SUCCESS (`c357530c82085d690abc4e151051e30cbfebd2c2`).
- GitHub Actions run #2: SUCCESS (`c1c793c1bfd6aa8b54508cf590c43ef71b65db01`).
- Arborele curent nu conține încă sursa Horizon; detectorul nu a auditat încă funcții reale din engine.
- Următorul pas: audit complet upstream Horizon, identificarea commit-ului exact și importul sursei fără a începe refactorul înainte de inventariere.

## 2026-11-14 — Upstream Horizon source imported

- Upstream baseline pinned to `horizon-rust/horizon` commit `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.
- Imported the upstream source snapshot into this repository while preserving project-memory, our duplicate detector, tests and CI.
- Upstream tree at the pinned commit contains 7,052 entries and 3,588 Rust source files; the source tree was not yet semantically audited at import time.
- Upstream already provides `tools/layers.json` and `tools/check_layers.py`; these are the canonical dependency-layer checks and will be reused, not reimplemented.
- Upstream documents engine, platform and Horizon layers. Our code-change scope is the vanilla engine only; launcher and platform/Horizon features are retained as references/build dependencies where needed, not refactor targets.
- The upstream layer inventory identifies 19 engine crates. This is a starting classification, not a claim that all gameplay has already been cleanly separated.

## 2026-11-15 — Import upstream confirmat și pregătirea auditului engine-only

- Importul snapshot-ului Horizon a fost confirmat prin GitHub Actions run #8 (SUCCESS); `main` a avansat la `e02bf6f8e3b3f2a8725fbd3975c92a192b2456cf`.
- Logul jobului confirmă fetch-ul upstream pinned `bcb7258ad101638c57e8cdd7b1e3da406440bea8`, rularea testelor, generarea raportului și push-ul commitului de import.
- Testele din workflow au raportat 967 teste OK și 9 skipped.
- Scanarea inițială a inclus toate sursele Rust: 41.988 funcții, 835 grupuri exacte și 11.343 candidați structurali. Rezultatele sunt doar candidați euristici și nu au fost folosite pentru ștergeri sau refactor.
- Branch-ul de integrare pornește din commitul de import și aduce regulile canonice, inventarul preliminar, harta dependențelor, matricea de paritate și detectorul cu scope engine-only.
- Testele detectorului engine-only și raportul restrâns trebuie confirmate de CI înainte de a folosi rezultatele pentru analiză semantică.
- Nicio componentă a engine-ului nu a fost încă refactorizată; urmează inventarierea semantică, nu ștergerea de cod.

## 2026-11-15 — Izolarea testelor de audit față de fixture-urile CI upstream

- PR #1 run `43652682189` a eșuat deoarece `unittest discover` a inclus teste upstream care presupun prezența workflow-urilor și template-urilor `.github` originale, pe care importul le-a eliminat intenționat pentru a păstra configurația CI a repository-ului nostru.
- Logul confirmă 922 teste, 9 skipped, 12 erori și 1 failure; problemele observate sunt fișiere workflow/template lipsă și un test care așteaptă cel puțin 10 runner jobs upstream.
- Suite-ul complet upstream a trecut înainte de curățarea `.github` în import run #8 (967 OK, 9 skipped). Pentru CI-ul proiectului, workflow-ul de audit a fost restrâns la testele dedicate `test_engine_audit.py`, urmate de scanarea read-only engine-only.
- Nicio modificare a engine-ului și nicio ștergere de cod nu a rezultat din această problemă.

## 2026-11-15 — CI engine-only și prima trecere semantică horizon-core

- PR #1, run #26 (`43652931734`), job `audit-tool` (`132625133568`): SUCCESS. Au trecut 7 teste `test_engine_audit.py`; scanarea a procesat 2.292 fișiere Rust și 34.788 funcții, raportând 555 grupuri exacte și 4.415 candidați structurali. Raportul detaliat nu a fost publicat ca artifact.
- A fost creat branch-ul `audit/horizon-core-primitives` din head-ul PR #1 `ab17491fd78b2f31b02feddcc9ef679ff46fafaa` pentru documentarea auditului semantic fără a modifica PR-ul de fundație.
- Prima trecere asupra `horizon-core` a cartografiat version, LegacyRandom/XoroshiroRandom, positional/owner/ordered/worldgen RNG adapters și dispatch-ul jmath/fdlibm.
- Au fost identificate harness-uri Java↔Rust pentru worldgen RNG, random ticks, fishing RNG, Float LogSampler, noise și geometry; ele nu au fost rulate în această sesiune.
- Candidat de investigat, nu dublură confirmată: LCG bit sampling din `LegacyRandom::bits` și `WorldgenSource for LegacyRandom::worldgen_bits`. Cea din urmă poate evita `sync()`/trace; callsites și utilizarea cu OrderedDraws trebuie verificate înainte de orice schimbare.
- Niciun fișier de engine nu a fost modificat; nu s-a executat niciun test Cargo sau harness Java↔Rust în această sesiune. Paritatea 1:1 nu este declarată.

## 2026-11-15 — Verificare suplimentară a callsite-urilor WorldgenRandom

- În `horizon-world/src/worldgen/generator.rs::jigsaw_pieces`, callsite-ul de producție inspectat construiește un LegacyRandom local și îl învelește în WorldgenRandom pentru jigsaw placement; nu este transmis un lane OrderedDraws.
- `horizon-world/src/worldgen/generator_decoration.rs` folosește WorldgenRandom<XoroshiroRandom> pentru feature decoration. Fixture-ul dedicat testează ambele surse, nesting-ul wrapper-elor și operațiile de seed/fork/positional.
- Asta reduce riscul ipotetic de sincronizare pentru callsite-ul jigsaw; nu îl transformă într-un bug. Bit adapter-ul continuă să ocolească rng-trace, dar trace-ul este descris ca diagnostic pentru LegacyRandom/mob AI. Trebuie verificat dacă această diferență este intenționată și dacă mai există constructori în workspace.
- `horizon-client-render/src/random_source.rs` este o responsabilitate distinctă pentru randoms ne-seed-uite și alocarea ordonată a seed-urilor; nu este duplicarea generatorului worldgen.
- Nu au fost modificate implementările RNG și nu au fost rulate teste; s-a actualizat numai documentația de audit.


## 2026-11-15 — Prima trecere poziții, geometrie și noise

- Inspectate `position.rs`, `geometry/mod.rs`, `geometry/trig.rs`, `jmath.rs`, `noise/mod.rs`, `noise/perlin.rs`, `noise/simplex.rs`, `noise/normal.rs` și `noise/volume.rs`.
- Confirmate formate distincte BlockPos packet/storage/packed, shift semantics pentru coordonate negative și overflow wrapping; testele unitare acoperă limite și roundtrip.
- Confirmată separarea semantică între Mth.atan2 table-driven și fdlibm Math.atan2; nu sunt dubluri. `Vec3::rotation` folosește native f64 atan2/asin și rămâne un punct de validat prin oracle, nu un bug confirmat.
- Harness-urile `test_geometry_parity.py` și `test_noise_parity.py` au corpus diferențial extins, însă nu au fost rulate în această sesiune. Scope-ul lor are limite explicite față de entități live, grafuri complete de density functions și terrain.
- Nicio implementare engine nu a fost modificată; nicio concluzie de paritate completă nu a fost trasă.


## 2026-11-15 — Gap-uri vanilla explicite identificate în baseline

- Comentariile din `horizon-core/src/density/mod.rs` și `density/graph/mod.rs` declară că generarea completă a terenului, codec-urile serializate și unele tipuri de funcții density sunt încă pending.
- Comentariile din `horizon-world/src/worldgen/generator.rs` și `generator_decoration.rs` declară că structurile nu sunt generate și că placed features fără algoritm/bindings sunt omise din execuție.
- Harness-urile inspectate au scope limitat: tick parity exclude block/fluid behavior; carving-mask parity exclude cave/canyon generation și aplicarea mask-ului; biome-zoom parity exclude biome storage/source/blending/chunk integration.
- Concluzie: baseline-ul upstream nu poate fi considerat încă 1:1 complet. Paritatea samplerelor individuale nu echivalează cu paritatea worldgen-ului sau a jocului complet.
- Următoarea etapă trebuie să transforme aceste limitări într-o listă de funcționalități lipsă cu owner/callsites/oracle; nu se elimină sau simplifică nimic pe baza acestor lipsuri.


## 2026-11-15 — Audit de reprezentări packed storage și palettes

- Confirmat că `horizon-core::storage::PackedStorage` este SimpleBitStorage padded și este folosit în `horizon-world/src/chunk.rs` pentru adaptorul NBT generic și BlockSection.
- `horizon-core::paletted::PalettedContainer` este un container numeric separat pentru block/biome IDs și wire/saved forms; folosește aceiași helperi `locate/mask`, dar își păstrează propriile fast paths și operații bulk.
- `horizon-world::chunk::PalettedContainer` stochează Tag NBT + câmpuri extra; `BlockSection` are BlockStateId, counts și free-slot reuse. Nu sunt duplicate directe ale containerului core.
- `CarvingMask` folosește bit layout contiguu Y/Z/X și nu este SimpleBitStorage; nu se unifică pe baza tipului `Vec<u64>`.
- Rămâne doar un candidat de optimizare internă pentru low-level read/write din paletted core, care necesită analiză de performanță și parity tests. Nicio schimbare de engine.


## 2026-11-15 — Confirmarea scope-ului PalettedContainer oracle

- Inspectat `tools/test_paletted_container_parity.py`: compară `horizon-core::paletted::PalettedContainer` cu oracle-ul Java client pentru block/biome strategies, praguri Single/List/HashMap/Global, creștere palette, editări, wire read/write și pack/unpack/reload.
- Harness-ul nu acoperă direct `horizon-world::chunk::PalettedContainer` (Tag/NBT), BlockSection counts/free-slot reuse sau toți consumatorii PackedStorage. Aceste straturi necesită probe chunk/NBT/save-load separate.
- Nici acest harness nu a fost rulat în sesiunea curentă; doar scope-ul sursei a fost inspectat.


## 2026-11-15 — Corecție și detaliere worldgen/structures

- Corectez explicit nota anterioară care spunea generic „structurile nu sunt generate”. Inspecția extinsă arată că există STRUCTURE_STARTS și STRUCTURE_REFERENCES, structure-set placements, validare biome, weighted selection, jigsaw assembly, saved-start loading și piece placement în FEATURES.
- `generator_structures.rs` orchestrează starts/references/cache; `structure/start.rs` generează starts și pieces; `generator_decoration.rs` plasează piesele în FEATURES. Nu sunt un singur sistem duplicat, ci etape distincte.
- Sunt implementate mai multe structure kinds vanilla (buried treasure, igloo, swamp hut, desert pyramid, jungle temple, mansion, monument, ruined portal, shipwreck, ocean ruin, mineshaft, stronghold, fortress, nether fossil, End city și jigsaw). Există fallback `Unsupported` pentru tipuri/placements neacoperite și limite de jigsaw reach; verdictul corect este suport parțial, nu absență și nu paritate completă.
- Density decode suportă o familie largă de funcții vanilla și named holders; limitările declarate sunt DAG only, depth max 256, interval branches max 256 și inline noise holders unsupported. Nu mai descriem generic „codec-urile density lipsesc”.
- `feature_catalog.rs` declară explicit că este snapshot, nu datapack registry loader complet/live reload; `feature_graph.rs` execută arbori inline bounded. Unsupported placed-feature roots sunt omise din execuție, păstrând per-step order și seed-urile următoare.
- `test_chunk_generation_parity.py` compară chunk-uri întregi (block states, quart biomes, block entity NBT) folosind un oracle Java care rulează stages vanilla; `test_structure_layout_parity.py`, `test_structure_blocks_parity.py` și `test_terrain_parity.py` verifică subdomenii separate. Harness-urile nu au fost rulate în această sesiune.
- Nicio logică de engine nu a fost modificată; concluziile sunt source-backed, nu rezultate de execuție.
