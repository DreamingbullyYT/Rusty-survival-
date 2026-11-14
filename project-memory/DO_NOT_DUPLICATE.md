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
