# Compatibility Policy

## Active target
DreamMod integrates with Horizon. Compatibility is defined by exact pinned revisions, not a broad claim that all Horizon versions work.

Every Core build must record:
- Horizon commit/release
- horizon-mixins commit
- hooks/mod API revision
- Rust toolchain and target
- DreamMod Core version
- supported .dmod API range

## .dmod compatibility
Each package declares a DreamMod API range. The loader rejects incompatible packages before activation and reports the reason. Stable APIs are versioned and deprecated APIs receive a migration window.

## Distribution compatibility
DreamMod Core is integrated at build time. A Core native integration update requires a new prebuilt Horizon distribution. Ordinary interpreted .dmod script/data updates do not require rebuilding Horizon.

## Historical compatibility records
The former Minecraft Java Edition 26.3, protocol 777, data version 5023 and Android-first standalone-engine records describe a superseded prototype direction. They are not DreamMod compatibility claims.
