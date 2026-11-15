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


## 2026-11-15 — Decizie pentru resurse Mojang efemere în GitHub Actions

- A fost consemnat planul de testare Java↔Rust Minecraft 26.3 într-un workflow manual, cu descărcare din endpoint-urile oficiale Mojang și verificare strictă prin research-lock.json.
- Resursele vor exista numai în spațiul temporar al runnerului; fără commit, artifact, cache Actions sau release. Se vor folosi permisiuni minime și loguri limitate la checksum-uri și sumarul testelor.
- S-a documentat explicit că logurile unui repository public rămân publice; efemer înseamnă că fișierele nu sunt păstrate/distribuite de repo, nu că GitHub sau vizitatorii logurilor nu pot vedea metadatele execuției.
- Inspecția tools/lab.py a arătat că importul actual se bazează pe instalarea oficială locală și pe un server JAR furnizat separat. Este necesară adaptarea inputurilor sau un adapter minimal înainte de a putea folosi același flux pe runner.
- Documentația dedicată: docs/private-vanilla-26-3-parity.md; decizia D-007; task-ul rămâne în curs.
- Doar documentare și planificare: workflow-ul nu a fost încă implementat/rulat; nu s-au executat teste de paritate noi.


## 2026-11-15 — Implementare downloader Mojang efemer și workflow manual

- După inspecția completă a CLI-ului horizon-prepare și a preparării upstream, s-a confirmat că importatorul Rust existent acceptă layout-ul launcherului și nu necesită modificări sau un al doilea importator.
- Adăugate tools/ci/vanilla_data.py pentru descărcarea oficială a version JSON, client/server JAR, artifactelor de bibliotecă, asset index-ului și tuturor obiectelor de asset; fiecare fișier este verificat prin SHA-1 și, când manifestul declară, dimensiune.
- Adăugate teste unitare pentru validarea căilor, pin-urilor și descărcării atomice. Aceste teste sunt definite, dar nu au fost încă executate.
- Adăugat .github/workflows/private-vanilla-parity.yml, manual-only, cu JDK 25, suite smoke/all/named, fără cache sau upload de artifacte, și cleanup la final.
- Corectată documentația: manifest_sha1 este hash-ul version JSON-ului 26.3, nu al manifestului global Mojang.
- Workflow-ul nu a fost executat: GitHub cere ca fișierul să fie prezent pe branch-ul default pentru workflow_dispatch; branch-ul curent este încă PR #1. Nicio resursă Mojang nu a fost descărcată și nu există rezultate de paritate noi.


## 2026-11-15 — Confirmare teste unitare downloader în CI

- Engine Audit Tooling run #32, commit dcf5300497ea816f91c1c11d032ea52c62fa822d: pasul Run vanilla downloader unit tests s-a încheiat cu SUCCESS; toate cele patru teste unitare au trecut.
- La momentul înregistrării, auditul read-only engine din același job era încă în desfășurare. Workflow-ul de descărcare Mojang/parity nu a fost declanșat.


## 2026-11-15 — Audit read-only al engine-ului pe baseline-ul bcb7258

- Patru subagenți read-only au analizat repository-ul fără să-l modifice: graful de dependențe, inventarul crate-urilor engine, starea parității din datele comise și trierea grupurilor exacte ale detectorului. Rapoartele sunt în `reports/engine-audit-2026-11-15/`.
- Verificate independent înainte de consemnare: `check_layers.py` trece (55 crate-uri, 26 excepții); `horizon-cosmetics` și `horizon-telemetry` sunt dependențe necondiționate ale `horizon-client`; `git status` a rămas curat după audit.
- Rezultate principale: engine-ul are 1.430.651 de linii Rust (887.804 în `src/` fără teste); ~33k linii de glue pentru mod API și ~6,4k de funcții Horizon în render sunt încă în crate-uri engine; 52 de harness-uri nu au oracle Java; port ledger-ul nu poate fi interogat fără `.local/`; din 24 de grupuri de duplicate netriviale, 15 sunt reale, 4 neclare, 5 coincidente.
- Observație: cele 835 de grupuri exacte sunt majoritar zgomot (363 doar în teste, 431 banale). Economia netă estimată din dublurile reale este mică (~90–120 de linii). Concluzie provizorie: o reducere semnificativă de dimensiune nu vine din aceste grupuri; rămâne de verificat în duplicarea structurală și în codul din afara fazei curente.
- Nu s-a rulat niciun build, test sau harness, nu s-au descărcat resurse Mojang și nu s-a modificat codul engine-ului. Nu există rezultate noi de paritate.
- Un agent a rulat `tools/parity.py --list/--status`, care a scris `.local/parity-runner/state.json`; directorul a fost șters imediat și `git status` a fost curat.
- Următorul pas propus: rularea manuală a workflow-ului Private Vanilla 26.3 Parity după merge, ca să existe o bază măsurată, apoi inventarul semantic pe modulele mari.
