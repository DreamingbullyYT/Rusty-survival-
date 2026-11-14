# Decision Log

## D-001 — Scope-ul proiectului

- Decizie: repository-ul este dedicat exclusiv refacerii engine-ului Horizon.
- În afara scope-ului curent: launcher și Modding API propriu.
- Motiv: Modding API-ul trebuie proiectat pe arhitectura finală, nu pe o bază instabilă.

## D-002 — Memorie append-only

- Decizie: `project-memory/MASTER_LOG.md` nu se rescrie și nu se curăță niciodată; se adaugă numai înregistrări noi.
- Motiv: istoricul deciziilor și activităților trebuie să rămână recuperabil.

## D-003 — Anti-duplicare înainte de implementare

- Decizie: fiecare schimbare începe cu căutarea implementărilor și responsabilităților existente.
- Motiv: prevenirea duplicării exacte, structurale și conceptuale.

## D-004 — Detector read-only

- Decizie: detectorul raportează candidați; nu șterge, nu mută și nu rescrie cod.
- Motiv: similitudinea nu dovedește automat duplicarea semantică.
