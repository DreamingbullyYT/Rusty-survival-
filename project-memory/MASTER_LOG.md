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

## 2026-11-14 — Obiectivul de paritate și regulile canonice

- Utilizatorul a stabilit cerința suplimentară: codul să fie cât mai mic posibil, dar să păstreze paritate 1:1 cu Minecraft: Java Edition.
- A fost fixat ca baseline inițial Java Edition 26.3, versiunea menționată explicit de documentația upstream `docs/layers.md` la commit-ul de referință planificat.
- Paritatea înseamnă comportament observabil vanilla, nu copierea codului Java linie cu linie și nu doar asemănare vizuală.
- Asset-urile Mojang vor fi reutilizate din surse legitime și numai în limitele permisiunilor/licenței; folosirea asset-urilor nu substituie logica și mecanicile jocului.
- Optimizările și reducerea codului sunt permise numai dacă nu schimbă comportamentul vanilla. Nu se acceptă code-golf, scurtături nesigure sau eliminarea validărilor necesare.
- A fost creat `PROJECT_GOAL_AND_RULES.md` ca document canonic; `project-memory/START_HERE.md` îl cere la începutul fiecărei sesiuni, iar `project-memory/DECISIONS.md` înregistrează decizia D-005.
- Aceste schimbări sunt pe branch-ul `project-rules/parity-goal`; trebuie integrate în `main` după rezolvarea importului upstream aflat în desfășurare. Nu se declară încă prezente pe `main`.


## 2026-11-15 — Politica local-only pentru fișierele Mojang

- Clarificată regula canonică: jar-urile Minecraft, asset-urile Mojang, sursele decompilate și resursele pregătite/exportate nu se comit în repository.
- Datele se folosesc local din instalarea legitimă a utilizatorului sau din altă sursă autorizată, de regulă sub .local/.
- Decizia este documentată ca D-006 și completează regula existentă privind respectarea licențelor și permisiunilor.
