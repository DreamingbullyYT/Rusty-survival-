# Android Vulkan POC

Isolated platform/rendering proof of concept; not part of the game engine.

Uses winit 0.30.13 with NativeActivity glue and wgpu 25.0.0 with Vulkan only. It renders a clear-only frame and logs adapter, driver, surface and lifecycle details.

## Build locally

Install Rust, Android SDK/NDK, Gradle and cargo-ndk:

    rustup target add aarch64-linux-android
    cargo install cargo-ndk
    cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --release
    gradle :app:assembleDebug

The APK is written under app/build/outputs/apk/debug/.

## Device test

Install the APK on an ARM64 Android device. It should display a solid blue-green screen. Check logcat for Vulkan adapter/driver information, surface capabilities, Resumed/Suspended events, and surface recreation after rotation and background/foreground.

Passing CI proves compilation and packaging only. Real-device presentation and lifecycle remain unverified until tested on hardware.
