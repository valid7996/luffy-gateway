//! Luffy UI: Embedded frontend assets, Persian/English i18n catalogs, and theme assets.

pub mod i18n;
pub mod templates;

pub use i18n::I18nManager;
pub use templates::{PanelTemplates, UserViewModel};

pub const DEFAULT_LANG: &str = "fa";
pub const FALLBACK_LANG: &str = "en";

/// Escapes user-controlled text for safe interpolation into HTML documents.
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html_neutralizes_injection() {
        assert_eq!(escape_html("Ali"), "Ali");
        assert_eq!(
            escape_html(r#"<img src=x onerror="alert(1)">"#),
            "&lt;img src=x onerror=&quot;alert(1)&quot;&gt;"
        );
    }
}
