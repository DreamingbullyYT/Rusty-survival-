# Private Vanilla 26.3 Parity — GitHub Actions

## Scop

Pregătim o cale repetabilă de a rula oracle-urile Java și harness-urile Rust↔Java pentru Minecraft: Java Edition 26.3 fără a introduce JAR-uri, asset-uri sau resurse Mojang în repository. Acest document este planul canonic pentru implementarea CI; workflow-ul nu este încă implementat sau executat.

## Modelul de execuție

- Workflow separat, declanșat manual prin workflow_dispatch; nu descărcăm resursele la fiecare CI obișnuit.
- Runner GitHub-hosted efemer, Linux, cu JDK 25 și Python/Rust potrivite proiectului.
- Permisiuni minime: contents: read. Descărcările Mojang sunt publice, deci nu este necesar un secret sau token personal.
- Toate fișierele Mojang și toate fișierele generate rămân în spațiul temporar al runnerului (sub .local/ sau un director temporar echivalent).
- Nu folosim actions/upload-artifact, cache Actions, release assets, commit sau push pentru JAR-uri, assets, surse decompilate ori resurse pregătite.
- La final, se șterge explicit directorul temporar. Runnerul GitHub-hosted este oricum distrus după job.

## Descărcare și verificare

1. Citește research-lock.json; nu duplica versiunea sau hash-urile în YAML.
2. Descarcă manifestul oficial Mojang de la piston-meta.mojang.com și verifică SHA-1 față de manifest_sha1 sau lista accepted_manifest_sha1.
3. Rezolvă versiunea fixată din manifest și verifică id == 26.3; downloads.client.sha1 == minecraft.client_sha1; downloads.server.sha1 == minecraft.server_sha1; versiunea majoră Java corespunde cu minecraft.java_major (25).
4. Descarcă client și server JAR din URL-urile oficiale indicate de version JSON și verifică SHA-1 înainte de folosire.
5. Descarcă asset index-ul indicat de manifest și verifică hash-ul lui față de hash-ul din manifest / lista acceptată din lock. Descarcă numai obiectele necesare testelor (sau setul complet dacă harness-urile demonstrează că este necesar) din infrastructura oficială Mojang și verifică SHA-1 pentru fiecare obiect.
6. Descarcă bibliotecile necesare din URL-urile oficiale din manifest și verifică hash-urile declarate. Aplică regulile de platformă/architecture din manifest pentru runnerul Linux.
7. Orice mismatch de versiune, URL lipsă, fișier incomplet sau checksum greșit oprește jobul. Nu accepta automat hash-uri noi: actualizarea lock-ului cere verificare și documentarea motivului.

## Integrarea cu tooling-ul existent

tools/lab.py este reutilizarea preferată pentru pregătire, inventariere și import. La momentul documentării, acesta caută version JSON și client JAR în instalarea oficială locală, caută bibliotecile și asset store în directorul launcherului și primește separat server JAR prin argument sau HORIZON_SERVER_JAR; verifică fișierele importate față de lock.

Prin urmare, înainte de activarea workflow-ului, extinde controlat interfața de input pentru a accepta explicit manifest/client/server/libraries/assets pregătite de CI, ori creează un adapter foarte subțire către layout-ul pe care lab.py îl consumă. Păstrează fluxul existent pentru utilizatorii cu launcher local. Nu crea un al doilea downloader, lock sau parity runner și nu schimba semantica verificărilor.

După pregătire, folosește infrastructura existentă: tools/parity.py --list și --status; docs/port-ledger.md și tools/port_ledger.py; tools/test_*_parity.py și oracle-urile Java existente; docs/known-issues.md.

Selectează testele pe baza dependențelor reale de resurse. Nu pretinde că toate cele 678 harness-uri au rulat doar pentru că un subset a trecut. Nu relaxa sau dezactiva teste pentru a obține un rezultat verde.

## Loguri și confidențialitate — limită importantă

Repository-ul este public. Așadar, logurile GitHub Actions sunt vizibile publicului. Acest mecanism nu face execuția secretă față de GitHub și nu ascunde metadatele din loguri; el împiedică păstrarea și distribuirea JAR-urilor/assets prin Git, artifacts, cache sau releases.

Logurile trebuie să conțină numai versiunea țintă, numele fișierului și rezultatul verificării checksum (fără conținutul fișierului), numele harness-urilor, rezultat PASS/FAIL și numărul de teste, plus mesaje de eroare curățate.

Nu afișa tokenuri, secrete, conținut binar, fișiere întregi sau dump-uri de resurse. Nu activa debug logging pentru descărcări. Nu folosi secrete pentru URL-urile publice Mojang.

## Criterii de acceptare

- [ ] Workflow manual, separat de CI-ul obișnuit.
- [ ] JDK 25 și versiunea 26.3 sunt verificate.
- [ ] Manifest, client/server JAR, asset index, obiecte și biblioteci sunt verificate criptografic din lock/manifest.
- [ ] lab.py poate consuma inputurile runnerului fără a strica fluxul local.
- [ ] Harness-urile selectate sunt alese explicit și rezultatele reale sunt raportate.
- [ ] Nu există upload-artifact, cache sau commit de resurse Mojang.
- [ ] Directorul temporar este curățat la final, inclusiv la eșec.
- [ ] Logurile nu expun conținutul resurselor.
- [ ] Testarea este efectiv rulată; doar atunci se actualizează TEST_RESULTS.md cu rezultate concrete.

## Stare actuală

Documentat, neimplementat. Următorul pas este inspectarea completă a CLI-ului lab.py, a harness-urilor care necesită assets/libraries și a structurii Java oracle; apoi implementarea minimă și testarea workflow-ului pe branch separat/PR. Nicio resursă Mojang nu a fost descărcată în acest workflow, pentru că workflow-ul nu există încă.
