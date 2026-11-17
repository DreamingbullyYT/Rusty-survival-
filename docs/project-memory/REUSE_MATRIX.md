# Minelib Reuse Matrix — Preliminary Audit

Audit date: 2026-11-17  
Upstream: https://github.com/Viendax/minelib  
Pinned reference: 1cd3faeca7a29acf491b794dda1eacf768d61e41  
Audit depth: manifests, workspace layout, selected crate entry points, README, license, and release workflow. This is not yet a full source/security/test audit.

Decision meanings:
- KEEP: suitable to reuse with minimal changes after license review.
- ADAPT: useful concept/code, but requires isolation, refactoring, or platform work.
- REPLACE: do not reuse implementation; implement our own.
- REJECT: incompatible or unacceptable risk.

| Component | Preliminary decision | Evidence / reason | Required next check |
|---|---|---|---|
| `minelib-world` | ADAPT candidate | Depends only on `bytemuck`; its public API includes block/chunk/generation/mesh/raycast/world systems. It is independent of wgpu and platform crates at manifest level. Its model is a 16×512×16 chunk, not proof of Minecraft Java semantics. | Read all world modules; compare data model, generation, lighting, fluids, collision, and serialization against selected Java version; run tests. |
| `minelib-render` | ADAPT candidate | Uses wgpu 25, winit, glam, and directly depends on Minelib world/assets. Renderer entry point uses a winit Window. | Audit surface creation, shader assumptions, GPU limits, Android Vulkan surface lifecycle, and separation from Minelib-specific world types. |
| `minelib-platform` | REPLACE or ADAPT selectively | Built around winit event loop and keyboard/mouse input; comments describe Windows/Linux/macOS; dependency includes sysinfo. Android lifecycle/touch semantics are not demonstrated. | Inspect cfg-specific code and winit Android integration; likely write a dedicated Android adapter. |
| `minelib-assets` | ADAPT candidate | Uses zip/image plus filesystem/resource-pack logic. | Audit path assumptions, archive safety, image formats, resource-pack format and asset provenance. |
| `minelib-net` | ADAPT candidate | Protocol/crypto-oriented crate with serde/postcard, Noise, Ed25519/X25519, Argon2, ChaCha20-Poly1305, HKDF and hashes. | Threat-model review, protocol compatibility needs, crypto API review, fuzzing and test coverage. Do not inherit Minelib account/service design by default. |
| `minelib-p2p` | REPLACE initially | Uses iroh, mainline DHT, Tokio; package manifest says MIT OR Apache-2.0 while repository root is AGPL-3.0. Strongly tied to Minelib relay architecture. | Resolve license scope and inspect transport/security/Android support before any reuse. |
| `minelib-relay` | REPLACE initially | Server executable built around Minelib P2P/net and relay protocol. Not required for first Android vanilla vertical slice. | Revisit only after multiplayer architecture is specified. |
| `minelib-app` | REPLACE | Monolithic game application ties together UI, platform, rendering, audio, networking, accounts, saves, and multiplayer. Depends on cpal, rodio, arboard and winit. | Use only as behavioral reference where useful; design our own modular runtime. |
| `minelib-launcher` | REPLACE | Separate desktop launcher using winit and softbuffer; no Android launcher integration. | None for initial game runtime. |

## Cross-cutting findings

- Root workspace contains nine crates: platform, render, world, assets, net, relay, p2p, app, launcher.
- Workspace declares Rust 1.85 / edition 2021, winit 0.30, wgpu 25.
- README presents Windows and Linux as available; Android/iOS are described as in preparation.
- Current release workflow runs on Windows and builds Windows executables; it does not establish Android support.
- `minelib-world` is the strongest initial technical candidate because its manifest has only one dependency and no platform/render dependency.
- No Minelib code has been copied into this repository.
- Repository LICENSE is AGPLv3. GitHub metadata reports `NOASSERTION`; this does not override the actual LICENSE file. The P2P crate separately declares MIT OR Apache-2.0; determine whether that declaration is valid for the crate and its contributions before reuse.
- Assets have separate credits/licensing; do not copy assets until each item is verified.

## Current recommendation

Do not import Minelib wholesale. Continue a source-level audit of `world` first, while independently prototyping Android lifecycle + Vulkan surface + touch input. Keep the Android platform adapter separate. No component receives final KEEP approval yet.
