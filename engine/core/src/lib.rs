//! Minimalium's foundational native core.
//!
//! This crate is not a browser engine yet. In particular, it does not fetch,
//! parse, execute, or render web content.

mod navigation;

use std::ffi::{c_char, CStr};

pub use navigation::{validate_navigation_target, NavigationError};

/// Stable engine configuration for the initial API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    /// Maximum URL length accepted by the navigation API.
    pub max_url_bytes: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_url_bytes: 8 * 1024,
        }
    }
}

/// Minimal engine state. Later milestones will add document and session types.
#[derive(Debug, Default)]
pub struct Engine {
    last_navigation_target: Option<String>,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates and records a target. This does not perform network I/O.
    pub fn navigate(&mut self, target: &str) -> Result<(), NavigationError> {
        validate_navigation_target(target)?;
        self.last_navigation_target = Some(target.to_owned());
        Ok(())
    }

    pub fn last_navigation_target(&self) -> Option<&str> {
        self.last_navigation_target.as_deref()
    }
}

const STATUS_OK: i32 = 0;
const STATUS_NULL_ARGUMENT: i32 = 1;
const STATUS_INVALID_UTF8: i32 = 2;
const STATUS_INVALID_TARGET: i32 = 3;

/// Returns the C ABI version. Increment only with an explicitly documented
/// compatibility change.
#[no_mangle]
pub extern "C" fn minimalium_engine_abi_version() -> u32 {
    1
}

/// Creates an engine instance. The returned pointer must be released with
/// `minimalium_engine_destroy`.
#[no_mangle]
pub extern "C" fn minimalium_engine_create() -> *mut Engine {
    Box::into_raw(Box::new(Engine::new()))
}

/// Destroys an instance created by `minimalium_engine_create`.
///
/// # Safety
/// `engine` must be null or a live pointer returned by
/// `minimalium_engine_create`, and must not have been destroyed already.
#[no_mangle]
pub unsafe extern "C" fn minimalium_engine_destroy(engine: *mut Engine) {
    if !engine.is_null() {
        // SAFETY: guaranteed by this function's caller contract.
        drop(unsafe { Box::from_raw(engine) });
    }
}

/// Validates and records a navigation target. No request is sent.
///
/// # Safety
/// `engine` must point to a live engine instance. `target` must point to a
/// valid, NUL-terminated C string for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn minimalium_engine_navigate(
    engine: *mut Engine,
    target: *const c_char,
) -> i32 {
    if engine.is_null() || target.is_null() {
        return STATUS_NULL_ARGUMENT;
    }

    // SAFETY: pointer validity and NUL termination are caller requirements.
    let target = unsafe { CStr::from_ptr(target) };
    let Ok(target) = target.to_str() else {
        return STATUS_INVALID_UTF8;
    };

    // SAFETY: engine was checked for null; caller guarantees it is live and
    // exclusive access is required by the C ABI contract.
    match unsafe { &mut *engine }.navigate(target) {
        Ok(()) => STATUS_OK,
        Err(_) => STATUS_INVALID_TARGET,
    }
}

/// Returns a stable status code for an empty pointer check in foreign callers.
#[no_mangle]
pub extern "C" fn minimalium_engine_status_ok() -> i32 {
    STATUS_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_engine_has_no_navigation_target() {
        let engine = Engine::new();
        assert_eq!(engine.last_navigation_target(), None);
    }

    #[test]
    fn navigation_records_valid_https_target() {
        let mut engine = Engine::new();
        engine.navigate("https://example.com/path").unwrap();
        assert_eq!(
            engine.last_navigation_target(),
            Some("https://example.com/path")
        );
    }

    #[test]
    fn navigation_rejects_unsupported_schemes() {
        let mut engine = Engine::new();
        assert!(engine.navigate("file:///etc/passwd").is_err());
        assert!(engine.navigate("javascript:alert(1)").is_err());
    }

    #[test]
    fn navigation_rejects_missing_host() {
        let mut engine = Engine::new();
        assert!(engine.navigate("https:///missing-host").is_err());
        assert!(engine.navigate("https://").is_err());
    }

    #[test]
    fn navigation_rejects_control_characters() {
        let mut engine = Engine::new();
        assert!(engine.navigate("https://example.com/\nheader").is_err());
    }
}
