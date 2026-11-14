# System Registry — sursa adevărului pentru ownership

## Stare

Nu există încă sisteme Horizon inventariate în acest registru. Sursa upstream trebuie importată și inspectată înainte de completare.

## Regulă

Fiecare responsabilitate are un singur owner canonic. Aliasurile, fațadele și adapterele trebuie să indice owner-ul real și să nu replieze logica.

## Format pentru fiecare sistem

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

Detectorul conceptual citește intrări `[[system]]` din `project-memory/system_registry.toml`. Menține acel fișier în sincron cu acest registru.
