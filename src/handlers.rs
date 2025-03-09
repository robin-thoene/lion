use crate::components::{
    atoms::{ProjectCard, TimelineElement},
    molecules::Timeline,
    pages::Index,
};
use askama_axum::IntoResponse as _;
use axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    let templ = Index {
        education_timeline: Timeline {
            timeline_elements: vec![
                TimelineElement {
                    time_display: "2020",
                    title: "B. Sc. Business Informatics",
                    subtitle: "AKAD University",
                    link: Some("https://www.akad.de/"),
                },
                TimelineElement {
                    time_display: "2017 - 2020",
                    title: "Training as an IT specialist - Application Development",
                    subtitle: "Berufsbildende Schulen „Otto von Guericke“",
                    link: Some("https://bbsovg-magdeburg.de/"),
                },
                TimelineElement {
                    time_display: "2015 - 2016",
                    title: "Dual study - Auditor",
                    subtitle: "Duale Hochschule Baden-Württemberg",
                    link: Some("https://www.dhbw.de/"),
                },
                TimelineElement {
                    time_display: "2012 - 2015",
                    title: "A-Levels",
                    subtitle: "Berufsbildende Schulen 1 Goslar -Am Stadtgarten-",
                    link: Some("http://www.bbs1goslar.de/"),
                },
            ],
        },
        work_experience_timeline: Timeline {
            timeline_elements: vec![
                TimelineElement {
                    time_display: "2023",
                    title: "Senior Software Developer",
                    subtitle: "Enpal",
                    link: Some("https://www.enpal.de/"),
                },
                TimelineElement {
                    time_display: "2020 - 2023",
                    title: "Software Developer",
                    subtitle: "DEVDEER GmbH",
                    link: Some("https://devdeer.com/"),
                },
                TimelineElement {
                    time_display: "2017 - 2020",
                    title: "Training as an IT specialist - Application Development",
                    subtitle: "AV-TEST GmbH",
                    link: Some("https://www.av-test.org/"),
                },
                TimelineElement {
                    time_display: "2016 - 2017",
                    title: "Voluntary Ecological Year",
                    subtitle: "Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt",
                    link: Some("https://www.sunk-lsa.de/"),
                },
                TimelineElement {
                    time_display: "2015 - 2016",
                    title: "Dual study - Auditor",
                    subtitle: "PricewaterhouseCoopers GmbH",
                    link: Some("https://www.pwc.de/"),
                },
            ],
        },
        side_projects: vec![
            ProjectCard {
                project_name: "pr-hub",
                project_description: "A central entry point to view pull requests across multiple version control provider. Currently this targets only the platform Azure DevOps and enables viewing pull request related data across multiple organizations, projects and repositories.",
                github_url: Some("https://github.com/CodeNovum/pr-hub"),
                website_url: None,
                package_url: Some("https://github.com/CodeNovum/pr-hub/releases"),
                technologies: vec!["Rust", "Tauri", "React", "TypeScript"],
            },
            ProjectCard {
                project_name: "HarzerKurbelixe",
                project_description: "This was the first side project to learn technologies for my first full-time job as a developer. In addition, this self-created bike blog has been raising funds for charity for the past few years.",
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
                project_description: "A simple CLI tool to test if the method \"DefaultAzureCredential\" from the package \"Azure.Identity\" can pickup the authentication details from the local machine and retrieve a token.",
                github_url: Some("https://github.com/robin-thoene/default-azure-auth-debug"),
                website_url: None,
                package_url: Some(
                    "https://www.nuget.org/packages/RobinThoene.DefaultAzureAuthDebug.Console#readme-body-tab",
                ),
                technologies: vec!["Azure", "C#"],
            },
            ProjectCard {
                project_name: "hotsave",
                project_description: "A simple CLI tool to immediately backup and restore a backup for a single file using global hotkeys.",
                github_url: Some("https://github.com/robin-thoene/hotsave"),
                website_url: None,
                package_url: Some("https://github.com/robin-thoene/hotsave/releases"),
                technologies: vec!["Rust"],
            },
            ProjectCard {
                project_name: "fromsoft-boss-checker",
                project_description: "A small application to track the progress in FromSoftware games.",
                github_url: Some("https://github.com/robin-thoene/fromsoft-boss-checker"),
                website_url: Some("https://fromsoft-boss-checker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "timewaste-tracker",
                project_description: "Project to try out \"recharts\" and the Steam API by graphically visualizing the playing time of some Steam users within the last two weeks.",
                github_url: Some("https://github.com/robin-thoene/timewaste-tracker"),
                website_url: Some("https://timewaste-tracker.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "tailwind-wysiwyg-editor",
                project_description: "\"What You See Is What You Get\" editor using \"Tailwind CSS\" with Markdown as target format.",
                github_url: Some("https://github.com/robin-thoene/tailwind-wysiwyg-editor"),
                website_url: Some("https://tailwind-wysiwyg-editor.vercel.app/"),
                package_url: None,
                technologies: vec!["TypeScript", "React", "Next.js"],
            },
            ProjectCard {
                project_name: "lion",
                project_description: "This project contains the source code of this website.",
                github_url: Some("https://github.com/robin-thoene/lion"),
                website_url: Some("https://robin-thoene.com/"),
                package_url: None,
                technologies: vec!["Rust", "Askama", "Axum", "Docker"],
            },
        ],
    };
    templ.into_response()
}
