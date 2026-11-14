# System Registry — sursa adevărului pentru ownership

## Stare

Inventar inițial de crate-uri realizat din manifestele upstream și docs/layers.md la commitul bcb7258ad101638c57e8cdd7b1e3da406440bea8.

**Atenție:** lista de mai jos este o hartă de orientare, nu un registru final de ownership. Clasificarea upstream engine este la nivel de crate; documentația upstream confirmă că unele crate-uri conțin și funcționalități de platformă/Horizon. Nicio funcție nu este declarată aici ca owner canonic până nu inspectăm modulele, callsite-urile, testele și fluxul de date.

## Inventar provizoriu — 19 crate-uri clasificate upstream ca engine

| Crate | Responsabilitate declarată în manifest | Încadrare preliminară pentru scope-ul vanilla | Stare |
|---|---|---|---|
| horizon-assets | Decodare modele și resurse dependente de versiune | Candidate vanilla: resurse/model decoding | Manifest inspectat; sursa neauditată |
| horizon-bench | Format raport benchmark comun jocului și launcherului | Tooling/performance, nu comportament vanilla | În afara țintei de refactor |
| horizon-client | Client nativ de dezvoltare Horizon | Mixt: gameplay vanilla plus host/platform/Horizon; gameplay-ul încă locuiește aici | Audit prioritar |
| horizon-client-core | Chat/traduceri, SNBT/NBT paths, scoreboard, boss bars, game rules, registry/particle tables | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-client-input | Key bindings, mouse, controllere, clipboard, window options | Mixt: tastatură/mouse vanilla plus platform features | Audit prioritar |
| horizon-client-lib | Sisteme client reutilizabile, control tool și parity probes | Mixt: sisteme de joc, instrumente și integrare platform | Audit prioritar |
| horizon-client-render | Date client/world, chunks, meshing, lumină, streaming, renderer, HUD, meniuri și settings | Mixt: randare/ecrane vanilla plus UI/settings platform | Audit prioritar |
| horizon-command | Arbore Brigadier vanilla, argumente, selectors, parsing, suggestions și forma de rețea | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-core | Fundații de simulare pentru versiunea Java țintă | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-hooks | Extension points și platform registries pentru crate-urile superioare | Infrastructură de extensie; nu presupunem că este Modding API-ul propriu | Audit de contracte necesar |
| horizon-items | Fapte despre block/item, item stacks, data components, enchantments, cooldowns, bundles, NBT | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-narrator | GameNarrator/text-to-speech și starea screen narration | Candidate vanilla: mecanismul de narrator | Verificare backend/platform necesară |
| horizon-perf-format | Contractul de captură .hzperf, protobuf și gzip writer | Tooling/performance, nu comportament vanilla | În afara țintei de refactor |
| horizon-prepare | Importă/verifică jar-urile Minecraft ale jucătorului și exportă resurse | Build/resource tooling, nu runtime vanilla | În afara țintei de refactor |
| horizon-protocol | Protocolul de rețea Java 26.3, protocol 777 | Candidate vanilla | Manifest inspectat; sursa neauditată |
| horizon-server-files | Fișiere vanilla de server: properties, EULA, ops, whitelist, bans și account lookups | Candidate vanilla, cu verificare pentru partea de account lookup | Manifest inspectat; sursa neauditată |
| horizon-system | Informații despre OS, CPU, memorie, drive, power și GPU | Suport platform/performance, nu gameplay vanilla | În afara țintei de refactor |
| horizon-threads | Scheduling hints, allocator și apeluri OS | Infrastructură de suport; se păstrează dacă este necesară, fără extindere arbitrară | Audit de dependențe necesar |
| horizon-world | Date world vanilla, NBT, regiuni și chunk storage | Candidate vanilla | Manifest inspectat; sursa neauditată |

## Observații importante

- docs/layers.md definește engine-ul ca Vanilla Minecraft Java Edition 26.3, dar spune explicit că gameplay-ul încă locuiește în horizon-client și că va fi mutat gradual spre crate-urile portabile.
- horizon-client, horizon-client-input, horizon-client-lib și horizon-client-render nu pot fi tratate ca vanilla pure doar pentru că întregul crate este etichetat engine.
- Documentația upstream indică deja funcționalități de platformă aflate în crate-uri engine: game/mod_api_host.rs, game/native_*.rs, shader_ui.rs, hdr_calibration.rs, codul de controller și splitscreen.
- Nu mutăm și nu eliminăm nimic în această etapă. Mai întâi se identifică exact codul, consumatorii, extension points și comportamentul vanilla observabil.
- Nu proiectăm un Modding API nou acum. horizon-hooks se inspectează doar pentru a înțelege contractele existente și rolul său în engine/platform boundary.

