use askama::Result;
use rust_i18n::t;

pub fn localize(key: &str, _: &dyn askama::Values, lang: &str) -> Result<String> {
    let res = t!(key, locale = lang);
    Ok(res.to_string())
}
