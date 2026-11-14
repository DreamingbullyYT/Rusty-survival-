# Dependency Map

## Stare

Prima hartă a dependențelor directe a fost extrasă din manifestele Cargo ale celor 19 crate-uri clasificate upstream ca engine la commitul bcb7258ad101638c57e8cdd7b1e3da406440bea8.

Aceasta este o hartă de dependențe locale declarate prin path în Cargo.toml, nu încă un graf complet Cargo transitive. Nu deducem ownership doar dintr-o muchie.

## Muchii directe către alte crate-uri engine

| Consumer | Provider(i) declarați |
|---|---|
| horizon-assets | horizon-core |
| horizon-bench | horizon-system |
| horizon-client | horizon-assets, horizon-client-core, horizon-client-input, horizon-client-lib, horizon-client-render, horizon-command, horizon-core, horizon-hooks, horizon-items, horizon-narrator, horizon-perf-format, horizon-protocol, horizon-server-files, horizon-system, horizon-threads, horizon-world |
| horizon-client-core | horizon-world |
| horizon-client-input | horizon-client-core |
| horizon-client-lib | horizon-assets, horizon-client-core, horizon-client-input, horizon-client-render, horizon-command, horizon-core, horizon-hooks, horizon-items, horizon-narrator, horizon-perf-format, horizon-protocol, horizon-server-files, horizon-threads, horizon-world |
| horizon-client-render | horizon-assets, horizon-client-core, horizon-client-input, horizon-command, horizon-core, horizon-hooks, horizon-items, horizon-narrator, horizon-perf-format, horizon-system, horizon-threads, horizon-world |
| horizon-command | horizon-client-core, horizon-core, horizon-protocol, horizon-world |
| horizon-core | — |
| horizon-hooks | — |
| horizon-items | horizon-client-core, horizon-core, horizon-world |
| horizon-narrator | — |
| horizon-perf-format | — |
| horizon-prepare | — |
| horizon-protocol | horizon-core, horizon-world |
| horizon-server-files | horizon-core, horizon-protocol, horizon-world |
| horizon-system | — |
| horizon-threads | — |
| horizon-world | horizon-core, horizon-threads |

## Dependențe directe în afara layer-ului engine

Manifestele crate-urilor engine declară și aceste muchii către crate-uri clasificate upstream ca platform, Horizon sau tool. Sunt importante deoarece arată unde granița la nivel de crate nu coincide cu granița comportamentului vanilla.

| Consumer engine | Provider în alt layer | Layer conform tools/layers.json |
|---|---|---|
| horizon-client | horizon-anticheat, horizon-chat-filter, horizon-dualsense, horizon-hdr, horizon-mod-api-gen, horizon-mods, horizon-optifine, horizon-shader-api, horizon-shaders | platform |
| horizon-client | horizon-friends, horizon-theme, horizon-telemetry | platform |
| horizon-client | horizon-cosmetics | horizon |
| horizon-client | horizon-perf | tool |
| horizon-client-input | horizon-theme | platform |
| horizon-client-lib | horizon-anticheat, horizon-hdr, horizon-mcp, horizon-mods, horizon-optifine, horizon-shader-api, horizon-shaders | platform |
| horizon-client-lib | horizon-telemetry, horizon-theme | platform |
| horizon-client-lib | horizon-perf | tool |
| horizon-client-render | horizon-hdr, horizon-optifine, horizon-polytone, horizon-shader-api, horizon-shaders, horizon-theme | platform |
| horizon-client-render | horizon-perf | tool |

Acestea sunt dependențe declarate în manifestele de la commitul upstream fixat. Nu sunt automat bug-uri și nu autorizăm eliminarea lor. Verificăm feature gates, callsites și responsabilitățile înainte de orice schimbare.

## Direcții și constatări inițiale

- Stratul engine depinde în principal de engine, dar excepțiile de mai sus sunt prezente și trebuie înțelese, nu șterse mecanic.
- horizon-client și horizon-client-lib sunt noduri mari de integrare: trag atât sisteme vanilla, cât și componente platform/Horizon/tool.
- horizon-client-render are responsabilități de randare vanilla, dar și integrare cu HDR, OptiFine, Polytone, shader APIs și theme.
- horizon-client-input include input vanilla, dar și integrare cu horizon-theme; manifestul său descrie și controllere/clipboard/window options.
- horizon-hooks nu are dependențe locale declarate în manifestul inspectat; asta nu demonstrează singur că fiecare hook este necesar vanilla.
- horizon-world depinde de horizon-core și horizon-threads; această muchie trebuie păstrată dacă este necesară pentru semantica și performanța vanilla.
- Nu am rulat încă un audit complet de cicluri/transitive prin cargo metadata pentru toate workspace-urile. Nu declarăm graful complet până atunci.

## Reguli

- Orice dependență trebuie să aibă motiv și direcție documentate.
- Evităm ciclurile între sisteme.
- Dependențele către implementări concrete se justifică; preferăm interfețe interne existente când acestea sunt potrivite.
- Nu duplicăm date, cache-uri sau mecanisme de sincronizare fără o decizie explicită.
- O dependență de platform/Horizon într-un crate engine nu se elimină înainte de a urmări toate callsites, feature gates, contractele și testele.

## Înregistrare pentru fiecare muchie confirmată

consumer -> provider, motiv, tipul contractului, riscuri, teste și excepții.


## Clarificare de ownership din AGENTS.md

- horizon-client-lib nu reprezintă automat un al doilea arbore de surse: manifestul indică surse existente sub crates/horizon-client. Pentru analiza duplicărilor, verificăm exact Cargo target src_path și modulele partajate înainte de a interpreta două crate-uri ca două implementări.
- horizon-command deține parsing-ul/tree-ul Brigadier, în timp ce dispatch-ul și efectele comenzilor sunt în gameplay-ul din horizon-client.
- horizon-world deține datele și mecanismele world/server-side (ticks, redstone, worldgen, entity physics); horizon-client coordonează bucla de gameplay și client/server. Muchiile nu trebuie confundate cu ownership comun.
- horizon-client-render nu ajunge la gameplay, conform documentației upstream; dependențele sale către state-ul world/client trebuie urmărite ca flux de citire, nu presupuse drept duplicate.
