use axum::{
    extract::FromRequestParts,
    http::{
        StatusCode,
        header::{ACCEPT_LANGUAGE, COOKIE},
        request::Parts,
    },
};
use axum_extra::extract::cookie::Cookie;
use rust_i18n::available_locales;

/// Extractor to get the desired language from the users request
pub struct ExtractUserLang(pub String);

impl<S> FromRequestParts<S> for ExtractUserLang
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let available = available_locales!();
        // Try to get the preferred language from a cookie
        let cookie_header = parts.headers.get(COOKIE);
        if let Some(cookie_header) = cookie_header {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in Cookie::split_parse(cookie_str).flatten() {
                    let (name, value) = cookie.name_value();
                    if name == "pref-lang" && available.contains(&value) {
                        return Ok(ExtractUserLang(value.to_string()));
                    }
                }
            }
        }
        // Get the language from the http header, falling back to en as default
        let lang = parts
            .headers
            .get(ACCEPT_LANGUAGE)
            .and_then(|header_val| header_val.to_str().ok())
            .and_then(|header_str| header_str.split_once(",").map(|s| s.0))
            .filter(|&s| available.contains(&s))
            .unwrap_or("en");
        Ok(ExtractUserLang(lang.to_string()))
    }
}
