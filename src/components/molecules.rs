use super::atoms::TimelineElement;
use askama::Template;

#[derive(Template)]
#[template(path = "molecules/timeline.html")]
pub struct Timeline<'a> {
    pub timeline_elements: Vec<TimelineElement<'a>>,
}
