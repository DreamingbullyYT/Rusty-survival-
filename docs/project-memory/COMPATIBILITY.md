# Compatibility and Provenance

## Current verified targets
- Android: project target only; no build verified.
- Desktop: not currently in scope.
- Vulkan: candidate graphics API/backend; device/API requirements not yet validated.
- Minecraft Java Edition target: 26.3 (selected project target; no parity implementation verified yet).

## Minelib upstream
- Repository: https://github.com/Viendax/minelib
- Audit reference: main commit 1cd3faeca7a29acf491b794dda1eacf768d61e41 (2026-11-17).
- Root LICENSE: AGPLv3.
- GitHub repository metadata: license SPDX field NOASSERTION.
- Workspace: Rust 2021, rust-version 1.85, winit 0.30, wgpu 25.
- README platform status: Windows/Linux available; macOS soon; Android/iOS in preparation.
- Current release workflow: Windows runner, Windows game/relay/launcher executables.
- No Minelib source has been copied into this repository.

## Architectural inspiration
- Horizon may inform architectural ideas, modularity, system boundaries, and engineering approach.
- No Horizon source code, assets, or implementation is to be copied.
- All systems in this repository are independently designed and implemented unless a separately audited Minelib component is deliberately reused.

## Candidate crate portability
- world: manifest only depends on bytemuck; platform-neutral at dependency level, but behavior and test compatibility remain unverified.
- render: wgpu 25 + winit 0.30; Android Vulkan viability must be proven with an actual Android surface and device.
- platform: winit event loop and keyboard/mouse-centric input; Android lifecycle/touch support not demonstrated.
- app: includes cpal, rodio, arboard, winit and many tightly coupled systems; not Android-ready by evidence gathered so far.
- p2p: iroh/mainline/Tokio; Android support and license scope require further review.

## Asset policy
Every asset must have a source URL or origin, author, license, attribution requirements, modifications, and redistribution status recorded before inclusion.
