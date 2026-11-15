# Private Vanilla 26.3 Parity — GitHub Actions

## Scop

Pregătim o cale repetabilă de a rula oracle-urile Java și harness-urile Rust↔Java pentru Minecraft: Java Edition 26.3 fără a introduce JAR-uri, asset-uri sau resurse Mojang în repository. Acest document este planul canonic pentru implementarea CI; workflow-ul este implementat pe branch-ul PR #1; testele unitare sunt adăugate și vor rula în CI-ul PR-ului.

## Implementare

- Workflow: .github/workflows/private-vanilla-parity.yml.
- Downloader: tools/ci/vanilla_data.py; teste unitare: tools/ci/test_vanilla_data.py.
- Suite: smoke (noise, geometry, terrain, chunk_generation), all sau named (nume separate prin virgulă).
- Import/preparare: reutilizează horizon-prepare --prepare, apoi copiază sounds și languages din asset store.
- Workflow-ul este workflow_dispatch numai. GitHub cere ca fișierul workflow să existe pe branch-ul default înainte ca dispatch-ul manual să fie disponibil; cât timp PR-ul nu este integrat în main, nu poate fi declanșat prin butonul Actions.

## Modelul de execuție

- Workflow separat, declanșat manual prin workflow_dispatch; nu descărcăm resursele la fiecare CI obișnuit.
- Runner GitHub-hosted efemer, Linux, cu JDK 25 și Python/Rust potrivite proiectului.
- Permisiuni minime: contents: read. Descărcările Mojang sunt publice, deci nu este necesar un secret sau token personal.
- Toate fișierele Mojang și toate fișierele generate rămân în spațiul temporar al runnerului (sub .local/ sau un director temporar echivalent).
- Nu folosim actions/upload-artifact, cache Actions, release assets, commit sau push pentru JAR-uri, assets, surse decompilate ori resurse pregătite.
- La final, se șterge explicit directorul temporar. Runnerul GitHub-hosted este oricum distrus după job.

## Descărcare și verificare

1. Citește research-lock.json; nu duplica versiunea sau hash-urile în YAML.
2. Descarcă manifestul global oficial de la piston-meta.mojang.com prin HTTPS și găsește intrarea versiunii fixate. Atenție: manifest_sha1 din research-lock.json este hash-ul version JSON-ului Minecraft 26.3, nu al manifestului global.
3. Descarcă version JSON-ul din URL-ul manifestului global și verifică SHA-1 față de manifest_sha1 sau accepted_manifest_sha1. Verifică apoi id, hash-urile client/server și javaVersion.majorVersion față de research-lock.json.
4. Descarcă client și server JAR din URL-urile oficiale indicate de version JSON și verifică SHA-1 înainte de folosire.
5. Descarcă asset index-ul din version JSON și verifică SHA-1-ul indicat acolo; descarcă obiectele din resources.download.minecraft.net și verifică SHA-1 și dimensiunea fiecăruia. Prima implementare descarcă setul complet al indexului pentru ca horizon-prepare și harness-urile să aibă aceleași intrări ca instalarea vanilla.
6. Descarcă artifactele Maven listate în version JSON, cu căile relative validate și URL-uri HTTPS, verificând SHA-1 și dimensiunea declarată. horizon-prepare selectează apoi bibliotecile de client necesare pentru platforma curentă.
7. Orice mismatch de versiune, URL lipsă, fișier incomplet sau checksum greșit oprește jobul. Nu accepta automat hash-uri noi: actualizarea lock-ului cere verificare și documentarea motivului.

## Integrarea cu tooling-ul existent

crates/horizon-prepare este importatorul/preparatorul canonic și acceptă deja --version-json, --client-jar, --libraries-dir, --assets-dir, --server-jar și --minecraft-dir. Workflow-ul creează un layout standard de launcher în directorul temporar și îl transmite prin --prepare --minecraft-dir ... --server-jar ...; nu este necesară o nouă cale de import în Rust. tools/lab.py rămâne utilitarul Python pentru inventariere/decompilare locală, dar nu este necesar să descarce un Vineflower JAR pentru testele de paritate. Nu duplica logica de import sau verificare.

După pregătire, folosește infrastructura existentă: tools/parity.py --list și --status; docs/port-ledger.md și tools/port_ledger.py; tools/test_*_parity.py și oracle-urile Java existente; docs/known-issues.md.

Selectează testele pe baza dependențelor reale de resurse. Nu pretinde că toate cele 678 harness-uri au rulat doar pentru că un subset a trecut. Nu relaxa sau dezactiva teste pentru a obține un rezultat verde.

## Loguri și confidențialitate — limită importantă

Repository-ul este public. Așadar, logurile GitHub Actions sunt vizibile publicului. Acest mecanism nu face execuția secretă față de GitHub și nu ascunde metadatele din loguri; el împiedică păstrarea și distribuirea JAR-urilor/assets prin Git, artifacts, cache sau releases.

Logurile trebuie să conțină numai versiunea țintă, numele fișierului și rezultatul verificării checksum (fără conținutul fișierului), numele harness-urilor, rezultat PASS/FAIL și numărul de teste, plus mesaje de eroare curățate.

Nu afișa tokenuri, secrete, conținut binar, fișiere întregi sau dump-uri de resurse. Nu activa debug logging pentru descărcări. Nu folosi secrete pentru URL-urile publice Mojang.

## Criterii de acceptare

- [x] Workflow manual, separat de CI-ul obișnuit.
- [x] JDK 25 și versiunea 26.3 sunt verificate din research-lock.json.
- [x] Manifestul versiunii, client/server JAR, asset index, obiecte și biblioteci sunt verificate criptografic din lock/manifest.
- [x] Refolosește interfața existentă horizon-prepare --prepare și layout-ul standard de launcher; nu modifică fluxul local.
- [x] Harness-urile pot fi alese ca smoke, all sau named; rezultatele sunt reduse la sumar și numele testelor eșuate.
- [x] Nu există upload-artifact, cache sau commit de resurse Mojang.
- [x] Directorul temporar este curățat la final, inclusiv la eșec.
- [x] Logurile nu expun conținutul resurselor; output-ul harness-urilor este filtrat la sumar.
- [ ] Testarea este efectiv rulată; doar atunci se actualizează TEST_RESULTS.md cu rezultate concrete.

## Stare actuală

Implementarea este adăugată pe branch-ul PR #1, dar nu a fost încă executată: workflow_dispatch este disponibil numai după ce workflow-ul există pe branch-ul default. Nu au fost descărcate resurse Mojang în această conversație și nu există încă rezultate Java↔Rust noi. După ce schimbarea ajunge pe main, se poate declanșa manual.
