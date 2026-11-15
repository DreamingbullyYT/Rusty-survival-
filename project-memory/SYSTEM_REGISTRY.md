# System Registry — sursa adevărului pentru ownership

## Stare

Inventar inițial de crate-uri realizat din manifestele upstream și docs/layers.md la commitul bcb7258ad101638c57e8cdd7b1e3da406440bea8.

**Atenție:** lista de mai jos este o hartă de orientare, nu un registru final de ownership. Clasificarea upstream engine este la nivel de crate; documentația upstream confirmă că unele crate-uri conțin și funcționalități de platformă/Horizon. Nicio funcție nu este declarată aici ca owner canonic până nu inspectăm modulele, callsite-urile, testele și fluxul de date.

## Inventar provizoriu — 19 crate-uri clasificate upstream ca engine

| Crate | Responsabilitate declarată în manifest | Încadrare preliminară pentru scope-ul vanilla | Stare |
|---|---|---|---|
| horizon-assets | Decodare modele și resurse dependente de versiune | Candidate vanilla: resurse/model decoding | Manifest inspectat; sursa neauditată |
| horizon-bench | Format raport benchmark comun jocului și launcherului | Tooling/performance, nu comportament vanilla | În afara țintei de refactor |
| horizon-client | Client nativ de dezvoltare Horizon | Mixt: gameplay vanilla plus host/platform/Horizon; gameplay-ul încă locuiește aici | Audit prioritar |
| horizon-client-core | Chat/traduceri, SNBT/NBT paths, scoreboard, boss bars, game rules, registry/particle tables | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-client-input | Key bindings, mouse, controllere, clipboard, window options | Mixt: tastatură/mouse vanilla plus platform features | Audit prioritar |
| horizon-client-lib | Sisteme client reutilizabile, control tool și parity probes | Mixt: sisteme de joc, instrumente și integrare platform | Audit prioritar |
| horizon-client-render | Date client/world, chunks, meshing, lumină, streaming, renderer, HUD, meniuri și settings | Mixt: randare/ecrane vanilla plus UI/settings platform | Audit prioritar |
| horizon-command | Arbore Brigadier vanilla, argumente, selectors, parsing, suggestions și forma de rețea | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-core | Fundații de simulare pentru versiunea Java țintă | Candidate vanilla | Audit semantic în curs |
| horizon-hooks | Extension points și platform registries pentru crate-urile superioare | Infrastructură de extensie; nu presupunem că este Modding API-ul propriu | Audit de contracte necesar |
| horizon-items | Fapte despre block/item, item stacks, data components, enchantments, cooldowns, bundles, NBT | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-narrator | GameNarrator/text-to-speech și starea screen narration | Candidate vanilla: mecanismul de narrator | Verificare backend/platform necesară |
| horizon-perf-format | Contractul de captură .hzperf, protobuf și gzip writer | Tooling/performance, nu comportament vanilla | În afara țintei de refactor |
| horizon-prepare | Importă/verifică jar-urile Minecraft ale jucătorului și exportă resurse | Build/resource tooling, nu runtime vanilla | În afara țintei de refactor |
| horizon-protocol | Protocolul de rețea Java 26.3, protocol 777 | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-server-files | Fișiere vanilla de server: properties, EULA, ops, whitelist, bans și account lookups | Candidate vanilla, cu verificare pentru partea de account lookup | Manifest inspectat; sursa neauditată |
| horizon-system | Informații despre OS, CPU, memorie, drive, power și GPU | Suport platform/performance, nu gameplay vanilla | În afara țintei de refactor |
| horizon-threads | Scheduling hints, allocator și apeluri OS | Infrastructură de suport; se păstrează dacă este necesară, fără extindere arbitrară | Audit de dependențe necesar |
| horizon-world | Date world vanilla, NBT, regiuni și chunk storage | Candidate vanilla | Manifest inspectat; sursa neauditată |

## Observații importante

