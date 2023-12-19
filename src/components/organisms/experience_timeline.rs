use leptos::*;
use leptos_icons::*;

use crate::components::molecules::timeline::{Timeline, TimelineElement};
use crate::i18n::*;

#[component]
pub fn ExperienceTimeline() -> impl IntoView {
    let i18n = use_i18n();
    let data = vec![
        TimelineElement {
            title: view! {<>Senior {t!(i18n, work_title_software_developer)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "Enpal"
                    <a class="ml-4" aria-label="Enpal" href="https://www.enpal.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2023,
            end_year: None,
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, work_title_software_developer)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "DEVDEER GmbH"
                    <a class="ml-4" aria-label="DEVDEER GmbH" href="https://devdeer.com/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2020,
            end_year: Some(2023),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, apprenticeship_title_software_developer)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "AV-TEST GmbH"
                    <a class="ml-4" aria-label="AV-TEST GmbH" href="https://www.av-test.org/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, work_title_ecological_volunteer)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt"
                    <a class="ml-4" aria-label="Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt" href="https://www.sunk-lsa.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2016,
            end_year: Some(2017),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, dual_study_auditor)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "PricewaterhouseCoopers GmbH"
                    <a class="ml-4" aria-label="PricewaterhouseCoopers GmbH" href="https://www.pwc.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
    ];

    view! { <Timeline content=data/> }
}

