use askama::Template;

#[derive(Template)]
#[template(path = "atoms/timeline_element.html")]
pub struct TimelineElement<'a> {
    pub time_display: &'a str,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub link: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "atoms/project_card.html")]
pub struct ProjectCard<'a> {
    pub project_name: &'a str,
    pub project_description: &'a str,
    pub github_url: Option<&'a str>,
    pub package_url: Option<&'a str>,
    pub website_url: Option<&'a str>,
    pub technologies: Vec<&'a str>,
}
