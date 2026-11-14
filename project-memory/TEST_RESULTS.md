# Test Results

## 2026-11-14 — Inițializare

- Repository-ul a fost inspectat prin GitHub API.
- S-a confirmat că vechiul repo conținea doar scheletul HNMod (manifest, registry, teste de bază și CI), nu sursa engine-ului Horizon.
- Nu au fost rulate teste Rust pentru engine: sursa engine-ului nu este încă prezentă.
- Detectorul Python a fost introdus cu teste unitare; rezultatele efective vor fi consemnate după rularea lui într-un mediu local/CI.

## 2026-11-14 — Detector inițial

- GitHub Actions run #1: SUCCESS — teste Python și rularea detectorului.
- GitHub Actions run #2: SUCCESS — după îmbunătățirea lexerului pentru literali Rust.
- Testele acoperă normalizarea identificatorilor/literalilor, shingle overlap, extragerea corpului funcției cu acolade în stringuri și comentarii bloc imbricate.
- Scanarea curentă nu poate valida duplicări din engine deoarece sursa Rust Horizon nu a fost încă importată.
- Limitare cunoscută: detectarea conceptuală depinde momentan de registrul de sisteme completat manual; urmează să fie extinsă după inventarierea arhitecturii și dependențelor.


## 2026-11-15 — Audit restrâns la engine

- Adăugate teste pentru citirea crate-urilor engine din tools/layers.json.
- Adăugată verificare că fișierele din crate-uri platform, testele standalone și exemplele nu intră în scope-ul implicit.
- Adăugată opțiunea explicită --scope all pentru scanarea largă.
- Actualizat raportul pentru a număra separat fișierele Rust și funcțiile.
- Status: teste adăugate, execuția CI încă neconfirmată.


## 2026-11-15 — Matrice de paritate

- Verificate în tree-ul upstream fixat 678 de harness-uri parity, 397 fișiere Java oracle și 115 fișiere de port map.
- Creată matricea high-level în project-memory/PARITY_MATRIX.md, fără duplicarea port ledger-ului și a statusului harness-urilor.
- Status: inventar documentar; niciun harness nu a fost executat de această activitate.
