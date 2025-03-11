use rust_i18n::i18n;

pub mod components;
pub mod extractors;
pub mod filters;
pub mod handlers;

i18n!("locales", fallback = "en");
