# Task Ledger

## Finalizate
- [x] Confirmat repository-ul și permisiunile de scriere.
- [x] Stabilit scope-ul: numai engine; Modding API-ul propriu este ulterior.
- [x] Înlocuit scheletul HNMod în branch-ul curent, păstrând istoricul Git.
- [x] Creată memoria permanentă și regula append-only pentru MASTER_LOG.
- [x] Creat detectorul read-only pentru similitudini exacte/structurale și suprapuneri declarate.
- [x] Adăugate teste și CI; rulările detectorului au trecut.
- [x] Fixat commit-ul upstream și importat sursa Horizon.
- [x] Identificat și păstrat tooling-ul upstream existent pentru verificarea straturilor.

## În curs
- [ ] Auditul real al celor 19 crate-uri clasificate upstream ca engine.
- [ ] Cartografierea funcționalităților care sunt încă amestecate între engine și platform.
- [ ] Construirea registrului complet de sisteme și a hărții dependențelor.
- [ ] Analiza raportului detectorului și confirmarea manuală a dublurilor.

## Mai târziu
- [ ] Definirea arhitecturii țintă pe baza auditului.
- [ ] Refactor incremental, cu teste de regresie.
- [ ] Stabilizarea engine-ului.
- [ ] Proiectarea propriului Modding API — numai după stabilizare.
