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
