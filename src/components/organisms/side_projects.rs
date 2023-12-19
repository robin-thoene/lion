use leptos::*;

use crate::components::atoms::project_card::{Project, ProjectCard};

#[component]
pub fn SideProjects() -> impl IntoView {
    let data = vec![Project {
        name: "Project 1".to_string(),
        description: "Description 1".to_string(),
        github_url: None,
        website_url: None,
    }];
    view! { <div>{data.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}</div> }
}

