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

## 2026-11-14 — Import upstream

- Baseline upstream fixat la `bcb7258ad101638c57e8cdd7b1e3da406440bea8`.
- Înainte de import, tree audit a raportat 7,052 intrări și 3,588 fișiere Rust.
- Testele detectorului și scanarea read-only sunt executate în workflow-ul de import înainte de commit.
- Rezultatele scanării vor fi consemnate după finalizarea jobului.
