//! Standards-based navigation target parsing.
//!
//! This module validates and canonicalizes HTTP(S) URLs. It does not perform
//! DNS resolution, network requests, redirects, or origin security checks.

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationError {
    Empty,
    TooLong,
    UnsupportedScheme,
    InvalidUrl,
    CredentialsNotAllowed,
    MissingHost,
    InvalidCharacter,
}

const MAX_URL_BYTES: usize = 8 * 1024;

/// Parses an absolute HTTP(S) URL and returns its canonical serialization.
///
/// Credentials are rejected so navigation targets cannot silently carry
/// embedded usernames or passwords. Callers must still apply network policy,
/// redirect checks, DNS/IP filtering, and TLS validation before connecting.
pub fn normalize_navigation_target(target: &str) -> Result<String, NavigationError> {
    normalize_navigation_target_with_limit(target, MAX_URL_BYTES)
}

pub(crate) fn normalize_navigation_target_with_limit(
    target: &str,
    max_url_bytes: usize,
) -> Result<String, NavigationError> {
    if target.is_empty() {
        return Err(NavigationError::Empty);
    }
    if target.len() > max_url_bytes {
        return Err(NavigationError::TooLong);
    }
    if target.chars().any(char::is_control) {
        return Err(NavigationError::InvalidCharacter);
    }

    let parsed = Url::parse(target).map_err(|_| NavigationError::InvalidUrl)?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(NavigationError::UnsupportedScheme),
    }

    if !parsed.has_host() {
        return Err(NavigationError::MissingHost);
    }

    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(NavigationError::CredentialsNotAllowed);
    }

    Ok(parsed.to_string())
}

/// Validates an absolute HTTP(S) navigation target.
pub fn validate_navigation_target(target: &str) -> Result<(), NavigationError> {
    normalize_navigation_target(target).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_and_https() {
        assert!(validate_navigation_target("http://example.com").is_ok());
        assert!(validate_navigation_target("https://example.com").is_ok());
    }

    #[test]
    fn canonicalizes_scheme_host_and_default_port() {
        assert_eq!(
            normalize_navigation_target("HTTPS://EXAMPLE.COM:443/a").unwrap(),
            "https://example.com/a"
        );
    }

    #[test]
    fn supports_ipv6_hosts() {
        assert_eq!(
            normalize_navigation_target("http://[::1]:8080/").unwrap(),
            "http://[::1]:8080/"
        );
    }

    #[test]
    fn rejects_non_web_schemes() {
        assert_eq!(
            validate_navigation_target("data:text/plain,hello"),
            Err(NavigationError::UnsupportedScheme)
        );
        assert_eq!(
            validate_navigation_target("javascript:alert(1)"),
            Err(NavigationError::UnsupportedScheme)
        );
    }

    #[test]
    fn rejects_missing_or_malformed_hosts() {
        assert!(validate_navigation_target("https://?q=x").is_err());
        assert!(validate_navigation_target("https://").is_err());
        assert!(validate_navigation_target("https://example.com:99999").is_err());
    }

    #[test]
    fn rejects_embedded_credentials() {
        assert_eq!(
            validate_navigation_target("https://user:secret@example.com"),
            Err(NavigationError::CredentialsNotAllowed)
        );
    }

    #[test]
    fn rejects_control_characters_and_oversized_urls() {
        assert_eq!(
            validate_navigation_target("https://example.com/\nheader"),
            Err(NavigationError::InvalidCharacter)
        );
        assert_eq!(
            validate_navigation_target(&format!("https://example.com/{}", "a".repeat(8192))),
            Err(NavigationError::TooLong)
        );
    }

    #[test]
    fn rejects_empty_target() {
        assert_eq!(validate_navigation_target(""), Err(NavigationError::Empty));
    }
}
