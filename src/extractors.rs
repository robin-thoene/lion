use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::ACCEPT_LANGUAGE, request::Parts},
};

/// Extractor to get the desired language from the users request
pub struct ExtractUserLang(pub String);

impl<S> FromRequestParts<S> for ExtractUserLang
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the language from the http header, falling back to en as default
        let default_lang = "en";
        let lang = if let Some(lang) = parts.headers.get(ACCEPT_LANGUAGE) {
            if let Ok(lang) = lang.to_str() {
                if let Some(s) = lang.split_once(",") {
                    s.0
                } else {
                    default_lang
                }
            } else {
                default_lang
            }
        } else {
            default_lang
        };
        Ok(ExtractUserLang(lang.to_string()))
    }
}
