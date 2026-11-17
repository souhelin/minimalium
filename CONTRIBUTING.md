# Contributing

- Keep the project independent of Chromium, Blink, Gecko, and Android WebView as rendering engines.
- Do not claim browser functionality that is not implemented and tested.
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` before submitting Rust changes.
- Add tests with every parser, policy, FFI, or state change.
- Document unsafe Rust and every FFI ownership/lifetime contract.
- Review dependency licenses, maintenance, security advisories, and target support before adding dependencies.
- Prefer small pull requests with explicit acceptance criteria.
