//! Luffy UI: Embedded frontend assets, Persian/English i18n catalogs, and theme assets.

pub mod i18n;
pub mod templates;

pub use i18n::I18nManager;
pub use templates::{PanelTemplates, UserViewModel};

pub const DEFAULT_LANG: &str = "fa";
pub const FALLBACK_LANG: &str = "en";
