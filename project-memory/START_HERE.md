# START HERE — Memoria proiectului

La începutul fiecărei sesiuni:

1. Citește acest fișier.
2. Citește integral `MASTER_LOG.md` și ultimele stări din `TASKS.md`, `ARCHITECTURE.md`, `SYSTEM_REGISTRY.md`, `DEPENDENCIES.md`, `DECISIONS.md`, `TEST_RESULTS.md` și `DO_NOT_DUPLICATE.md`.
3. Verifică repository-ul și commit-ul curent; documentația poate fi depășită.
4. Compară starea reală a codului cu memoria. Dacă diferă, investighează și consemnează diferența înainte de a modifica.
5. Nu presupune că o sarcină este făcută doar fiindcă apare ca planificată.

## Reguli care nu se negociază

- Lucrăm numai la engine-ul Horizon.
- Nu implementăm acum launcherul și nici propriul Modding API.
- Înainte să adaugi, modifici sau ștergi ceva, caută în tot repository-ul dacă există deja.
- Nicio funcționalitate nu trebuie să aibă două implementări canonice.
- Dublurile conceptuale contează chiar dacă au nume diferite sau sunt în fișiere diferite.
- Detectorul nu șterge și nu modifică automat codul.
- Nu elimina cod până când nu există dovezi, înțelegerea dependențelor, teste și o decizie documentată.
- `MASTER_LOG.md` este append-only. Adaugă numai la final. Nu rescrie și nu șterge niciodată intrări vechi.
- Actualizează fișierele de memorie afectate după fiecare activitate tehnică.
- Nu pretinde că ai modificat sau testat ceva dacă tool-ul nu a confirmat operația.

## Ordinea de lucru

Audit upstream → inventar sisteme → hartă dependențe → analiză duplicări → arhitectură țintă → refactor incremental → teste → stabilizare → abia apoi proiectarea Modding API-ului propriu.
