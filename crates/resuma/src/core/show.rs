//! Conditional rendering — Leptos-style `Show` without a separate macro system.

use super::view::{Child, Fragment, View};

/// Render `children` when `when` is true, otherwise `fallback` or nothing.
pub fn show(when: bool, children: Vec<Child>, fallback: Option<View>) -> View {
    if when {
        View::Fragment(Fragment { children })
    } else if let Some(fb) = fallback {
        fb
    } else {
        View::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::view::{Child, View};
    use super::show;

    #[test]
    fn show_renders_children_when_true() {
        let v = show(true, vec![Child::Text("hi".into())], None);
        assert!(matches!(v, View::Fragment(_)));
    }

    #[test]
    fn show_renders_fallback_when_false() {
        let fb = View::text("no");
        let v = show(false, vec![], Some(fb));
        assert!(matches!(v, View::Text(s) if s == "no"));
    }
}
