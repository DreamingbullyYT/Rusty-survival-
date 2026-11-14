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

## D-005 — Paritate 1:1 cu Minecraft Java Edition și implementare minimă

- Decizie: obiectivul final este paritate comportamentală 1:1 cu Minecraft: Java Edition; versiunea inițială de referință este 26.3, conform documentației upstream fixate.
- Decizie: implementarea trebuie să fie cât mai mică posibil, fără a sacrifica paritatea, corectitudinea, mentenabilitatea sau performanța. Optimizările nu pot schimba comportamentul vanilla observabil.
- Decizie: folosim asset-uri Mojang din surse legitime și în limitele permisiunilor/licenței; asset-urile nu substituie mecanicile și logica jocului.
- Motiv: utilizatorul cere un engine compact care să reproducă Minecraft Java, nu o implementare aproximativă sau un simplu joc cu aspect similar.
- Detalii normative: `PROJECT_GOAL_AND_RULES.md`.


## D-006 — Mojang game files remain local

- Decizie: nu comitem în repository jar-uri Minecraft, assets Mojang, surse decompilate sau resurse pregătite/exportate din joc. Le folosim local din instalarea legitimă a utilizatorului sau din altă sursă autorizată, în mod normal sub .local/.
- Motiv: folosirea asset-urilor pentru referință și teste nu implică drept de redistribuire. Regula upstream AGENTS.md interzice explicit comiterea fișierelor Mojang.
- Excepție: numai o permisiune/licență aplicabilă, verificată explicit, poate permite redistribuirea unui fișier anume.
