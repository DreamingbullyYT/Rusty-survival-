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
