# Audit engine, 2026-11-15 (read-only)

Patru rapoarte produse de subagenți read-only peste snapshot-ul upstream `bcb7258`:

- `deps.md`: graful dependențelor între crate-urile workspace, `tools/check_layers.py`, versiuni externe multiple.
- `inventory.md`: linii și module per crate engine, nepotriviri între documentație și cod.
- `parity.md`: ce se poate extrage fără date Mojang din port map, harness-uri și oracle-uri.
- `dups.md`: trierea celor 835 de grupuri exacte din `reports/upstream-bcb7258-audit.json`.

Limitări: nu s-a rulat niciun `cargo build/check/test`, niciun harness și nu s-au descărcat resurse Mojang.
Numerele marcate „estimare” sau „inferat” în rapoarte nu sunt măsurători. Scripturile ajutătoare ale agenților
nu sunt comise; căile lor din rapoarte apar ca `<scratchpad>`. Detectorul de duplicate produce candidați,
nu dovezi: nimic nu a fost șters sau modificat pe baza lui.
