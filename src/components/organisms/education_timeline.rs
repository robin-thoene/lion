use leptos::*;
use leptos_icons::*;

use crate::components::{atoms::timeline_element::TimelineEntry, molecules::timeline::Timeline};
use crate::i18n::*;

#[component]
pub fn EducationTimeline() -> impl IntoView {
    let i18n = use_i18n();
    let data = vec![
        TimelineEntry {
            title: view! {<>{t!(i18n, education_university_business_informatics)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "AKAD University"
                    <a class="ml-4" aria-label="AKAD University" href="https://www.akad.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2020,
            end_year: None,
            content: None,
        },
        TimelineEntry {
            title: view! {<>{t!(i18n, apprenticeship_title_software_developer)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "Berufsbildende Schulen „Otto von Guericke“"
                    <a class="ml-4" aria-label="Berufsbildende Schulen „Otto von Guericke“" href="https://bbsovg-magdeburg.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineEntry {
            title: view! {<>{t!(i18n, dual_study_auditor)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "Duale Hochschule Baden-Württemberg"
                    <a class="ml-4" aria-label="Duale Hochschule Baden-Württemberg" href="https://www.dhbw.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
        TimelineEntry {
            title: view! {<>{t!(i18n, education_a_levels)}</>}.into_view(),
            subtitle: view! {
                <div class="flex">
                    "Berufsbildende Schulen 1 Goslar -Am Stadtgarten-"
                    <a class="ml-4" aria-label="Berufsbildende Schulen 1 Goslar -Am Stadtgarten-" href="http://www.bbs1goslar.de/" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FiIcon::FiExternalLink) />
                    </a>
                </div>
            }.into_view(),
            start_year: 2012,
            end_year: Some(2015),
            content: None,
        },
    ];
    view! { <Timeline content=data/> }
}

