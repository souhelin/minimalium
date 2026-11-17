# Architecture

## Product boundary

Minimalium is an independent browser-engine project. It must not silently fall back to Android WebView or embed Blink/Chromium/Gecko as its rendering engine. Reuse of focused libraries is allowed after license, maintenance, security, and conformance review.

The current code is only a native foundation. No HTML/CSS/JavaScript/network/rendering functionality is implemented.

## Planned layers

1. **Android host (Kotlin):** lifecycle, input, accessibility integration, and public Android API.
2. **JNI/C ABI boundary:** narrow, versioned, ownership-documented interface. Avoid passing complex Rust objects across FFI.
3. **Engine coordinator (Rust):** owns browsing contexts, documents, navigation state, and task scheduling.
4. **Platform services:** surfaces, input, clocks, file access, and process management behind traits.
5. **Web platform subsystems:** networking, HTML parsing, DOM, CSS, layout, paint, compositor, JavaScript, storage, and security.

## Planned subsystem boundaries

- `html`: html5ever provides HTML5 tokenization and tree construction; Minimalium owns the DOM tree sink. Add html5lib/WPT coverage before claiming full conformance.
- `dom`: document, node ownership, mutation, events, and traversal.
- `css`: tokenization, parsing, selectors, cascade, computed styles.
- `layout`: formatting contexts, intrinsic sizing, fragmentation, and viewport layout.
- `paint`: display lists, text, images, and invalidation.
- `compositor`: layers, frame scheduling, GPU resource lifetime, and presentation.
- `javascript`: ECMAScript runtime plus Web IDL bindings and host APIs.
- `network`: URL parsing, HTTP(S), redirects, TLS validation, fetch, and cancellation.
- `storage`: cookies, cache, local storage, and quota policy.
- `security`: origin model, permissions, CSP, sandboxing, site isolation, and safe IPC.

Subsystems should communicate through typed interfaces and explicit ownership. Avoid a single global mutable browser state.

## Threading and process model

Start with a small number of well-defined worker threads only after the synchronous baseline is correct and measured. Rendering workers must not share unsynchronized DOM mutation. The long-term design should isolate untrusted content in a separate Android process; threads alone are not a security boundary.

## Rendering pipeline

Target pipeline: HTML bytes -> tokens -> DOM -> style resolution -> layout tree -> display list -> rasterization -> compositor -> Android Surface. Each stage needs deterministic unit tests and WPT coverage before integration.

## JavaScript

The JavaScript runtime is a separate subsystem. DOM, timers, events, Fetch, promises integration, modules, and Web APIs are host responsibilities; embedding an ECMAScript interpreter alone does not create browser JavaScript compatibility.

## Dependency policy

Every dependency needs a recorded purpose, license, maintenance review, known vulnerability review, and replacement strategy. Prefer standards-oriented components with active maintenance. Approved initial dependencies: `url` for WHATWG URL parsing and `html5ever` for HTML5 tokenization/tree construction. The DOM sink is maintained by Minimalium. All other engine dependencies remain subject to review.
