# Minimalium Engine

Independent Android-first browser-engine research project focused on performance, security, and a small native core.

> **Status: foundation / pre-alpha.** This repository does not yet implement a web browser. The current milestone establishes a compilable Rust core, a narrow C ABI, an Android JNI bridge contract, an HTML5 parser with an initial engine-owned DOM tree, tests, and architecture documentation. It does not implement the full DOM Standard, execute JavaScript, fetch network resources, render pages, or provide a security sandbox.

## Project principles

- Independent implementation: do not embed Blink, Chromium, Gecko, or Android WebView as the rendering engine.
- Rust-first core; Kotlin is the Android-facing API.
- Small, explicit module boundaries and a versioned C ABI.
- Security and conformance are requirements, not claims made before implementation.
- Prefer measured performance over assumptions.

## Repository layout

- `engine/core`: Rust library and C ABI.
- `android/engine-bridge`: Kotlin API and Android NDK/JNI bridge.
- `include`: public C header.
- `docs`: architecture, roadmap, security model, and decisions.
- `.github/workflows`: formatting, lint, and test checks.

## Build the Rust foundation

Requires a current stable Rust toolchain.

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p minimalium-core --release
```

The release build produces a native library for the host target. Android builds require the Android NDK and Rust Android targets; see [Android integration](docs/android-integration.md).

## Current native API

The initial API supports engine creation/destruction and validation/recording of a navigation target. Navigation is **not** a network request and does not load or display a page.

## Roadmap

See [the roadmap](docs/roadmap.md). Work must proceed incrementally: HTML parsing, DOM, CSS, layout, painting/compositing, JavaScript, networking, storage, and process isolation each need independent design and conformance tests.

## License

MIT. See [LICENSE](LICENSE).