## Regula de ownership

Fiecare responsabilitate are un singur owner canonic. Aliasurile, fațadele și adapterele trebuie să indice owner-ul real și să nu replieze logica.

## Format pentru fiecare sistem confirmat

- ID stabil:
- Nume:
- Owner canonic (crate/modul/fișier):
- Responsabilitate unică:
- Ce NU face:
- API intern:
- Dependențe:
- Consumatori:
- Teste:
- Status:
- Dovezi / commit:

Detectorul conceptual citește intrări [[system]] din project-memory/system_registry.toml. Menține acel fișier în sincron cu acest registru. Intrările automate se adaugă numai după confirmarea semantică a sistemelor; inventarul provizoriu de crate-uri de mai sus nu se transformă automat în ownership.


## Hartă de module indicată de AGENTS.md upstream

Sursa: AGENTS.md din commitul bcb7258ad101638c57e8cdd7b1e3da406440bea8. Acestea sunt indicii de ownership documentate upstream; confirmarea finală se face în cod și prin callsites.

| Responsabilitate | Locații declarate upstream | Notă pentru audit |
|---|---|---|
| Gameplay loop, tick, frame | horizon-client/src/game.rs și src/game/ | Principalul owner actual al gameplay-ului; nu mutăm înainte de inventariere |
| Executabil client | horizon-client/src/main.rs și entry.rs | Entry/executable, separat de logica gameplay propriu-zisă |
| Sisteme client reutilizabile, control și probes | horizon-client-lib; manifestul indică surse existente în horizon-client/src | Atenție la surse partajate prin manifest; nu presupunem duplicate doar pentru că există două crate-uri |
| Rendering, chunks, meshing, light, streaming, HUD, meniuri | horizon-client-render și modulele re-exportate de horizon-client | Documentația spune explicit că acest crate nu ajunge la gameplay |
| Date world, NBT, regions, chunks, datafix, worldgen, block ticks/redstone, entity physics | horizon-world | Owner de date și comportamente de world/server-side, fără gameplay loop |
| Block/item facts și item components | horizon-items | Consumate de gameplay; trebuie urmărit fluxul de date către Game |
| Command parsing/tree | horizon-command | Separat de dispatch/executarea comenzilor, care rămâne în horizon-client/game |
| Client bottom layer: text/lang/SNBT/NBT path/scoreboard/bossbar/gamerules/registry/particles | horizon-client-core | Candidate de logică vanilla |
| Key bindings/mouse/controller/clipboard/window | horizon-client-input | Mixt; izolăm input vanilla de controllere și alte extensii |
| Protocol codecs/framing/encryption/packets | horizon-protocol | Candidate vanilla; target protocol 777 |
| Server-side files/config | horizon-server-files | Verificăm separat account lookups și orice comportament Horizon |
| Version, Java RNG/Math, positions/noise/density/paletted storage | horizon-core | Primitive de simulare ce trebuie să reproducă exact Java |
| Hook/extension seam pentru crate-urile superioare | horizon-hooks | Înțelegem contractele existente; nu dezvoltăm API nou |

### Consecințe pentru inventar

- horizon-client-lib nu trebuie numărat automat ca o a doua implementare a logicii din horizon-client: manifestul poate indica fișiere sursă aflate în alt crate. Verificăm canonical path și Cargo target înainte de a marca duplicate.
- Parsing-ul comenzilor și executarea lor sunt responsabilități diferite: horizon-command deține tree/argument parsing; gameplay-ul din horizon-client deține dispatch și efectele comenzilor.
- horizon-world deține logică world/server-side (inclusiv ticks/redstone), în timp ce horizon-client deține bucla de gameplay și coordonarea client/server. Nu le unim conceptual fără a distinge state ownership și threading.
- Rendererul este separat de gameplay; un sistem vizual care citește starea jocului nu devine automat owner al regulilor de simulare.
