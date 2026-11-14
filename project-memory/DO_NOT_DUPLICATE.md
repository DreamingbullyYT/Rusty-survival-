# Do Not Duplicate

## Regula

Nu crea un sistem nou înainte să verifici dacă aceeași responsabilitate există deja, indiferent de numele fișierelor, crate-urilor sau funcțiilor.

## Intrări confirmate

Momentan nu există dubluri confirmate în engine, deoarece sursa engine-ului nu a fost încă importată.

## Candidați

Se adaugă aici numai după audit, cu:
- responsabilitatea comună;
- locațiile implicate;
- dovezile;
- owner-ul canonic ales;
- decizia și planul de migrare.

## Existing upstream architecture tooling to reuse

- `tools/layers.json` — canonical crate-layer and dependency policy.
- `tools/check_layers.py` — existing layer/dependency checker.
- `docs/layers.md` — definitions and intended boundaries for engine/platform/Horizon.
Do not implement a competing layer registry or dependency-layer checker. Extend the existing one only if a demonstrated gap requires it.

## Rezultate detector — fără concluzii automate

Scanarea inițială de import a fost prea largă (toate sursele Rust), a raportat candidați exacți/structurali și nu a confirmat semantic nicio dublură. Nu elimina și nu modifica cod pe baza acelor numere. Versiunea engine-only trebuie să ruleze în CI, iar fiecare candidat relevant trebuie verificat manual prin ownership, callsites, dataflow și teste.

## Surse partajate care nu trebuie confundate cu duplicări

- `horizon-client-lib` declară în manifest `[lib] path = "../horizon-client/src/client_lib.rs"`; este aceeași cale de sursă expusă ca target Cargo separat, nu două implementări independente.
- Parsing-ul comenzilor (`horizon-command`) și dispatch-ul/efectele comenzilor (`horizon-client/game`) sunt responsabilități distincte.
- Rendererul care citește world state nu devine automat owner al regulilor de gameplay.

## Candidat pentru revizuire semantică — bit sampling WorldgenRandom

- Locații: `crates/horizon-core/src/random.rs` (`LegacyRandom::bits`) și `crates/horizon-core/src/random/worldgen.rs` (`WorldgenSource for LegacyRandom::worldgen_bits`).
- Ambele conțin actualizarea LCG `state = state * MULTIPLIER + 11 mod 2^48` și extragerea biților.
- Nu este declarată dublură confirmată: `WorldgenRandom` are propriul contract Java, draw counter și wrapped/plain behavior.
- Callsite de producție verificat până acum: `horizon-world/src/worldgen/generator.rs::jigsaw_pieces` construiește un `LegacyRandom` local, apoi îl învelește în `WorldgenRandom`; nu transmite un lane `OrderedDraws`. În decorarea feature-urilor, callsite-ul identificat folosește `WorldgenRandom<XoroshiroRandom>`.
- Prin urmare, ocolirea gate-ului `OrderedDraws` nu este un defect demonstrat în callsite-ul jigsaw inspectat. Adapterul ocolește `LegacyRandom::bits` și hook-ul `rng-trace`; relevanța acestei diferențe trebuie verificată față de scopul trace-ului și toate celelalte callsites.
- Decizie curentă: păstrează ambele implementări; nu le comasa. Încheie inventarul tuturor constructorilor și testează comportamentul înainte de orice refactor. Detectorul structural singur nu poate rezolva cazul.


## Clarificare — Mth trig vs Java Math trig

În `horizon-core/src/geometry/trig.rs` coexistă algoritmul tabelat `Mth.atan2` și funcțiile `fdlibm_atan/fdlibm_asin/fdlibm_atan2`. Nu sunt două implementări ale aceleiași responsabilități: Mth folosește lookup table și aproximarea specifică jocului; fdlibm reproduce StrictMath/Math. `jmath::atan2` delegă la `fdlibm_atan2`.

Un punct separat de verificat este `Vec3::rotation`, care folosește `f64::atan2/asin` native, pe când `direction_from_rotation` folosește Mth trig. Nu schimbăm apelurile până când harness-ul geometry și oracle-ul Java nu confirmă comportamentul bit cu bit.


## Clarificare — PalettedContainer și packed storage

- `horizon-core/src/paletted.rs::PalettedContainer`: palette numerică residentă pentru IDs block/biome și wire/saved forms; folosește `storage::locate/mask`.
- `horizon-world/src/chunk.rs::PalettedContainer`: adaptor generic pentru NBT `Tag` plus câmpuri extra; convertește apoi valorile prin registry. Folosește `PackedStorage`.
- `horizon-world/src/chunk.rs::BlockSection`: section runtime BlockStateId cu counts, free slots și PackedStorage.
- `horizon-core/src/storage.rs::PackedStorage`: API generic de SimpleBitStorage pentru consumerii care au nevoie de slot/word-run/unpack.
- `horizon-core/src/carving.rs::CarvingMask`: bitset contiguu pentru coloane de carving, nu padded SimpleBitStorage.
- Aceste tipuri au roluri și contracte diferite; nu sunt dubluri confirmate. Un candidat viitor de minimizare este verificarea dacă partea de low-level read/write din PalettedContainer numeric poate delega mai mult către PackedStorage fără a pierde fast paths sau exactitatea layout-ului. Nu modifica până nu există măsurători și parity tests.
