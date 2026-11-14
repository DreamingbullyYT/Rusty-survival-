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
