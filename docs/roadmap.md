# Roadmap

This roadmap is gated by automated tests, security review, and reproducible builds. Dates are intentionally omitted.

## M0 — Foundation
- [x] Rust workspace and core library.
- [x] Stable initial C ABI and ownership contract.
- [x] Android Kotlin/JNI bridge source contract.
- [x] CI for formatting, lint, tests, and release build.
- [ ] Android NDK packaging/build verified in a real Android project.

## M1 — URL and networking foundations (in progress)
- [x] Standards-based URL parsing and canonicalization using the Rust `url` crate.
- [x] Restrict navigation targets to HTTP(S).
- [x] Reject embedded URL credentials and control characters.
- [ ] Define request/response and cancellation APIs.
- [ ] Implement HTTPS with certificate validation and redirect policy.
- [ ] Add network tests, including redirect and private-network policy.
- [ ] Add DNS rebinding and SSRF protections before arbitrary URL loading.

## M2 — HTML and DOM
- [ ] HTML5 tokenizer and tree builder using an audited standards implementation.
- [ ] DOM node model, mutation, traversal, and basic events.
- [ ] Run relevant Web Platform Tests (WPT).

## M3 — CSS and layout
- [ ] CSS parser, selectors, cascade, computed values.
- [ ] Incremental layout and viewport sizing.
- [ ] Expand WPT coverage and visual regression tests.

## M4 — Painting and Android presentation
- [ ] Display lists, text shaping, image decoding.
- [ ] Rasterization and compositor.
- [ ] Present frames through Android Surface with lifecycle-safe resource handling.

## M5 — JavaScript and Web APIs
- [ ] Select and audit an ECMAScript runtime.
- [ ] Implement DOM bindings, events, timers, promises integration, and core Web APIs.
- [ ] Track ECMAScript and WPT conformance separately.

## M6 — Storage and security hardening
- [ ] Cookies, HTTP cache, local storage, quota and eviction.
- [ ] Same-origin policy, CSP, permissions, sandbox and process isolation.
- [ ] Fuzz parsers and IPC boundaries.

## M7 — Compatibility and performance
- [ ] Expand WPT coverage.
- [ ] Benchmark page load, script execution, layout, frame time, memory, and battery.
- [ ] Optimize only from reproducible profiles.

A milestone is not complete because a demo page works. It requires tests, documented limitations, security review, and reproducible build instructions.
