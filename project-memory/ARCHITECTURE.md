# Arhitectură

## Stare curentă

**Status: neauditată.** Nu inventăm arhitectura înainte de a inspecta sursa upstream.

## Principii obligatorii

- Un singur proprietar pentru fiecare responsabilitate.
- Separare clară între domenii; dependențele trebuie să curgă într-o direcție explicită.
- Refolosim implementările existente când sunt corecte și potrivite.
- Nu introducem abstracții sau crate-uri fără o nevoie demonstrată.
- Preferăm refactoruri mici, verificabile și reversibile.
- API-ul propriu de modding este în afara scope-ului curent.

## Arhitectura țintă

Se completează numai după auditul sursei. Pentru fiecare sistem se vor documenta: responsabilitatea, proprietarul unic, intrările/ieșirile, dependențele, limitele și testele.

## Upstream architecture discovered

The pinned upstream already defines engine/platform/Horizon/tool layers in `docs/layers.md`, with dependency rules in `tools/layers.json` and enforcement in `tools/check_layers.py`. Reuse these definitions and checks. Do not create a second layer checker.

The current upstream classifies 19 crates as engine. Its own documentation notes that some gameplay and platform code is still mixed inside large crates; this must be audited rather than assumed solved. The current classification is evidence for the inventory, not a final target architecture.

## Clarificări de ownership din harta upstream

La baseline-ul `bcb7258ad101638c57e8cdd7b1e3da406440bea8`, `AGENTS.md` indică:
- `horizon-client/src/game.rs` și `src/game/`: gameplay loop, tick și frame;
- `horizon-client-render`: rendering, chunks, meshing, lighting, streaming, HUD și meniuri; documentația upstream precizează că nu deține gameplay;
- `horizon-world`: world data, NBT, regions, chunks, datafix, worldgen, block ticks/redstone și entity physics;
- `horizon-items`: block/item facts și item components;
- `horizon-command`: parsing/tree Brigadier; dispatch-ul și efectele rămân în gameplay-ul din `horizon-client`;
- `horizon-protocol`: codecs, framing, encryption și packets;
- `horizon-core`: versiune, Java RNG/Math, poziții și primitive numerice.

Acestea sunt puncte de pornire documentate, nu concluzii finale de ownership. Fiecare sistem trebuie verificat în cod, manifest, callsites, dataflow și harness înainte de refactor.


## Inventar cantitativ al celor 19 crate-uri engine (2026-11-15)

Sursa: numărare mecanică pe snapshot-ul `bcb7258`. Raport complet: `reports/engine-audit-2026-11-15/inventory.md`. Cifrele de teste inline sunt estimări (linii de la primul `#[cfg(test)]` până la sfârșitul fișierului).

| Mărime | Linii Rust |
|---|---:|
| Total engine (19 crate-uri) | 1.430.651 |
| `src/` fără teste | 887.804 |
| Teste în `src/` (estimare) | 458.572 |
| `examples/` | 70.745 |
| `tests/` | 13.379 |

Cele mai mari: `horizon-client` 413.806 (din care `game/` 276.958 fără teste), `horizon-client-lib` 379.718 (surse în `crates/horizon-client/`, atribuite după manifest; `mobs/` 78.670), `horizon-client-render` 274.857 (`gfx` 36.042), `horizon-world` 201.670 (`worldgen` 56.804, `examples/` 41.092), `horizon-protocol` 52.483 (`adapter` 15.972).

Nepotriviri între documentația upstream și cod (de verificat înainte de orice mutare):
- `CLAUDE.md` spune că `horizon-client-lib` „never depends on gameplay”, dar conține `mobs/` (78k linii).
- `horizon-world` este descris ca „fără gameplay loop”, dar conține reguli de jucător și luptă.
- `horizon-threads` este descris ca priorități de thread, dar ~2,9k linii sunt profiler.
- `horizon-client/src/game/native_*` (82 fișiere, ~33k linii) este glue pentru mod API; `docs/layers.md` îl numește strat Platform aflat încă în crate engine.
- În `horizon-client/src/game/`, estimat: `pad*` ~5,7k (controllere), `split/` ~3,3k, `cosmetics` ~1,8k, `extras` ~1,6k, `replay`+`studio` ~1,6k.
- În `horizon-client-render`: pvp, bluemap, minimap, social, mod browser și studio UI, în total ~6,4k.
- `horizon-bench` este grupat în `CLAUDE.md` cu telemetria (Horizon), dar `tools/layers.json` îl pune în engine.
- `horizon-client-input/gamepad.rs` este Platform după `docs/layers.md`.
- `pack_browser.rs` contactează `api.modrinth.com` dintr-un crate engine (excepție permisă).

Aceasta nu este arhitectura țintă. Inventarul semantic (callsites, dataflow, teste per modul) rămâne de făcut; vezi `TASKS.md`.
