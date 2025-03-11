use askama::Result;
use rust_i18n::t;

pub fn localize(key: &str, lang: &str) -> Result<String> {
    let res = t!(key, locale = lang);
    Ok(res.to_string())
}
