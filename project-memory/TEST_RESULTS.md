# Test Results

## 2026-11-14 — Inițializare

- Repository-ul a fost inspectat prin GitHub API.
- S-a confirmat că vechiul repo conținea doar scheletul HNMod (manifest, registry, teste de bază și CI), nu sursa engine-ului Horizon.
- Nu au fost rulate teste Rust pentru engine: sursa engine-ului nu este încă prezentă.
- Detectorul Python a fost introdus cu teste unitare; rezultatele efective vor fi consemnate după rularea lui într-un mediu local/CI.

## 2026-11-14 — Detector inițial

- GitHub Actions run #1: SUCCESS — teste Python și rularea detectorului.
- GitHub Actions run #2: SUCCESS — după îmbunătățirea lexerului pentru literali Rust.
- Testele acoperă normalizarea identificatorilor/literalilor, shingle overlap, extragerea corpului funcției cu acolade în stringuri și comentarii bloc imbricate.
- Scanarea curentă nu poate valida duplicări din engine deoarece sursa Rust Horizon nu a fost încă importată.
- Limitare cunoscută: detectarea conceptuală depinde momentan de registrul de sisteme completat manual; urmează să fie extinsă după inventarierea arhitecturii și dependențelor.

## 2026-11-14 — Import upstream

- Baseline upstream fixat la `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.
- Înainte de import, tree audit a raportat 7,052 intrări și 3,588 fișiere Rust.
- Testele detectorului și scanarea read-only sunt executate în workflow-ul de import înainte de commit.
- Rezultatele scanării vor fi consemnate după finalizarea jobului.

## 2026-11-15 — Import upstream confirmat și audit inițial

- GitHub Actions import run #8: SUCCESS; commitul rezultat pe `main` este `e02bf6f8e3b3f2a8725fbd3975c92a192b2456cf`.
- Logul jobului confirmă `git push origin HEAD:main` și commitul `Import Horizon upstream engine baseline bcb7258`.
- `python -m unittest discover -s tests -p "test_*.py"`: 967 teste OK, 9 skipped.
- Scanarea inițială efectuată de detectorul vechi a inclus toate sursele Rust: 41.988 funcții, 835 grupuri exacte și 11.343 candidați structurali. Acestea sunt rezultate euristice dintr-un scope prea larg pentru deciziile de refactor; nu sunt dubluri confirmate și nu s-a șters/modificat cod pe baza lor.
- Detectorul nou din acest branch restrânge scope-ul implicit la sursele de producție ale crate-urilor clasificate upstream ca engine; scanarea largă rămâne disponibilă explicit prin `--scope all`.
- Testele specifice noului scope au fost adăugate, dar nu sunt încă validate de CI pe acest branch.
- Nu au fost executate harness-urile Java↔Rust și nu se declară nicio paritate completă.

## 2026-11-15 — Prima rulare CI a PR-ului de integrare

- PR #1, GitHub Actions run `43652682189`: FAILURE în etapa de testare generală.
- Rezultat: 922 teste rulate, 9 skipped, 12 erori și 1 failure. Erorile sunt teste upstream care citesc workflow-uri și template-uri `.github` eliminate intenționat la import pentru a păstra numai workflow-urile proiectului nostru; un test verifică și existența a cel puțin 10 runner jobs upstream.
- Eșecurile nu indică o problemă în logica detectorului; totuși, rularea generală nu poate fi folosită ca validare a detectorului.
- Ajustare: workflow-ul `engine-audit.yml` rulează acum numai `tests/test_engine_audit.py`, apoi execută auditul read-only cu `--scope engine`. Testele complete upstream au fost deja rulate în workflow-ul de import înainte de eliminarea workflow-urilor upstream.
- Rezultatul noii rulări CI rămâne în așteptare; nu declarăm încă detectorul validat.

## 2026-11-15 — Audit engine-only confirmat în CI

- PR #1, GitHub Actions run #26, run ID `43652931734`, job `audit-tool`, job ID `132625133568`: SUCCESS.
- Logul jobului confirmă `python -m unittest discover -s tests -p "test_engine_audit.py"`: 7 teste rulate, OK.
- Scanarea read-only `python tools/engine_audit.py --scope engine` a terminat cu SUCCESS: 2.292 fișiere Rust și 34.788 funcții scanate; 555 grupuri exacte, 4.415 candidați structurali, 0 candidați conceptuali.
- Raportul a fost scris în workspace-ul runnerului la `reports/engine-audit.json` și `reports/engine-audit.md`, dar run-ul nu a publicat artifacts; raportul detaliat nu este disponibil din GitHub Actions după încheierea jobului.
- Aceste rezultate sunt candidați de analizat, nu dubluri semantice confirmate. Nicio implementare engine nu a fost ștearsă sau modificată.
- Auditul semantic din această sesiune a inspectat prima trecere a `horizon-core` pentru version/RNG/Math și a identificat harness-uri existente pentru worldgen RNG, random ticks, fishing RNG, float log, noise și geometry. Niciun harness Java↔Rust și niciun test Cargo nu a fost executat în această sesiune.
- PR #1 rămâne deschis; main nu a fost modificat prin această activitate.


## 2026-11-15 — Worldgen source audit (fără execuție)

- Inspectate callsites și ownership în `generator.rs`, `terrain.rs`, `generator_structures.rs`, `generator_decoration.rs`, `structure/start.rs`, `density.rs`, `feature_catalog.rs` și `feature_graph.rs`.
- Confirmată existența pipeline-ului BIOMES/TERRAIN, structure starts/references, jigsaw assembly, structure piece placement și FEATURES 3×3 decoration. Unele structure kinds/placements și placed-feature roots sunt explicit unsupported.
- Density decoder-ul acceptă o familie largă de funcții; limitările observate sunt DAG-only, nesting 256, interval branches 256 și no inline noise holders.
- Inspectate descrierile harness-urilor whole-chunk, terrain, structure-layout și structure-block. Niciunul nu a fost rulat în această sesiune; niciun rezultat nou de paritate nu este raportat.
- Corectată documentația pentru a nu descrie structures ca sistem absent. Nicio modificare a engine-ului și nicio concluzie de paritate completă.


## 2026-11-15 — Feature codec source audit (fără execuție)

- Inspectate match arms și tipurile pentru configured feature graph, placement modifiers și block-state providers.
- Confirmată suprafața largă de tipuri implementate și separarea responsabilităților; decoder-ele au limite explicite pentru depth/node count/list sizes.
- Nu au fost încărcate bundle-uri vanilla 26.3 și nu s-au generat rapoarte concrete de unsupported roots; nu s-au rulat parity harnesses.
- Nicio modificare de engine.
