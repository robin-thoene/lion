use leptos::*;
use leptos_icons::*;

use crate::components::atoms::{Package, Project, ProjectCard};
use crate::i18n::*;

#[component]
pub fn SideProjects() -> impl IntoView {
    let i18n = use_i18n();

    let data = vec![
        Project {
            name: String::from("pr-hub"),
            description: view! {<>{t!(i18n, project_description_prhub)}</>}.into_view(),
            github_url: Some(String::from("https://github.com/CodeNovum/pr-hub")),
            website_url: None,
            used_technologies: vec![
                String::from("Rust"),
                String::from("Tauri"),
                String::from("React"),
                String::from("TypeScript"),
            ],
            published_package: None
        },
        Project {
            name: String::from("HarzerKurbelixe"),
            description: view! {<>{t!(i18n, project_description_harzerkurbelixe)}</>}.into_view(),
            github_url: None,
            website_url: Some(String::from("https://harzerkurbelixe.de")),
            used_technologies: vec![
                String::from("Azure"),
                String::from("C#"),
                String::from("ASP.NET Core"),
                String::from("MySQL"),
                String::from("React"),
                String::from("TypeScript"),
                String::from("Docker"),
            ],
            published_package: None
        },
        Project {
            name: String::from("default-azure-auth-debug"),
            description: view! {<>{t!(i18n,project_description_default_azure_auth_debug)}</>}
                .into_view(),
            github_url: Some(String::from(
                "https://github.com/robin-thoene/default-azure-auth-debug",
            )),
            website_url: None,
            used_technologies: vec![String::from("Azure"), String::from("C#")],
            published_package: Some(Package {
                url: String::from("https://www.nuget.org/packages/RobinThoene.DefaultAzureAuthDebug.Console#readme-body-tab"),
                icon: view!{<Icon class="h-5 w-5" icon=icondata::SiNuget/>}
            })
        },
        Project {
            name: String::from("hotsave"),
            description: view! {<>{t!(i18n, project_description_hotsave)}</>}.into_view(),
            github_url: Some(String::from("https://github.com/robin-thoene/hotsave")),
            website_url: None,
            used_technologies: vec![
                String::from("Rust"),
            ],
            published_package: None
        },
        Project {
            name: String::from("fromsoft-boss-checker"),
            description: view! {<>{t!(i18n, project_description_fromsoft_boss_checker)}</>}
                .into_view(),
            github_url: Some(String::from(
                "https://github.com/robin-thoene/fromsoft-boss-checker",
            )),
            website_url: Some(String::from("https://fromsoft-boss-checker.vercel.app")),
            used_technologies: vec![
                String::from("TypeScript"),
                String::from("React"),
                String::from("Next.js"),
            ],
            published_package: None
        },
        Project {
            name: String::from("timewaste-tracker"),
            description: view! {<>{t!(i18n, project_description_timewaste_tracker)}</>}.into_view(),
            github_url: Some(String::from(
                "https://github.com/robin-thoene/timewaste-tracker",
            )),
            website_url: Some(String::from("https://timewaste-tracker.vercel.app")),
            used_technologies: vec![
                String::from("TypeScript"),
                String::from("React"),
                String::from("Next.js"),
            ],
            published_package: None
        },
        Project {
            name: String::from("tailwind-wysiwyg-editor"),
            description: view! {<>{t!(i18n, project_description_tailwind_wysiwyg_editor)}</>}
                .into_view(),
            github_url: Some(String::from(
                "https://github.com/robin-thoene/tailwind-wysiwyg-editor",
            )),
            website_url: Some(String::from("https://tailwind-wysiwyg-editor.vercel.app")),
            used_technologies: vec![
                String::from("TypeScript"),
                String::from("React"),
                String::from("Next.js"),
            ],
            published_package: None
        },
        Project {
            name: String::from("lion"),
            description: view! {<>{t!(i18n, project_description_lion)}</>}.into_view(),
            github_url: Some(String::from("https://github.com/robin-thoene/lion")),
            website_url: Some(String::from("https://robin-thoene.com")),
            used_technologies: vec![
                String::from("Rust"),
                String::from("Leptos"),
                String::from("Docker"),
            ],
            published_package: None
        }
    ];

    view! {
        <div class="flex flex-row justify-center w-full flex-wrap gap-6">
            {data.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect_view()}
        </div>
    }
}