- docs/layers.md definește engine-ul ca Vanilla Minecraft Java Edition 26.3, dar spune explicit că gameplay-ul încă locuiește în horizon-client și că va fi mutat gradual spre crate-urile portabile.
- horizon-client, horizon-client-input, horizon-client-lib și horizon-client-render nu pot fi tratate ca vanilla pure doar pentru că întregul crate este etichetat engine.
- Documentația upstream indică deja funcționalități de platformă aflate în crate-uri engine: game/mod_api_host.rs, game/native_*.rs, shader_ui.rs, hdr_calibration.rs, codul de controller și splitscreen.
- Nu mutăm și nu eliminăm nimic în această etapă. Mai întâi se identifică exact codul, consumatorii, extension points și comportamentul vanilla observabil.
- Nu proiectăm un Modding API nou acum. horizon-hooks se inspectează doar pentru a înțelege contractele existente și rolul său în engine/platform boundary.

## Regula de ownership

Fiecare responsabilitate are un singur owner canonic. Aliasurile, fațadele și adapterele trebuie să indice owner-ul real și să nu replieze logica.

## Format pentru fiecare sistem confirmat

- ID stabil:
- Nume:
- Owner canonic (crate/modul/fișier):
- Responsabilitate unică:
- Ce NU face:
- API intern:
- Dependențe:
- Consumatori:
- Teste:
- Status:
- Dovezi / commit:

Detectorul conceptual citește intrări [[system]] din project-memory/system_registry.toml. Menține acel fișier în sincron cu acest registru. Intrările automate se adaugă numai după confirmarea semantică a sistemelor; inventarul provizoriu de crate-uri de mai sus nu se transformă automat în ownership.

## Hartă de module indicată de AGENTS.md upstream

Sursa: AGENTS.md din commitul bcb7258ad101638c57e8cdd7b1e3da406440bea8. Acestea sunt indicii de ownership documentate upstream; confirmarea finală se face în cod și prin callsites.

| Responsabilitate | Locații declarate upstream | Notă pentru audit |
|---|---|---|
| Gameplay loop, tick, frame | horizon-client/src/game.rs și src/game/ | Principalul owner actual al gameplay-ului; nu mutăm înainte de inventariere |
| Executabil client | horizon-client/src/main.rs și entry.rs | Entry/executable, separat de logica gameplay propriu-zisă |
| Sisteme client reutilizabile, control și probes | horizon-client-lib; manifestul indică surse existente în horizon-client/src | Atenție la surse partajate prin manifest; nu presupunem duplicate doar pentru că există două crate-uri |
| Rendering, chunks, meshing, light, streaming, HUD, meniuri | horizon-client-render și modulele re-exportate de horizon-client | Documentația spune explicit că acest crate nu ajunge la gameplay |
| Date world, NBT, regions, chunks, datafix, worldgen, block ticks/redstone, entity physics | horizon-world | Owner de date și comportamente de world/server-side, fără gameplay loop |
| Block/item facts și item components | horizon-items | Consumate de gameplay; trebuie urmărit fluxul de date către Game |
| Command parsing/tree | horizon-command | Separat de dispatch/executarea comenzilor, care rămâne în horizon-client/game |
| Client bottom layer: text/lang/SNBT/NBT path/scoreboard/bossbar/gamerules/registry/particles | horizon-client-core | Candidate de logică vanilla |
| Key bindings/mouse/controller/clipboard/window | horizon-client-input | Mixt; izolăm input vanilla de controllere și alte extensii |
| Protocol codecs/framing/encryption/packets | horizon-protocol | Candidate vanilla; target protocol 777 |
| Server-side files/config | horizon-server-files | Verificăm separat account lookups și orice comportament Horizon |
| Version, Java RNG/Math, positions/noise/density/paletted storage | horizon-core | Primitive de simulare ce trebuie să reproducă exact Java |

### Consecințe pentru inventar

- horizon-client-lib nu trebuie numărat automat ca o a doua implementare a logicii din horizon-client: manifestul poate indica fișiere sursă aflate în alt crate. Verificăm canonical path și Cargo target înainte de a marca duplicate.
- Parsing-ul comenzilor și executarea lor sunt responsabilități diferite: horizon-command deține tree/argument parsing; gameplay-ul din horizon-client deține dispatch și efectele comenzilor.
- horizon-world deține logică world/server-side (inclusiv ticks/redstone), în timp ce horizon-client deține bucla de gameplay și coordonarea client/server. Nu le unim conceptual fără a distinge responsabilitățile.

