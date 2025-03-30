use super::{atoms::ProjectCard, molecules::Timeline, organisms::TopNav};
use crate::filters;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub top_nav: TopNav<'a>,
    pub lang: &'a str,
    pub education_timeline: Timeline<'a>,
    pub work_experience_timeline: Timeline<'a>,
    pub side_projects: Vec<ProjectCard<'a>>,
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFound<'a> {
    pub title: &'a str,
    pub lang: &'a str,
}
