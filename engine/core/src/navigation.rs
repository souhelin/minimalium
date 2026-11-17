//! Deliberately small URL gate for the foundation milestone.
//! This is not a standards-compliant URL parser. Replace it with a vetted,
//! standards-conformant URL implementation before network loading is added.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationError {
    Empty,
    TooLong,
    UnsupportedScheme,
    MissingAuthority,
    InvalidCharacter,
}

pub fn validate_navigation_target(target: &str) -> Result<(), NavigationError> {
    if target.is_empty() {
        return Err(NavigationError::Empty);
    }
    if target.len() > 8 * 1024 {
        return Err(NavigationError::TooLong);
    }
    if target.chars().any(char::is_control) {
        return Err(NavigationError::InvalidCharacter);
    }

    let Some((scheme, remainder)) = target.split_once("://") else {
        return Err(NavigationError::UnsupportedScheme);
    };

    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return Err(NavigationError::UnsupportedScheme);
    }

    let authority = remainder.split(['/', '?', '#']).next().unwrap_or_default();
    if authority.is_empty() || authority.starts_with(':') || authority.ends_with('@') {
        return Err(NavigationError::MissingAuthority);
    }

    Ok(())
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
    fn rejects_non_web_schemes() {
        assert_eq!(
            validate_navigation_target("data:text/plain,hello"),
            Err(NavigationError::UnsupportedScheme)
        );
    }
}
