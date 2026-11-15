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


## D-007 — Resurse Mojang în CI numai pe runner efemer

- Decizie: testele de paritate Minecraft Java 26.3 pot pregăti resursele necesare din endpoint-urile oficiale Mojang într-un workflow GitHub Actions declanșat manual (workflow_dispatch), exclusiv în spațiul temporar al runnerului.
- Versiunea și hash-urile acceptate se citesc din research-lock.json; manifestul de versiune, client/server JAR, asset index, obiectele de asset și bibliotecile se verifică prin checksum înainte de folosire.
- Nu se comit resurse Mojang, nu se publică artifacte, nu se folosește cache Actions pentru ele și nu se salvează în release-uri. Curățarea spațiului temporar se face la finalul jobului; runnerul este efemer.
- Workflow-ul are permisiuni minime (contents: read) și nu are nevoie de secrete/tokenuri pentru descărcările publice Mojang. Nu se afișează conținutul fișierelor sau date private în loguri.
- Clarificare de securitate: într-un repository public, logurile Actions sunt publice; deci această soluție ține JAR-urile și asset-urile în afara repository-ului și artifactelor, dar nu face execuția sau metadatele logurilor secrete față de GitHub ori publicul care poate vedea logurile. Se raportează doar versiunea, rezultatele checksum și sumarul testelor.
- tools/lab.py actual importă clientul și bibliotecile din instalarea oficială locală și primește separat server JAR-ul; nu descarcă întregul set necesar pentru runner. Înainte de workflow, trebuie adăugat un mod de input explicit pentru fișierele/directoarele pregătite de CI sau un adapter subțire, păstrând comportamentul local actual și fără a duplica logica de verificare/parity.
- Nicio implementare de workflow sau testare CI privată nu este declarată finalizată prin această decizie; vezi docs/private-vanilla-26-3-parity.md.


## D-008 — Downloader CI folosește importatorul canonic horizon-prepare

- Decizie: workflow-ul CI pregătește un layout standard de launcher în RUNNER_TEMP, apoi folosește direct horizon-prepare --prepare cu --minecraft-dir și --server-jar. Nu se adaugă un al doilea importator și nu se modifică fluxul local.
- Downloader-ul Python citește versiunea și hash-urile numai din research-lock.json, ia URL-ul version JSON din manifestul global Mojang, verifică hash-ul version JSON, apoi verifică client/server JAR, artifactele bibliotecilor, asset index-ul și fiecare asset object.
- manifest_sha1 din lock identifică version JSON-ul fixat, nu manifestul global. Manifestul global este preluat prin HTTPS, iar version JSON-ul ales este verificat prin hash-ul fixat/acceptat.
- Workflow-ul rămâne manual-only; oferă suite smoke, all și named. Nu rulează automat la fiecare push.
- Workflow-ul nu poate fi declanșat manual până când fișierul lui există pe branch-ul default GitHub. Nu se face merge automat pentru a ocoli această limitare.
- Nu se folosesc Actions cache sau artifacts pentru resurse Mojang; fișierele sunt șterse din RUNNER_TEMP și .local la finalul jobului.
