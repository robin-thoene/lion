use crate::{
    components::{
        atoms::{ProjectCard, TimelineElement},
        molecules::Timeline,
        organisms::TopNav,
        pages::{Index, NotFound},
    },
    extractors::ExtractUserLang,
};
use askama::Template;
use axum::{
    Form, Json,
    http::{StatusCode, header::SET_COOKIE},
    response::{AppendHeaders, Html, IntoResponse, Redirect},
};
use rust_i18n::{available_locales, t};
use serde::{Deserialize, Serialize};

/// Root page
pub async fn index(
    ExtractUserLang(lang): ExtractUserLang,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let akad_title = t!("education_akad_title", locale = lang);
    let it_training_title = t!("education_it_training", locale = lang);
    let auditor_dual_study_title = t!("education_auditor_dual_study", locale = lang);
    let a_levels_title = t!("education_a_levels", locale = lang);
    let enpal_title = t!("work_exp_enpal", locale = lang);
    let devdeer_title = t!("work_exp_devdeer", locale = lang);
    let avtest_title = t!("work_exp_avtest", locale = lang);
    let foej_title = t!("work_exp_foej", locale = lang);
    let pwc_title = t!("work_exp_pwc", locale = lang);
    let wiretui_desc = t!("side_proj_wiretui_desc", locale = lang);
    let pr_hub_desc = t!("side_proj_pr_hub_desc", locale = lang);
    let hk_desc = t!("side_proj_hk_desc", locale = lang);
    let daad_desc = t!("side_proj_daad_desc", locale = lang);
    let hotsave_desc = t!("side_proj_hotsave_desc", locale = lang);
    let fromsoft_desc = t!("side_proj_fromsoft_desc", locale = lang);
    let timewaste_desc = t!("side_proj_timewaste_desc", locale = lang);
    let lion_desc = t!("side_proj_lion_desc", locale = lang);
    // Build the index page template including dependencies
    let templ = Index {
        title: "Robin Thöne",
        top_nav: TopNav { lang: &lang },
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
                project_name: "wiretui",
                project_description: &wiretui_desc,
                github_url: Some("https://github.com/robin-thoene/wiretui"),
                website_url: None,
                package_url: None,
                technologies: vec!["Rust", "ratatui"],
            },
            ProjectCard {
                project_name: "pr-hub",
                project_description: &pr_hub_desc,
                github_url: Some("https://github.com/CodeNovum/pr-hub"),
                website_url: None,
                package_url: Some("https://github.com/CodeNovum/pr-hub/releases"),
                technologies: vec!["Rust", "Tauri", "React", "TypeScript"],
            },
            ProjectCard {
                project_name: "HarzerKurbelixe",
                project_description: &hk_desc,
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
                project_description: &daad_desc,
                github_url: Some("https://github.com/robin-thoene/default-azure-auth-debug"),
                website_url: None,
                package_url: Some(
                    "https://www.nuget.org/packages/RobinThoene.DefaultAzureAuthDebug.Console#readme-body-tab",
                ),
                technologies: vec!["Azure", "C#"],
            },
            ProjectCard {
                project_name: "hotsave",
                project_description: &hotsave_desc,
                github_url: Some("https://github.com/robin-thoene/hotsave"),
                website_url: None,
                package_url: Some("https://github.com/robin-thoene/hotsave/releases"),
                technologies: vec!["Rust"],
            },
            ProjectCard {
                project_name: "fromsoft-boss-checker",
                project_description: &fromsoft_desc,
                github_url: Some("https://github.com/robin-thoene/fromsoft-boss-checker"),
                website_url: Some("https://fromsoft-boss-checker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "timewaste-tracker",
                project_description: &timewaste_desc,
                github_url: Some("https://github.com/robin-thoene/timewaste-tracker"),
                website_url: Some("https://timewaste-tracker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "lion",
                project_description: &lion_desc,
                github_url: Some("https://github.com/robin-thoene/lion"),
                website_url: Some("https://robin-thoene.com/"),
                package_url: None,
                technologies: vec!["Rust", "Askama", "Axum", "Docker"],
            },
        ],
    };
    match templ.render() {
        Ok(html) => Ok(Html(html)),
        Err(_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Server error")),
    }
}

#[derive(Deserialize)]
pub struct SetLangPayload {
    lang: String,
}

/// HTTP endpoint to set the preferred language as cookie
pub async fn set_lang_cookie(Form(payload): Form<SetLangPayload>) -> impl IntoResponse {
    // Ensure only supported languages can be set
    let available = available_locales!();
    if !available.contains(&payload.lang.as_str()) {
        return axum::response::IntoResponse::into_response((
            StatusCode::BAD_REQUEST,
            "Provided language is not supported".to_string(),
        ));
    }
    // Set the cookie to the desired language and redirect to the main page
    // ensuring that the correct language is used
    (
        StatusCode::FOUND,
        AppendHeaders([(
            SET_COOKIE,
            format!("pref-lang={};Secure;HttpOnly;Path=/", payload.lang),
        )]),
        Redirect::to("/"),
    )
        .into_response()
}

#[derive(Serialize)]
pub struct HealthResponse {
    message: String,
}

/// Endpoint to check health status of the server
pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        message: "healthy".to_string(),
    })
}

/// Fallback route to serve when the requested resource is not found
pub async fn fallback(
    ExtractUserLang(lang): ExtractUserLang,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let not_found_title = t!("not_found_title", locale = lang);
    let not_found_templ = NotFound {
        title: &not_found_title,
        lang: &lang,
    };
    match not_found_templ.render() {
        Ok(html) => Ok((StatusCode::NOT_FOUND, Html(html))),
        Err(_err) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Server error")),
    }
}
