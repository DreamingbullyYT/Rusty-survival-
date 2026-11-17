# Android Vulkan Surface Proof of Concept

Status: PLANNED — NO APK OR DEVICE RESULT YET
Purpose: validate the Android app lifecycle + window surface + wgpu 25 Vulkan presentation path before committing the engine to a renderer backend.

## Candidate stack

- Rust target: aarch64-linux-android (initial device target only).
- Window/lifecycle candidate: winit 0.30 with android-native-activity, using the matching android-activity version re-exported by winit.
- GPU abstraction candidate: wgpu 25, Vulkan backend.
- Android API baseline: choose after confirming the test device's Vulkan support and packaging toolchain; do not infer support from OS version alone.
- No Minelib source, shaders, assets, world model, or gameplay code in this proof of concept.

Why this stack:
- wgpu 25 documents Vulkan support for Linux/Android.
- winit 0.30 documents Android integration through android-activity glue and requires an Android entry point plus AndroidApp passed to the event loop.
- winit documents that the app should create graphics/window resources after Resumed; Suspended on Android means the SurfaceView has been destroyed and existing render surfaces are invalid.

References:
- wgpu 25 supported platforms: https://docs.rs/crate/wgpu/25.0.0
- winit 0.30 Android integration: https://docs.rs/winit/0.30.13/winit/platform/android/
- winit Android source and glue requirements: https://docs.rs/winit/0.30.13/src/winit/platform/android.rs.html
- Vulkan Android surface extension: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_android_surface.html

## Minimal behavior

1. Launch as a native Android application.
2. Wait for Resumed before creating the window and wgpu surface.
3. Enumerate adapters and log vendor, device, backend, API/driver version, supported features, surface formats and present modes.
4. Request only the minimum required device features.
5. Render a single clear color and present continuously or on redraw.
6. On Suspended, stop rendering and release surface-dependent resources safely.
7. On the next Resumed, recreate the window/surface/configuration and continue rendering.
8. Log all failures and lifecycle transitions to logcat.

## Acceptance tests

Build/packaging:
- [ ] Rust Android target compiles in CI.
- [ ] APK packages and installs.
- [ ] App launches without crash.

Surface/lifecycle:
- [ ] First surface creation and first present succeed.
- [ ] Home -> app resume works.
- [ ] Screen off/on works.
- [ ] Rotate or resize causes correct surface/swapchain recreation.
- [ ] Repeated background/foreground cycles do not leak or reuse stale surfaces.
- [ ] Back navigation and process destruction exit cleanly.

GPU:
- [ ] Vulkan adapter is selected on target device.
- [ ] Surface format and present mode are supported.
- [ ] No validation errors or synchronization faults in normal run.
- [ ] Device/driver details are logged for reproducibility.

Performance/stability:
- [ ] No continuous high CPU usage while suspended.
- [ ] Stable frame presentation for a 10-minute smoke test.
- [ ] Memory usage and thermal behavior recorded.
- [ ] Fallback behavior documented if Vulkan is unavailable; no silent claim that Vulkan succeeded.

## Test devices

Start with one current ARM64 Android device, then add at least one device from a different GPU family before treating the path as broadly validated. Record exact model, Android build, GPU vendor/model, driver version and result. Do not generalize from one phone.

## Decision gate

- If the POC passes: keep wgpu+winit as the leading renderer/platform candidate, then benchmark and test broader lifecycle/input/storage needs.
- If it fails due to winit lifecycle limitations: test native Android activity/surface integration with wgpu directly.
- If Vulkan driver support is missing or unreliable: report the limitation and evaluate an explicit fallback without changing vanilla simulation semantics.

Passing this POC proves only app startup, Android surface lifecycle and basic GPU presentation. It does not prove engine performance, Minecraft rendering parity, chunk meshing, or gameplay correctness.
