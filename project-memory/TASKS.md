# Task Ledger

## Finalizate
- [x] Confirmat repository-ul de lucru și permisiunile de scriere.
- [x] Stabilit scope-ul: numai engine; Modding API-ul propriu este ulterior.
- [x] Înlocuit scheletul HNMod în branch-ul curent, păstrând istoricul Git.
- [x] Creată memoria permanentă și regula append-only pentru MASTER_LOG.
- [x] Creat detectorul read-only pentru similitudini exacte/structurale și suprapuneri declarate.
- [x] Adăugate teste și CI pentru detector; rulările inițiale au trecut.
- [x] Fixat commit-ul upstream de referință: bcb7258ad101638c57e8cdd7b1e3da406440bea8.
- [x] Inspectate manifestele celor 19 crate-uri clasificate upstream ca engine.
- [x] Creat inventarul provizoriu de crate-uri și harta dependențelor directe declarate în Cargo.toml.
- [x] Citită harta upstream AGENTS.md pentru ownership-ul principal al modulelor.
- [x] Creată matricea high-level de paritate, reutilizând harness-urile și port ledger-ul upstream.
- [x] Restrâns detectorul implicit la sursele de producție din crate-urile engine și adăugate teste pentru filtrare.

## În curs
- [ ] Importul sursei upstream în main; ultima rulare trebuie verificată înainte de a declara importul reușit.
- [ ] Integrarea regulilor canonice din branch-ul project-rules/parity-goal.
- [ ] Integrarea schimbărilor din branch-ul audit/engine-only-scope și rularea CI pentru testele noi.
- [ ] Inventarierea semantică a sistemelor, callsites și ownership-ului real.
- [ ] Confirmarea grafului complet de dependențe Cargo și a excepțiilor între layere.
- [ ] Enumerarea statusului real pentru harness-uri și a gap-urilor din port ledger.
- [ ] Auditul duplicărilor exacte, structurale și conceptuale doar în codul vanilla relevant.

## Mai târziu
- [ ] Arhitectură țintă documentată după audit.
- [ ] Refactor incremental cu teste de regresie.
- [ ] Stabilizarea engine-ului și verificarea parității 1:1.
- [ ] Proiectarea propriului Modding API — numai după stabilizare.
