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


## 2026-11-15 — Planificare resurse Mojang efemere pentru parity CI

- Au fost inspectate research-lock.json și fluxul tools/lab.py.
- Lock-ul fixează Minecraft Java Edition 26.3, SHA-1 pentru client și server JAR, hash-uri acceptate pentru manifest/asset index și Java major 25.
- tools/lab.py folosește în prezent instalarea locală a launcherului pentru manifest, client JAR, biblioteci și asset store; server JAR-ul este furnizat separat. Nu există încă un mod de input complet pentru resurse descărcate pe runner.
- Workflow-ul privat/efemer este doar documentat și planificat; nu a fost creat, declanșat sau testat. Nu există rezultate Java↔Rust noi și nu se declară paritate.


## 2026-11-15 — Implementare downloader și workflow privat de paritate

- Inspecția a confirmat că horizon-prepare are deja inputuri explicite pentru version JSON, client JAR, biblioteci, assets și server JAR; importul său scrie .local/minecraft/<version>/prepared.json și inventarele necesare harness-urilor.
- Adăugate tools/ci/vanilla_data.py și teste unitare pentru căi sigure, verificarea lock-ului și descărcări atomice cu SHA-1.
- Adăugat workflow manual .github/workflows/private-vanilla-parity.yml, cu JDK 25, import prin horizon-prepare, copiere sounds/languages și suite smoke/all/named.
- Nu există încă execuție GitHub Actions pentru acest workflow: branch-ul este PR #1, iar GitHub permite dispatch manual numai după ce workflow-ul există pe branch-ul default. Nu s-au descărcat resurse Mojang și nu s-au rulat teste Java↔Rust în această sesiune.
- Testele unitare ale downloader-ului sunt conectate la workflow-ul Engine Audit Tooling, astfel încât să poată rula pe PR fără a descărca resurse Mojang. Nu au fost încă confirmate ca trecute; se așteaptă rezultatul check-ului PR.


## 2026-11-15 — CI run #32: downloader unit tests

- GitHub Actions run 43700810845, commit dcf5300497ea816f91c1c11d032ea52c62fa822d.
- Step `Run vanilla downloader unit tests`: completed SUCCESS. The four unit tests for safe paths, version-lock validation, verified atomic download/reuse and cleanup after checksum mismatch passed.
- The subsequent `Run read-only engine audit` step was still in progress at the time of this entry. The manual Mojang download/preparation/parity workflow was not triggered.
