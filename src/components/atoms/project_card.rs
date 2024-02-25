use leptos::*;
use leptos_icons::*;

pub struct Project {
    pub name: String,
    pub description: View,
    pub github_url: Option<String>,
    pub website_url: Option<String>,
    pub used_technologies: Vec<String>,
}

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="flex flex-col border border-slate-400 dark:border-slate-100 rounded-lg p-3 gap-3 max-w-md">
            <div class="flex justify-between">
                <div class="mr-8 font-bold">{project.name.clone()}</div>
                <div class="flex gap-3">
                    {if let Some(url) = project.github_url {
                        view! {
                            <a aria-label=url.clone() href=url target="_blank" rel="noreferrer">
                                <Icon class="h-5 w-5" icon=icondata::FaGithubBrands/>
                            </a>
                        }
                            .into_view()
                    } else {
                        view! { <></> }.into_view()
                    }}
                    {if let Some(url) = project.website_url {
                        view! {
                            <a aria-label=url.clone() href=url target="_blank" rel="noreferrer">
                                <Icon class="h-5 w-5" icon=icondata::FiExternalLink/>
                            </a>
                        }
                            .into_view()
                    } else {
                        view! { <></> }.into_view()
                    }}

                </div>
            </div>
            <div class="flex flex-row flex-wrap gap-3">
                {project
                    .used_technologies
                    .into_iter()
                    .map(|t| {
                        view! {
                            <div class="bg-primary text-black w-max rounded-2xl px-3 py-1">{t}</div>
                        }
                    })
                    .collect_view()}
            </div>
            <div>
                <p class="text-justify">{project.description}</p>
            </div>
        </div>
    }
}

