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
