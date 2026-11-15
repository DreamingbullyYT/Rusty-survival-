# Do Not Duplicate

## Regula

Nu crea un sistem nou înainte să verifici dacă aceeași responsabilitate există deja, indiferent de numele fișierelor, crate-urilor sau funcțiilor.

## Intrări confirmate

Momentan nu există dubluri confirmate în engine, deoarece sursa engine-ului nu a fost încă importată.

## Candidați

Se adaugă aici numai după audit, cu:
- responsabilitatea comună;
- locațiile implicate;
- dovezile;
- owner-ul canonic ales;
- decizia și planul de migrare.

## Existing upstream architecture tooling to reuse

- `tools/layers.json` — canonical crate-layer and dependency policy.
- `tools/check_layers.py` — existing layer/dependency checker.
- `docs/layers.md` — definitions and intended boundaries for engine/platform/Horizon.
Do not implement a competing layer registry or dependency-layer checker. Extend the existing one only if a demonstrated gap requires it.

## Rezultate detector — fără concluzii automate

Scanarea inițială de import a fost prea largă (toate sursele Rust), a raportat candidați exacți/structurali și nu a confirmat semantic nicio dublură. Nu elimina și nu modifica implementări pe baza acelor numere. Versiunea engine-only trebuie să ruleze în CI, iar fiecare candidat relevant trebuie verificat manual prin ownership, callsites, dataflow și teste.

## Surse partajate care nu trebuie confundate cu duplicări

- `horizon-client-lib` declară în manifest `[lib] path = "../horizon-client/src/client_lib.rs"`; este aceeași cale de sursă expusă ca target Cargo separat, nu două implementări independente.
- Parsing-ul comenzilor (`horizon-command`) și dispatch-ul/efectele comenzilor (`horizon-client/game`) sunt responsabilități distincte.
- Rendererul care citește world state nu devine automat owner al regulilor de gameplay.


## Trierea detectorului pe `reports/upstream-bcb7258-audit.json` (2026-11-15)

Raport complet: `reports/engine-audit-2026-11-15/dups.md`. Nimic nu a fost șters sau modificat. „Exact” înseamnă aceeași formă de tokeni cu identificatorii și literalele abstractizate, nu același text; JSON-ul este scanare all-scope (include teste și exemple).

| Clasă | Grupuri | Linii duplicate |
|---|---:|---:|
| A: doar teste | 363 | 6.117 |
| B: banale (≤3 instrucțiuni) | 431 | 4.479 |
| C: între crate-uri, netriviale | 4 | 66 |
| D: același crate, netriviale | 20 | 296 |
| E: generat, protocol sau tabele | 17 | 307 |

Cele 24 de grupuri C și D au fost citite integral: 15 REAL, 4 UNCLEAR, 5 COINCIDENTAL. Economie netă estimată pentru REAL: ~90–120 linii de producție (estimare, nu măsurătoare). Candidații structurali (11.343) sunt în mare parte tabele enum-spre-șir și nu sunt dubluri reale.

Nu se unifică fără rularea harness-ului pe oracle-ul 26.3:
- senzorii `sense_players`, `sense_hurt`, `sense_living` (mobs): în `villager_brain` limita `FOLLOW_RANGE` diferă;
- `place_configured_feature` și `place_placed_feature`, `rotate_left` și `rotate_right` din `java_hash_map.rs`, `corridor_left` și `corridor_right`, `draw_overlay_message` și `draw_title`: oglindesc metode vanilla diferite;
- `introduce` și `code_hello` (horizon-friends): handshake-uri cu MAGIC și timeout-uri diferite;
- `apply` și `apply_secret` din `rsa.rs`: varianta secretă este apărare intenționată împotriva side-channel.

Pentru un PR viitor: dublura din `realms.rs` (`closest_expiration` copiat în `mod tests`) face ca testul să nu exerseze funcția livrată; se poate înlocui cu apel la funcția reală păstrând aserțiunile.
