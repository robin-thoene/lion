use askama::Template;

#[derive(Template)]
#[template(path = "organisms/top_nav.html")]
pub struct TopNav<'a> {
    pub lang: &'a str,
}
