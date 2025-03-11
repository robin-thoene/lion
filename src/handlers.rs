use crate::{
    components::{
        atoms::{ProjectCard, TimelineElement},
        molecules::Timeline,
        pages::Index,
    },
    extractors::ExtractUserLang,
};
use askama_axum::IntoResponse as _;
use axum::response::IntoResponse;
use rust_i18n::t;

pub async fn index(ExtractUserLang(lang): ExtractUserLang) -> impl IntoResponse {
    let akad_title = t!("education_akad_title", locale = lang);
    let it_training_title = t!("education_it_training", locale = lang);
    let auditor_dual_study_title = t!("education_auditor_dual_study", locale = lang);
    let a_levels_title = t!("education_a_levels", locale = lang);
    let enpal_title = t!("work_exp_enpal", locale = lang);
    let devdeer_title = t!("work_exp_devdeer", locale = lang);
    let avtest_title = t!("work_exp_avtest", locale = lang);
    let foej_title = t!("work_exp_foej", locale = lang);
    let pwc_title = t!("work_exp_pwc", locale = lang);
    let pr_hub_title = t!("side_proj_pr_hub_desc", locale = lang);
    let hk_title = t!("side_proj_hk_desc", locale = lang);
    let daad_title = t!("side_proj_daad_desc", locale = lang);
    let hotsave_title = t!("side_proj_hotsave_desc", locale = lang);
    let fromsoft_title = t!("side_proj_fromsoft_desc", locale = lang);
    let timewaste_title = t!("side_proj_timewaste_desc", locale = lang);
    let wysiwyg_title = t!("side_proj_wysiwyg_desc", locale = lang);
    let lion_title = t!("side_proj_lion_desc", locale = lang);
    // Build the index page template including dependencies
    let templ = Index {
        lang: &lang,
        education_timeline: Timeline {
            timeline_elements: vec![
                TimelineElement {
                    time_display: "2020",
                    title: &akad_title,
                    subtitle: "AKAD University",
                    link: Some("https://www.akad.de/"),
                },
                TimelineElement {
                    time_display: "2017 - 2020",
                    title: &it_training_title,
                    subtitle: "Berufsbildende Schulen „Otto von Guericke“",
                    link: Some("https://bbsovg-magdeburg.de/"),
                },
                TimelineElement {
                    time_display: "2015 - 2016",
                    title: &auditor_dual_study_title,
                    subtitle: "Duale Hochschule Baden-Württemberg",
                    link: Some("https://www.dhbw.de/"),
                },
                TimelineElement {
                    time_display: "2012 - 2015",
                    title: &a_levels_title,
                    subtitle: "Berufsbildende Schulen 1 Goslar -Am Stadtgarten-",
                    link: Some("http://www.bbs1goslar.de/"),
                },
            ],
        },
        work_experience_timeline: Timeline {
            timeline_elements: vec![
                TimelineElement {
                    time_display: "2023",
                    title: &enpal_title,
                    subtitle: "Enpal",
                    link: Some("https://www.enpal.de/"),
                },
                TimelineElement {
                    time_display: "2020 - 2023",
                    title: &devdeer_title,
                    subtitle: "DEVDEER GmbH",
                    link: Some("https://devdeer.com/"),
                },
                TimelineElement {
                    time_display: "2017 - 2020",
                    title: &avtest_title,
                    subtitle: "AV-TEST GmbH",
                    link: Some("https://www.av-test.org/"),
                },
                TimelineElement {
                    time_display: "2016 - 2017",
                    title: &foej_title,
                    subtitle: "Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt",
                    link: Some("https://www.sunk-lsa.de/"),
                },
                TimelineElement {
                    time_display: "2015 - 2016",
                    title: &pwc_title,
                    subtitle: "PricewaterhouseCoopers GmbH",
                    link: Some("https://www.pwc.de/"),
                },
            ],
        },
        side_projects: vec![
            ProjectCard {
                project_name: "pr-hub",
                project_description: &pr_hub_title,
                github_url: Some("https://github.com/CodeNovum/pr-hub"),
                website_url: None,
                package_url: Some("https://github.com/CodeNovum/pr-hub/releases"),
                technologies: vec!["Rust", "Tauri", "React", "TypeScript"],
            },
            ProjectCard {
                project_name: "HarzerKurbelixe",
                project_description: &hk_title,
                github_url: None,
                website_url: Some("https://harzerkurbelixe.de/"),
                package_url: None,
                technologies: vec![
                    "Azure",
                    "C#",
                    "ASP.NET Core",
                    "MySQL",
                    "React",
                    "TypeScript",
                    "Docker",
                ],
            },
            ProjectCard {
                project_name: "default-azure-auth-debug",
                project_description: &daad_title,
                github_url: Some("https://github.com/robin-thoene/default-azure-auth-debug"),
                website_url: None,
                package_url: Some(
                    "https://www.nuget.org/packages/RobinThoene.DefaultAzureAuthDebug.Console#readme-body-tab",
                ),
                technologies: vec!["Azure", "C#"],
            },
            ProjectCard {
                project_name: "hotsave",
                project_description: &hotsave_title,
                github_url: Some("https://github.com/robin-thoene/hotsave"),
                website_url: None,
                package_url: Some("https://github.com/robin-thoene/hotsave/releases"),
                technologies: vec!["Rust"],
            },
            ProjectCard {
                project_name: "fromsoft-boss-checker",
                project_description: &fromsoft_title,
                github_url: Some("https://github.com/robin-thoene/fromsoft-boss-checker"),
                website_url: Some("https://fromsoft-boss-checker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "timewaste-tracker",
                project_description: &timewaste_title,
                github_url: Some("https://github.com/robin-thoene/timewaste-tracker"),
                website_url: Some("https://timewaste-tracker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "tailwind-wysiwyg-editor",
                project_description: &wysiwyg_title,
                github_url: Some("https://github.com/robin-thoene/tailwind-wysiwyg-editor"),
                website_url: Some("https://tailwind-wysiwyg-editor.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "lion",
                project_description: &lion_title,
                github_url: Some("https://github.com/robin-thoene/lion"),
                website_url: Some("https://robin-thoene.com/"),
                package_url: None,
                technologies: vec!["Rust", "Askama", "Axum", "Docker"],
            },
        ],
    };
    templ.into_response()
}
