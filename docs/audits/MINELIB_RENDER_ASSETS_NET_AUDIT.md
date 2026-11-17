# Minelib Render, Assets and Networking Audit

Audit date: 2026-11-17
Upstream repository: https://github.com/Viendax/minelib
Pinned upstream commit: 1cd3faeca7a29acf491b794dda1eacf768d61e41
Scope: source-level review of crate manifests and renderer, asset-loader, and networking entry points.
Method: GitHub source inspection only. No upstream source was cloned, built, or executed.

## 1. Renderer (crates/render)

### Observed design
- Depends on wgpu, winit, glam, bytemuck, pollster, font8x8, and directly on minelib-world and minelib-assets.
- Public renderer construction accepts Arc<winit::window::Window>; it creates a wgpu::Surface from that desktop window and configures the surface internally.
- Adapter selection requests a high-performance adapter compatible with that surface.
- Requests optional MULTI_DRAW_INDIRECT and TIMESTAMP_QUERY features when supported by the adapter.
- Chooses a surface format, present mode, and alpha mode from surface capabilities; it prefers Immediate, then Mailbox, then FIFO.
- Renderer owns substantial world-specific pipeline state, GPU chunk meshes, textures, TAA, reflections, overlays, capture, and Minelib-specific lighting data.
- The root render module is approximately 2,583 lines / 99 KB at the pinned commit; shader and supporting modules add substantially more code.

### Android integration nuance
The pinned workspace uses winit 0.30 and wgpu 25. The wgpu 25 platform table lists Vulkan support on Linux/Android. Winit 0.30 also has an Android backend, but its documentation requires an Android glue feature such as android-native-activity and an android_main entry point; the application must pass AndroidApp into the event loop. The Minelib workspace manifests inspected here declare winit = 0.30 without Android feature flags, contain no android-activity dependency/entrypoint, and its release workflow only demonstrates Windows builds. Therefore Android support is technically plausible, but not wired or demonstrated by this pinned Minelib build.

Winit's Android lifecycle guidance says graphics initialization/window creation should happen after Resumed; on Android, Suspended corresponds to destruction of the SurfaceView and invalidates render surfaces. This lifecycle must be handled explicitly.

### Compatibility assessment
The wgpu + winit combination is a credible candidate path for an Android Vulkan proof of concept. It is not yet a verified choice. The Minelib renderer remains tightly coupled to Minelib world/assets and custom vertex/lighting/registry assumptions, so it cannot be adopted wholesale.

### Decision
**ADAPT algorithms and GPU techniques only; do not import the renderer wholesale.**
Potential study topics: chunk GPU buffer management, visibility/culling, batching, texture-array upload, shader organization, timing instrumentation, and render graph separation. Reimplement them behind our own renderer interfaces and test on Android hardware.

### Required proof before selecting wgpu or another backend
- Build an Android-only surface sample on a real device.
- Verify Android lifecycle pause/resume and surface recreation.
- Verify Vulkan instance/device/queue selection and supported formats.
- Verify swapchain recreation on rotation, background/foreground, and resolution changes.
- Verify synchronization, frame pacing, memory limits, device loss, and fallback behavior.
- Benchmark on representative Adreno, Mali, and Xclipse devices.
- Confirm chosen crate versions and Android activity integration are maintained and license-compatible.

## 2. Assets (crates/assets)

### Observed design
- Uses std::fs, std::env, PathBuf, and a platform-specific home-directory convention.
- Stores data under .minelib/, with resourcepacks/, saves/, and options.txt.
- Resource packs can be directories or ZIP files; ZIP entries are loaded into a HashMap.
- Missing files can fall back to a built-in default pack.
- The default pack is embedded with include_bytes! from assets/VoxelArt_Libre.zip and copied into the user's resource-pack directory at first initialization.
- Texture helpers understand Minelib-specific texture names, legacy and modern Minecraft texture paths, biome tint approximations, and Minelib-specific texture sources.
- The source code itself does not establish that every embedded asset has a compatible license or that the complete pack may be redistributed.

### Compatibility assessment
The directory/ZIP resource-pack abstraction is useful in principle. Global home-directory assumptions and direct filesystem ownership are not appropriate as-is for Android scoped storage/app-private paths. Texture lookup and tint behavior are Minelib-specific and are not a complete Minecraft Java 26.3 resource/data pack implementation.

### Decision
**ADAPT the resource I/O abstraction only; REPLACE platform path ownership and version-specific resource semantics.**
Create an Android storage provider using app-private directories and Android storage APIs. Keep asset loading independent of gameplay. Build a version-aware resource/data pack loader based on 26.3 formats and schemas.

### Asset provenance restriction
Do not copy or bundle VoxelArt_Libre.zip or any other Minelib/Mojang asset until provenance and redistribution rights are independently documented. No asset archive was downloaded or copied during this audit.

## 3. Networking (crates/net)

### Observed design
- Uses Serde and Postcard for serialization.
- Uses Noise XX over TCP with Curve25519, ChaCha20-Poly1305 and BLAKE2s through the snow crate.
- Includes account registration/login, password-derived authentication, Ed25519 signatures, TOTP, device trust, relay pinning, map upload/download, room discovery, voice and Minelib-specific FPS/build game messages.
- Game messages contain Minelib-specific block IDs, map hashes, player poses, custom relay events, build edits, inventory and game modes.
- Transport implements custom length-prefixed encrypted frames, queues, batching, back-pressure and stale-message dropping.

### Compatibility assessment
This is Minelib's own relay/client protocol, not Minecraft Java Edition's protocol. Its message types and gameplay assumptions do not provide 26.3 packet compatibility. Reusing transport or authentication code would also require an independent security review and risks coupling the vanilla runtime to Minelib's account/relay design.

### Decision
**REPLACE for the vanilla engine and Minecraft Java 26.3 networking.**
If multiplayer is in scope, write a separate protocol specification from 26.3 reference behavior. Do not use Postcard, Minelib relay message enums, or Minelib game protocol as substitutes for Java protocol packets. Generic queueing/back-pressure ideas may be studied without copying source.

## 4. Updated reuse decisions

| Component | Decision | Allowed use |
|---|---|---|
| Renderer | ADAPT | Study GPU batching, buffers, culling, texture upload, instrumentation; independently implement Android surface and rendering integration |
| Assets | ADAPT | Study directory/ZIP pack abstraction; independently implement Android storage and Java 26.3 pack semantics |
| Networking | REPLACE | Do not use Minelib protocol for Java multiplayer; design against Java 26.3 packet behavior |
| Embedded VoxelArt pack | BLOCKED pending license | No bundling/copying until provenance and redistribution rights are proven |

## 5. Limitations

- This was source inspection, not compilation or runtime testing.
- No Android build or Vulkan surface was tested.
- No GPU benchmark was run.
- No network security audit or protocol interoperability test was run.
- No license opinion is given; the workspace's AGPLv3 root license and the asset pack's independent provenance still require review.
- No source code, binary, shader, or asset was copied into Rusty Survival.
