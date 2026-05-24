//! Redirect helpers for `#[submit]` and `#[server]` handlers (PRG / post-action navigation).

use crate::core::{Result, ResumaError};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Redirect as AxumRedirect, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Redirect target returned from a submit or server action.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Redirect {
    pub to: String,
}

impl Redirect {
    pub fn to(path: impl Into<String>) -> Self {
        Self { to: path.into() }
    }

    pub fn into_response(self) -> axum::response::Response {
        redirect_response(&self.to)
    }
}

impl Serialize for Redirect {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Redirect", 1)?;
        s.serialize_field("redirect", &self.to)?;
        s.end()
    }
}

/// Build a redirect value for `#[submit]` / `#[server]` return types.
pub fn redirect(path: impl Into<String>) -> Redirect {
    Redirect::to(path)
}

/// Extract a same-origin redirect path from a serialized handler result.
pub fn extract_redirect(value: &Value) -> Option<String> {
    let path = value.get("redirect")?.as_str()?;
    validate_redirect_path(path).ok().map(str::to_string)
}

/// Reject open redirects — only root-relative paths are allowed.
pub fn validate_redirect_path(path: &str) -> Result<&str> {
    if !path.starts_with('/') || path.starts_with("//") {
        return Err(ResumaError::Other(format!(
            "invalid redirect path `{path}` (must start with `/`, not `//`)"
        )));
    }
    Ok(path)
}

/// HTTP 303 See Other — standard PRG response for form submits without JavaScript.
pub fn redirect_response(path: &str) -> Response {
    match validate_redirect_path(path) {
        Ok(loc) => AxumRedirect::to(loc).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}

/// Attach `Location` on JSON responses when a redirect hint is present.
pub fn redirect_json_headers(path: &str) -> Option<[(header::HeaderName, String); 1]> {
    validate_redirect_path(path)
        .ok()
        .map(|loc| [(header::LOCATION, loc.to_string())])
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_root_relative_paths() {
        assert_eq!(validate_redirect_path("/items").unwrap(), "/items");
        assert_eq!(
            validate_redirect_path("/items/42?created=1").unwrap(),
            "/items/42?created=1"
        );
    }

    #[test]
    fn rejects_open_redirects() {
        assert!(validate_redirect_path("https://evil.test").is_err());
        assert!(validate_redirect_path("//evil.test").is_err());
    }

    #[test]
    fn extracts_redirect_field() {
        assert_eq!(
            extract_redirect(&json!({ "redirect": "/done" })),
            Some("/done".into())
        );
        assert_eq!(extract_redirect(&json!({ "ok": true })), None);
    }
}
