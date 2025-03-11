use super::{atoms::ProjectCard, molecules::Timeline};
use crate::filters;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct Index<'a> {
    pub lang: &'a str,
    pub education_timeline: Timeline<'a>,
    pub work_experience_timeline: Timeline<'a>,
    pub side_projects: Vec<ProjectCard<'a>>,
}
