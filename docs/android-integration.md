# Android integration

The bridge source is provided as an integration contract, not a complete Android application module. The repository currently has no Android Gradle project, SDK configuration, or packaged native binaries.

## Components

- Kotlin wrapper: `android/engine-bridge/src/main/java/dev/minimalium/engine/NativeEngine.kt`
- JNI bridge: `android/engine-bridge/src/main/cpp/minimalium_jni.cpp`
- CMake: `android/engine-bridge/src/main/cpp/CMakeLists.txt`
- C ABI header: `include/minimalium.h`

## Required integration steps

1. Install Android Studio and the Android NDK.
2. Add Rust Android targets for each ABI that the app supports (at minimum arm64-v8a; add others only when tested).
3. Build `minimalium-core` for the Android target and place the resulting `libminimalium_core.so` under `android/engine-bridge/src/main/jniLibs/<abi>/`.
4. Configure the consuming Android app's externalNativeBuild to use this CMake project.
5. Ensure the C header include path and imported library path resolve in the consuming Gradle project.
6. Package `libminimalium_jni.so` and `libminimalium_core.so` for the same ABI.
7. Exercise create/navigate/destroy on a physical Android device and add instrumentation tests.

Example Rust target setup (verify target availability for the installed toolchain):

```sh
rustup target add aarch64-linux-android
cargo build -p minimalium-core --release --target aarch64-linux-android
```

The exact NDK linker configuration and Gradle integration must be added and verified in the actual host Android project. Do not claim Android support is verified until a device build and smoke test pass.

## Important

The current `validateNavigationTarget` method only checks a limited HTTP(S) URL shape and stores it in native memory. It does not open a socket, fetch a document, parse HTML, execute JavaScript, or render a page.
