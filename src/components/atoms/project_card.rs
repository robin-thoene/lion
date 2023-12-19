use leptos::*;

pub struct Project {
    pub name: String,
    pub description: String,
    pub github_url: Option<String>,
    pub website_url: Option<String>,
}

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="flex flex-col border border-slate-400 dark:border-slate-100 rounded-lg p-3 gap-3 max-w-max">
            <div class="flex justify-between">
                <div>{project.name}</div>
                <div class="flex gap-3">
                    <div>GH LINK</div>
                    <div>WEBSITE URL</div>
                </div>
            </div>
            <div class="flex gap-3">
                <div>Tech 1</div>
                <div>Tech 2</div>
                <div>Tech 3</div>
            </div>
            <div>{project.description}</div>
        </div>
    }
}

