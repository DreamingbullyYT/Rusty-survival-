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
- [x] Testele unitare ale downloader-ului au trecut în Engine Audit Tooling run #32.
- [ ] După integrarea în main, rularea manuală a suitei Private Vanilla 26.3 Parity și actualizarea TEST_RESULTS.md.
- [x] Implementat downloader-ul oficial Mojang, testele unitare și workflow-ul manual Private Vanilla 26.3 Parity; vezi docs/private-vanilla-26-3-parity.md. Rămâne de declanșat după ce workflow-ul ajunge pe main.
- [ ] Re-rularea CI pe PR după izolarea testelor detectorului de fixture-urile .github upstream.
- [ ] Verificarea raportului engine-only după ce CI rulează; candidații de similitudine necesită triere manuală.
  - Parțial (2026-11-15): cele 24 de grupuri exacte netriviale au fost triate manual (15 REAL, 4 UNCLEAR, 5 COINCIDENTAL) în `reports/engine-audit-2026-11-15/dups.md`; nimic nu a fost modificat.
- [ ] Inventarierea semantică a sistemelor din crate-urile mixte, cu module, callsites, dataflow și teste.
  - Parțial (2026-11-15): inventar cantitativ pe cele 19 crate-uri engine și lista nepotrivirilor doc/cod în `reports/engine-audit-2026-11-15/inventory.md`. Lipsesc callsites, dataflow și teste per modul.
- [ ] Confirmarea grafului complet de dependențe Cargo, inclusiv dependențe transitive și feature gates.
  - Parțial (2026-11-15): graful între crate-urile workspace, `check_layers.py` și versiunile externe multiple în `reports/engine-audit-2026-11-15/deps.md`. Dependențele externe tranzitive nu sunt confirmate (s-a folosit `--no-deps`).
- [ ] Extracția statusului real al harness-urilor și gap-urilor din port ledger; acestea nu au fost încă rulate în acest audit.
  - Parțial (2026-11-15): numărătorile din fișierele comise în `reports/engine-audit-2026-11-15/parity.md`. Gap-urile din port ledger rămân blocate: au nevoie de `.local/` (workflow-ul Private Vanilla 26.3 Parity).

## Candidați pentru PR-uri mici (după rularea parității pe CI)
- [ ] Înlocuirea copiei din `mod tests` din `realms.rs` cu apel la `closest_expiration` real (aserțiunile rămân).
- [ ] `from_server_toml` duplicat între `horizon-chat-filter` și `horizon-anticheat`, cu proprietar în `horizon-server-files`.
- [ ] `SplitMix64` repetat în trei locuri, cu proprietar în `horizon-core/src/random.rs` (verifică întâi `properties.rs`).
- [ ] Ierarhia dependențelor `horizon-cosmetics` și `horizon-telemetry` față de `docs/layers.md` (decizie, nu refactor).
- [ ] Rularea manuală a suitei Private Vanilla 26.3 Parity ca să existe prima bază de paritate măsurată.

## Mai târziu
- [ ] Definirea arhitecturii țintă minimală pe baza inventarului semantic.
- [ ] Refactor incremental, fiecare schimbare legată de cazuri de paritate și teste de regresie.
- [ ] Stabilizarea și verificarea parității 1:1 pentru toate domeniile vanilla.
- [ ] Proiectarea propriului Modding API — numai după stabilizarea engine-ului.
