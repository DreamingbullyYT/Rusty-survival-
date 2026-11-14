# Task Ledger

## Finalizate
- [x] Confirmat repository-ul de lucru și scope-ul: numai engine-ul vanilla Horizon.
- [x] Fixat baseline-ul upstream la `horizon-rust/horizon` commit `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.
- [x] Importat snapshot-ul upstream în `main` prin GitHub Actions run #8; commit rezultat `e02bf6f8e3b3f2a8725fbd3975c92a192b2456cf`.
- [x] Rulate testele de import: 967 teste OK, 9 skipped.
- [x] Identificate și păstrate instrumentele upstream canonice: `tools/layers.json`, `tools/check_layers.py`, parity harnesses și port ledger.
- [x] Creat inventarul provizoriu al celor 19 crate-uri etichetate upstream ca engine.
- [x] Creată harta dependențelor directe Cargo și matricea de domenii vanilla 26.3.
- [x] Pregătită versiunea detectorului limitată implicit la sursele de producție ale crate-urilor engine, cu opțiunea explicită `--scope all`.
- [x] Pregătite teste pentru filtrarea scope-ului engine și excluderea testelor standalone/examples/benches/platform.
- [x] Integrate în acest branch regulile canonice, deciziile D-005/D-006 și documentele de audit.

## În curs
- [ ] Rularea CI pe acest branch/PR pentru detectorul engine-only și testele sale.
- [ ] Verificarea raportului engine-only după ce CI rulează; candidații de similitudine necesită triere manuală.
- [ ] Inventarierea semantică a sistemelor din crate-urile mixte, cu module, callsites, dataflow și teste.
- [ ] Confirmarea grafului complet de dependențe Cargo, inclusiv dependențe transitive și feature gates.
- [ ] Extracția statusului real al harness-urilor și gap-urilor din port ledger; acestea nu au fost încă rulate în acest audit.

## Mai târziu
- [ ] Definirea arhitecturii țintă minimală pe baza inventarului semantic.
- [ ] Refactor incremental, fiecare schimbare legată de cazuri de paritate și teste de regresie.
- [ ] Stabilizarea și verificarea parității 1:1 pentru toate domeniile vanilla.
- [ ] Proiectarea propriului Modding API — numai după stabilizarea engine-ului.