## Audit semantic — horizon-core: prima trecere asupra primitivelor

Baseline: upstream `bcb7258ad101638c57e8cdd7b1e3da406440bea8`, branch de audit derivat din PR #1.

### Module confirmate în sursă

- `crates/horizon-core/src/version.rs`: constante generate pentru Minecraft 26.3, protocol 777, DataVersion 5023, formatele resource/data pack și Java 25. Fișierul indică generatorul `tools/gen_version_consts.py`; nu se editează manual.
- `crates/horizon-core/src/random.rs`: API-ul comun `RandomSource`, implementările `LegacyRandom` (48-bit Java LCG) și `XoroshiroRandom`, cache Gaussian, funcții comune de distribuție și wiring către adaptoare.
- `random/hash.rs`: MD5 pentru seed-uri vanilla derivate din nume.
- `random/ordered.rs`: coordonează draw-urile unui singur stream LegacyRandom între lane-uri paralele, păstrând ordinea listei seriale.
- `random/owner.rs`: tipuri `EntityRandom` și `LevelRandom` peste LegacyRandom; sunt protecții de ownership/type-safety, nu generatoare distincte.
- `random/positional.rs`: adaptoare positional pentru legacy și xoroshiro, Java string hash UTF-16 și position seed.
- `random/worldgen.rs`: wrapper WorldgenRandom cu draw counter, bit adapter, moduri wrapped/plain, delegare fork/positional și seed helpers pentru worldgen.
- `crates/horizon-core/src/jmath.rs`: dispatch Math compatibil cu comportamentul HotSpot în funcție de arhitectură/platformă.
- `jmath/fdlibm.rs`: implementări fdlibm și variante FMA folosite pentru traseele Java relevante; `jmath.rs` mai include codul `exact` pentru cazurile Windows documentate.
- `crates/horizon-core/src/lib.rs`: expune aceste primitive împreună cu biome, carving, density, noise, storage, ticks și alte module; crate-ul nu pretinde că reprezintă singur întregul joc.

### Observații de ownership și minimizare

- RNG-ul nu este un singur algoritm redundant: Legacy LCG, xoroshiro, positional seeding, wrapper-ul worldgen și stream-ul cu draw-uri ordonate au contracte diferite documentate.
- `owner.rs` nu calculează random numbers independent; delegă către LegacyRandom și adaugă constrângeri de tip pentru owner.
- `WorldgenRandom` repetă intenționat operații de bit sampling și primitive RandomSource pentru a păstra semantica wrapper-ului Java, inclusiv contorul și distincția wrapped/plain. Nu se comasează cu LegacyRandom înainte de verificarea tuturor constructorilor/callsites.
- `WorldgenSource for LegacyRandom` actualizează direct state-ul LCG în `worldgen.rs`, în loc să treacă prin `LegacyRandom::bits`. Constructorul de producție identificat până acum este în `horizon-world/src/worldgen/generator.rs::jigsaw_pieces`, unde creează un LegacyRandom local și îl învelește în WorldgenRandom; nu este un lane OrderedDraws. Astfel, ipoteza că acest callsite ocolește gate-ul OrderedDraws nu este susținută de traseul inspectat.
- Acest adapter ocolește în continuare `LegacyRandom::bits` și deci hook-ul `rng-trace`; trebuie stabilit dacă trace-ul este intenționat limitat la traseele mob AI și dacă mai există constructori de producție înainte de a decide dacă este relevant.
- `horizon-world/src/worldgen/generator_decoration.rs` folosește separat `WorldgenRandom<XoroshiroRandom>` pentru decorarea feature-urilor; test fixture-ul `worldgen_random_fixture.rs` verifică ambele surse și wrapper nesting.
- `horizon-client-render/src/random_source.rs` este o responsabilitate separată: creează randoms ne-seed-uite și coordonează alocarea seed-urilor din mob loop cu `OrderedDraws`; nu este un al doilea generator LCG.
- `jmath` nu poate fi înlocuit în bloc cu funcțiile standard Rust: comentariile și dispatch-ul documentează diferențe HotSpot între x86-64, AArch64 și Windows, inclusiv fdlibm/FMA și rezultate corect rotunjite.
- Testele unitare locale din `random.rs` verifică secvența JDK LCG, semnul low word în nextLong, input-uri invalide fără consum de state, resetarea cache-ului Gaussian și fallback-ul stării xoroshiro zero. Testele fdlibm verifică vectori StrictMath și cazuri HotSpot AArch64.

