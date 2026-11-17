# Security model and current limitations

## Current status

The current foundation does not load or execute web content. It is not safe to expose as a general-purpose browser and must not be described as sandboxed or production-ready.

The initial HTML parser currently has no explicit input-size, node-count, nesting-depth, or execution-time budget. Do not use it to parse arbitrary untrusted or very large documents until resource limits and parser stress/fuzz tests are implemented.

Navigation targets are parsed and canonicalized with the Rust `url` crate, which provides standards-oriented URL handling. This validation is **not a network security boundary**: it does not resolve DNS, enforce public-address policy, validate redirects, establish TLS connections, or implement an origin model. Those controls must be designed and tested before network loading is implemented.

## Required controls before loading untrusted content

- Standards-based URL parsing and canonicalization, combined with explicit scheme, origin, redirect, DNS/IP, and connection policies.
- HTTPS certificate and hostname validation; no insecure fallback.
- Same-origin policy and a clearly specified origin model.
- Content Security Policy and mixed-content policy.
- Process isolation for untrusted site content; never treat threads as a sandbox.
- Permission mediation for camera, microphone, location, notifications, downloads, and file access.
- Strict IPC validation, bounded messages, and lifecycle-safe handles.
- Resource limits for memory, CPU, recursion, and decompression.
- Safe handling of redirects, schemes, MIME types, and downloads.
- Fuzzing for parsers, decoders, URL handling, and IPC.
- Dependency and vulnerability monitoring.

## FFI rules

- Keep the ABI small and versioned.
- Document pointer validity, ownership, and lifetime for every exported function.
- Never allow Rust panics to unwind across the C ABI.
- Validate all foreign input; do not assume Kotlin input is trustworthy.
- Do not expose raw internal pointers as public API.
