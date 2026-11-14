# Horizon Engine Refactor

Acest repository este spațiul de lucru pentru auditarea și refacerea engine-ului Horizon.

**Scope:** numai engine-ul. Launcherul și Modding API-ul propriu sunt în afara etapei curente. Modding API-ul va fi proiectat după ce arhitectura engine-ului este stabilizată.

## Reguli obligatorii

1. Înainte de orice modificare, căutăm dacă funcționalitatea există deja.
2. Nu adăugăm o a doua implementare pentru o responsabilitate existentă.
3. Nu duplicăm concepte doar pentru că au nume sau forme diferite.
4. Nicio ștergere sau rescriere automată a codului detectat ca duplicat.
5. După fiecare activitate, actualizăm memoria proiectului.
6. `project-memory/MASTER_LOG.md` este append-only: numai intrări noi la final; niciodată nu ștergem sau rescriem istoricul.
7. Orice refactor trebuie justificat, testat și consemnat.

## Memoria proiectului

Citește mai întâi `project-memory/START_HERE.md` la fiecare sesiune nouă.

## Audit

Detectorul inițial este `tools/engine_audit.py`. El caută similitudini de cod și suprapuneri declarate în registrul de sisteme. Rezultatele sunt indicii pentru analiză umană, nu autorizații de ștergere.

## Sursa upstream

Repository upstream: https://github.com/horizon-rust/horizon  
Branch de referință: `main`

Sursa engine-ului nu a fost încă importată în acest repository. Primul pas tehnic după fundația de memorie este inventarierea și importul controlat al sursei upstream, cu commit-ul exact consemnat.