### Harness-uri de paritate identificate, dar nerulate în această sesiune

- `tools/test_worldgen_random_parity.py`: compară oracle Java 26.3 cu Rust pentru legacy/xoroshiro, wrapper depth 1–3, draw counters, seed/fork/positional factories, bounds și seed helpers. Scriptul documentează încă drept pending Gaussian samples, concurrent access și full chunk decoration.
- `tools/test_fishing_rng_parity.py`: acoperă anumite trasee de fishing RNG/motion/loot; propriul docstring precizează că UUID-to-seed relationship nu este încă acoperită.
- `tools/test_random_tick_parity.py`: compară random/scheduled ticks și fingerprint-ul RNG după fiecare apel.
- `tools/test_float_log_parity.py`: compară Float LogSampler cu două JAR-uri vanilla fixate; modul implicit verifică intervale de input, iar `--exhaustive` poate verifica toate cele 2^32 pattern-uri float. Nu dovedește paritatea `Math.log(double)` sau a întregului sistem numeric.
- Au fost identificate și harness-uri separate pentru noise și geometry; existența lor nu dovedește că au trecut în baseline-ul curent.

### Status

**Parțial inspectat; fără schimbări de engine.** Nu au fost executate aici testele Cargo sau harness-urile Java↔Rust. Nu s-a stabilit încă graful complet de callsites pentru RNG/math, nu s-au triat candidații auditului pentru aceste module și nu se declară paritate completă.


## Audit semantic extins — poziții, geometrie și noise (prima trecere)

