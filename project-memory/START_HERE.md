# START HERE — Memoria proiectului

La începutul fiecărei sesiuni:

1. Citește integral `PROJECT_GOAL_AND_RULES.md` — acesta este documentul canonic pentru obiectiv și regulile care nu se negociază.
2. Citește acest fișier.
3. Citește integral `MASTER_LOG.md` și stările curente din `TASKS.md`, `ARCHITECTURE.md`, `SYSTEM_REGISTRY.md`, `DEPENDENCIES.md`, `PARITY_MATRIX.md`, `DECISIONS.md`, `TEST_RESULTS.md` și `DO_NOT_DUPLICATE.md`. Pentru detectorul conceptual, verifică și `system_registry.toml`.
4. Verifică repository-ul, branch-ul și commit-ul curent; documentația poate fi depășită.
5. Compară memoria cu starea reală a codului. Dacă diferă, investighează și consemnează diferența înainte de a modifica.
6. Nu presupune că o sarcină este făcută doar fiindcă apare ca planificată.

## Reguli care nu se negociază

- Lucrăm numai la refactorizarea vanilla engine-ului Horizon.
- Ținta este paritate comportamentală 1:1 cu Minecraft: Java Edition; baseline-ul inițial din upstream este Java Edition 26.3.
- Codul trebuie să fie cât mai mic posibil, dar niciodată prin sacrificarea parității, corectitudinii, mentenabilității sau performanței.
- Refolosim asset-urile Mojang din surse legitime și numai în limitele permisiunilor/licenței aplicabile; asset-urile nu înlocuiesc logica jocului.
- Nu implementăm acum launcherul, funcționalități Android/platform/Horizon extras sau propriul Modding API. API-ul de modding vine numai după stabilizarea engine-ului.
- Înainte să adaugi, modifici, muți sau ștergi ceva, caută în tot repository-ul dacă există deja.
- O singură implementare canonică pentru fiecare responsabilitate; dublurile conceptuale contează chiar dacă numele diferă.
- Detectorul raportează candidați; nu șterge și nu modifică automat codul.
- Nu elimina cod fără dovezi, verificarea dependențelor și utilizărilor, teste și decizie documentată.
- `MASTER_LOG.md` este append-only. Adaugă numai la final. Nu rescrie și nu șterge niciodată intrări vechi.
- Actualizează fișierele de memorie afectate după fiecare activitate tehnică.
- Nu pretinde că ai modificat sau testat ceva dacă tool-ul nu a confirmat operația.

## Ordinea de lucru

Verificare upstream/import → inventar sisteme → hartă dependențe și ownership → matrice de paritate Java Edition → analiză duplicări → arhitectură țintă → refactor incremental → teste de paritate/regresie → stabilizare → abia apoi proiectarea Modding API-ului propriu.
