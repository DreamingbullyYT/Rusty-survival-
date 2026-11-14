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


## 2026-11-15 — Inventar preliminar și restrângerea auditului la engine

- În manifestele upstream de la commitul \`bcb7258ad101638c57e8cdd7b1e3da406440bea8\` au fost inspectate descrierile și dependențele locale directe pentru cele 19 crate-uri clasificate ca engine.
- A fost completat un inventar provizoriu în \`SYSTEM_REGISTRY.md\`. Acesta nu declară ownership final: crate-urile mixte și responsabilitățile interne rămân de verificat în sursă.
- A fost adăugată o hartă inițială a dependențelor directe declarate în Cargo.toml în \`DEPENDENCIES.md\`; nu este încă un graf transitive complet Cargo.
- Detectorul \`tools/engine_audit.py\` a fost restrâns implicit la fișierele de producție din \`src/\` și \`build.rs\` ale crate-urilor engine, pe baza \`tools/layers.json\`. Opțiunea \`--scope all\` păstrează scanarea largă la cerere.
- Au fost adăugate teste pentru selectarea crate-urilor engine și excluderea testelor standalone, exemplelor, benchmark-urilor și fixture-urilor. Testele nu sunt declarate trecute până când CI nu confirmă rularea.
- Raportul detectorului include acum numărul de fișiere Rust și numărul de funcții scanate.
- Schimbările sunt pe branch-ul \`audit/engine-only-scope\`; nu au fost integrate în \`main\` și nu au modificat sursa engine-ului.


## 2026-11-15 — Confirmarea hărții de module upstream

- AGENTS.md din commitul upstream fixat documentează ownership-ul principal pentru gameplay, rendering, world, commands, input, protocol și sistemele client.
- A fost adăugată în SYSTEM_REGISTRY.md o hartă preliminară de module, fără a declara ownership final pentru responsabilități care necesită verificarea callsite-urilor.
- A fost clarificat că horizon-client-lib poate folosi surse aflate fizic în crates/horizon-client; existența a două crate-uri nu demonstrează două implementări.
- S-a notat separarea dintre command parsing (horizon-command) și dispatch/efecte (horizon-client), precum și dintre world/server-side logic (horizon-world) și gameplay loop (horizon-client).