Baseline-ul rămâne commitul upstream `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.

### Poziții

- `crates/horizon-core/src/position.rs` deține `BlockPos`, `SectionPos`, `ChunkPos` și `BlockBoxIter` (re-export din `position/iter.rs`).
- Sunt distincte trei formate care nu trebuie confundate: poziția relativă în packet (X/Z/Y în ordinea documentată), indexul de storage al secțiunii (Y/Z/X) și packed long-urile pentru BlockPos/SectionPos/ChunkPos.
- Implementarea folosește shift aritmetic pentru coordonate negative și overflow wrapping explicit; testele unitare verifică limite signed, roundtrip în interval, coordonate negative și overflow i32.
- Harness-ul de geometry include block-box iteration, limite de coordonate și aliasing packed Y; nu a fost executat aici.

### Geometrie și trigonometrie

- `crates/horizon-core/src/geometry/mod.rs` deține vectori, AABB, direcții, mișcare/coliziuni și operații VoxelShape; modulele movement/shape/sweep/traversal sunt separate pe responsabilități.
- `geometry/trig.rs` conține două familii distincte:
  - trigonometria vanilla Mth: tabelul de 65.536 valori float, sin/cos cuantizate, Mth.atan2 table-driven și conversii în grade;
  - funcțiile fdlibm asin/atan/atan2 folosite pentru StrictMath/Math, inclusiv delegarea din `jmath::atan2`.
- Nu tratăm `geometry::trig::atan2` și `geometry::trig::fdlibm_atan2` ca dubluri: prima este algoritmul Mth tabelat, a doua reproduce Java Math.atan2/StrictMath.atan2.
- `Vec3::rotation` folosește în prezent metodele native `f64::atan2/asin`, în timp ce `direction_from_rotation` folosește tabela Mth. Această diferență este observabilă numeric și trebuie validată cu oracle-ul înainte de a înlocui apelurile.
- `tools/test_geometry_parity.py` generează un corpus mare pentru vectori, AABB, coliziuni, traversal, voxel shapes, rotations/look direction și toate intrările tabelei trig; scope-ul său nu include live entities, efectele de velocity/grounding sau fluid/block behavior. Nu a fost rulat în această sesiune.

### Noise și volume

- `noise/mod.rs` deține `GradientTable`, reducerea periodică `wrap`, gradientele Perlin/Simplex și primitivele comune de interpolare.
- `noise/perlin.rs` deține Perlin și SmearedPerlin; `noise/simplex.rs` deține Simplex; `noise/normal.rs` deține configurarea octave/amplitude și construcția NormalNoise/NoiseStack; `noise/volume.rs` deține layout-ul și indexarea DensityVolume.
- Perlin și Simplex împart intenționat GradientTable și tabelul gradientelor, dar au algoritmi de sampling și precizie diferite. Perlin folosește coordonate double, interpolare/derivate float; Simplex expune atât traseul double cât și rezultatul cast-uit float. Nu comasa algoritmii pe baza faptului că împart tabele.
- NormalNoise include trasee moderne, legacy Nether, positional seeding și failure draw-state; consumul RNG la inițializare este parte din comportamentul observabil.
- `tools/test_noise_parity.py` compară cu două JAR-uri vanilla fixate offsets/permutations, consum RNG, Perlin/Simplex/SmearedPerlin, derivate, NormalNoise, density volumes, buffer tails și noise stacks. Harness-ul exclude explicit definițiile serializate, grafurile complete de density functions, terenul generat și gameplay-ul. Nu a fost rulat aici.

### Status extins

Poziții, geometrie și noise sunt inspectate doar la nivel de module/API/teste/harness. Nu s-au executat Cargo tests sau oracle harnesses, nu s-au inspectat încă toate callsites-urile din horizon-world/horizon-client și nu s-a stabilit o arhitectură țintă. Nicio modificare de engine.


## Gap-uri explicite confirmate în codul upstream — nu sunt ținte de simplificare

Acestea sunt declarații din comentariile sursă, nu deducții din testele enumerate:

- `crates/horizon-core/src/density/mod.rs`: compilatoarele/samplerele de density sunt încă un graph compiler inițial; generarea completă a terenului este în așteptare.
- `crates/horizon-core/src/density/graph/mod.rs`: seeded noise registration este suportată, dar codec-urile serializate și restul tipurilor de funcții sunt în așteptare.
- `crates/horizon-world/src/worldgen/generator.rs`: pipeline-ul actual descrie BIOMES/TERRAIN și decoration parțială; comentariul precizează că structurile nu sunt generate.
- `crates/horizon-world/src/worldgen/generator_decoration.rs`: placed-feature roots pentru care algoritmul sau bindings nu sunt implementate sunt omise din execuție; poziția lor în ordering este păstrată pentru a nu schimba seed-urile feature-urilor rămase.
- `tools/test_tick_parity.py`: harness-ul verifică mecanismul scheduled-tick, nu comportamentul block/fluid.
- `tools/test_carving_mask_parity.py`: verifică mask-ul, nu generarea cave/canyon, aplicarea mask-ului, aquifers sau terrain complet.
- `tools/test_biome_zoom_parity.py`: verifică seed obfuscation și quart zoom, nu biome storage/source generation, blending sau chunk integration.

Aceste limitări arată că baseline-ul importat nu este încă un engine 1:1 complet. Nu începem refactorul de minimizare în aceste zone până când inventarul lipsurilor vanilla, callsite-urile și planul de paritate sunt explicite. Orice funcționalitate lipsă trebuie completată fără a introduce o arhitectură paralelă sau un Modding API prematur.


## Audit storage/palettes — ownership și reprezentări distincte

- `horizon-core/src/storage.rs::PackedStorage` este implementarea comună pentru `SimpleBitStorage`: valori packed în words padded, fără valori care traversează limita unui long. Expune get/set, slot access, word runs și unpack.
- `horizon-core/src/paletted.rs::PalettedContainer` este containerul numeric resident pentru block/biome IDs, cu Single/List/Global palettes, wire representation, canonical saved `Packed`, resize și fast bulk operations. Refolosește helper-ele low-level `storage::locate` și `storage::mask`, dar nu `PackedStorage` ca obiect.
- `horizon-world/src/chunk.rs::PalettedContainer` este adaptorul de NBT pentru valori `Tag` arbitrare și câmpuri extra; rezolvă ulterior block states prin registry. Folosește `PackedStorage` și este parte a persistence/chunk layer, nu înlocuitor direct al containerului numeric din horizon-core.
- `horizon-world/src/chunk.rs::BlockSection` este secțiunea runtime cu BlockStateId, counts/free slots și PackedStorage; face tranziția uniform → packed și re-folosește sloturi de palette. Acesta este încă un rol distinct de adaptorul Tag NBT și de containerul numeric core.
- `horizon-core/src/carving.rs::CarvingMask` folosește un bitset contiguu Y/Z/X cu cursor mutabil; nu are formatul padded SimpleBitStorage. Nu unificăm aceste structuri doar fiindcă toate conțin `Vec<u64>`.
- Concluzie: există trei tipuri de container palette/section în straturi diferite și un bitset de carving. Ele împart helper-ele de bit addressing acolo unde e posibil, dar nu sunt duplicate semantice confirmate. Orice unificare trebuie să păstreze formatul NBT, wire format, palette growth order, free-slot reuse, buffer bulk behavior și performanța.


### Harness ownership note — palette/storage

- `tools/test_paletted_container_parity.py` targets only `horizon-core::paletted::PalettedContainer` and its wire/saved representations against the Java client oracle.
- It does not by itself validate the Tag/NBT adapter or BlockSection in `horizon-world/src/chunk.rs`; those are separate layers and must be verified through chunk NBT/save-load harnesses.


## Worldgen — structură reală a pipeline-ului și limite confirmate

### Chunk pipeline

- `horizon-world/src/worldgen/generator.rs` descrie `generate_proto` ca rulând statusurile BIOMES și TERRAIN; pipeline-ul de terrain include noise fill cu aquifers, material surface și carvers, apoi heightmaps. Nu este doar un noise sampler.
- `terrain.rs` este owner-ul noise-stage chunk fill și al tranziției ProtoChunk/TerrainChunk; comentariul său precizează că nu produce singur un chunk complet. Structure lifecycle și feature decoration sunt etape separate.
- `generator_decoration.rs` rulează FEATURES pentru un 3×3 neighborhood, cu structuri și placed features. Rădăcinile de placed feature ale căror algoritmi sau bindings nu sunt implementate sunt omise din execuție, dar poziția în per-step ordering este păstrată pentru a nu altera seed-urile feature-urilor următoare.
- `generator_structures.rs` deține STRUCTURE_STARTS și STRUCTURE_REFERENCES, inclusiv cache, start generation, saved starts, referințe pentru chunk-uri vecine și beardifier.

### Structures — corecție față de nota anterioară

Nota anterioară „structurile nu sunt generate” este prea generală și trebuie interpretată ca limitare a anumitor trasee din pipeline, nu ca absență a sistemului:

- Există generare de starts prin `create_starts` / `try_generate`, structure-set placement (random spread, concentric rings, dimension origin), validare biome, weighted selection și jigsaw assembly.
- Sunt implementate mai multe structuri vanilla cu piese dedicate: buried treasure, igloo, swamp hut, desert pyramid, jungle temple, mansion, monument, ruined portal, shipwreck, ocean ruin, mineshaft, stronghold, fortress, nether fossil și End city; jigsaw structures sunt decodate și asamblate prin pool pieces.
- Structurile au etapă separată de plasare a pieselor în FEATURES, prin `place_in_chunk`, după ce starts/references sunt construite.
- Sunt suportate saved starts/legacy pieces în limitele formatelor implementate; la load, un start cu copii nesuportați sau necitibili este regenerat în loc să fie acceptat parțial.
- Există tipuri de structure cu `StructureKind::Unsupported`, placements unsupported și limite specifice pentru jigsaw reach. Prin urmare suportul este parțial, nu absent și nici complet 1:1.

### Density graph și feature registry

- `horizon-world/src/worldgen/density.rs` decodează named density holders și inline roots pentru o familie largă de funcții vanilla: unary/binary arithmetic, constant, blend fields, beardifier, end islands, cache, blend density, pow, round/floor/ceil/truncate, gradient, noise/shift, old blended noise, slice, interpolated, find_top_surface, clamp, range_choice, lerp, interval_select, distance_to_point și spline.
- Limitările explicit declarate: registrul acceptă numai grafuri aciclice, adâncime maximă 256 și cel mult 256 ramuri interval; inline noise holders nu sunt suportate. Acestea sunt limite de implementare concrete, nu dovada că lipsesc toate codec-urile.
- `feature_catalog.rs` este un snapshot imutabil al resurselor; comentariul declară că nu este un loader complet de datapack registries și nu oferă live reload. `feature_graph.rs` execută un arbore inline bounded, în timp ce named holders/tags/reload rămân separate.
- Feature roots cu decode errors sunt păstrate în identitate/order și raportate ca unsupported, nu executate. Acest lucru menține RNG order pentru rădăcinile rămase, dar efectele lipsă rămân diferențe de gameplay.

### Nivel de certitudine

Aceste concluzii provin din citirea sursei și a descrierilor harness-urilor. Nu s-au executat cargo tests, parity harnesses sau worldgen end-to-end în această sesiune. Nu se declară worldgen complet.


## Feature execution — ownership și suprafață de codec

### Configured feature graph

- `feature_graph.rs` este arborele executabil al unui configured/placed feature, nu registry loader. Include basic End/platform features; block column/pile/disk/spring/vines/snow-freeze/delta/bamboo/replace blobs/end podium/gateway/bonus chest/chorus/speleothem/spike/blue ice/underwater magma/block blob/simple block; ore/scattered ore; trees/fallen trees; vegetation patch/coral/huge mushroom/multiface; lake/monster room/geode/iceberg/root system/dripstone/fungus/sculk/end spike; template/fossil; și compoziții weighted/sequence/overlay/random/boolean/chance.
- Unele feature kinds cer `FeatureBlockBindings`, state-provider catalogs, biome/fluid/block tags, template resources ori callbacks de world. Așadar „type recunoscut” nu înseamnă că orice root poate fi executat fără toate resursele și bindings.
- Decoderul limitează graph depth la 128, nod budget la 65.536, feature lists la 8.192 și modifier chains la 1.024. Acestea sunt protecții de input/work, iar paritatea trebuie verificată pentru resurse vanilla reale, nu presupusă din limite.

### Placement modifiers

`placement.rs::PlacementModifier` include biome, block predicate, environment scan, heightmap, surface-relative threshold, surface water depth, count/count-on-every-layer, noise-based count, noise-threshold count, in-square, cuboid, offset, height range, rarity, random chance, fixed placement și randomly selected. `placement/chain.rs` deține executarea ordonată cu buffer pending reutilizabil; `placement/layer.rs` deține layer scan și sampling intercalat al count provider-ului. Nu dubla aceste două execuții.

### State providers și block-writing behavior

- `state_provider.rs` deține simple, weighted, random block, rotated, copy properties, randomized int, rule-based și noise/noise-threshold/dual-noise providers; `state_provider/catalog.rs` rezolvă named holders, iar `state_provider/noise.rs` deține selecția noise.
- `block_feature.rs` deține bindings și feature-uri de block-writing precum fill layer și replace single block; `generator_blocks.rs` leagă proprietățile/collision/support/survival ale blocurilor necesare features.
- `feature.rs::BasicFeature` are doar void start platform, End island și End platform; nu reprezintă catalogul complet, care este în `feature_graph.rs`.
- `decoration.rs` deține scheduling-ul biome→feature per step și seed index; `generator_decoration.rs` adaptează planul la 3×3 world region și adaugă structurile în ordinea vanilla.

### Limita de inventar rămasă

Nu putem spune care roots concrete din bundle-ul vanilla 26.3 sunt omise până nu inspectăm resursele autorizate extrase local (feature/placed_feature registries, tags, block facts, templates). Codul arată suportul tipurilor și decoderelor; lista roots efectiv unsupported este calculată la încărcarea bundle-ului și expusă de `unsupported_features()`.
