//! UI helpers for rendering errors without panicking.

use super::view::View;

/// Render `Ok` content or a fallback from a `Result` (e.g. mapped server action output).
pub fn error_boundary<E: AsRef<str>>(
    result: Result<View, E>,
    fallback: impl FnOnce(String) -> View,
) -> View {
    match result {
        Ok(v) => v,
        Err(e) => fallback(e.as_ref().to_string()),
    }
}
