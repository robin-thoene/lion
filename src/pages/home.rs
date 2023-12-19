use leptos::*;
use leptos_icons::*;
use leptos_meta::*;

use crate::components::timeline::{Timeline, TimelineElement};
use crate::components::top_nav::TopNav;
use crate::i18n::*;

#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();

    let experience_timeline_elements = vec![
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

    let education_timeline_elements = vec![
        TimelineElement {
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
        TimelineElement {
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
        TimelineElement {
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
        TimelineElement {
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

    view! {
        <Title text="Robin Thöne"/>
        <Meta name="description" content="Robin Thöne - Software Developer"/>
        <Meta name="robots" content="index, follow"/>
        <Meta property="og:title" content="Robin Thöne"/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:description" content="Robin Thöne - Software Developer"/>
        <Meta property="og:image" content="https://robin-thoene.com/open-graph.jpg"/>
        <Meta property="og:locale" content="en_US"/>
        <Meta property="og:locale:alternate" content="de_DE"/>
        <Meta
            name="google-site-verification"
            content="6E4fkyF9xXTXSHWCY2loZyjTPYV3DS6rMEEXRBuW0TU"
        />
        <Link rel="preload" as_="image" href="/profile.webp"/>
        <main class="w-screen max-w-screen-2xl p-6">
            <TopNav/>
            <div class="flex flex-col gap-20">
                <div class="flex justify-center">
                    <img
                        alt=t!(i18n, profile_image_alt_text)
                        src="/profile.webp"
                        class="h-64 w-64 object-cover rounded-full overflow-hidden"
                    />
                </div>
                <h1 class="text-center">Robin Thöne</h1>
                <div class="flex flex-col md:flex-row w-full gap-6">
                    <div class="flex flex-col flex-1">
                        <h2 class="text-center">{t!(i18n, education_headline)}</h2>
                        <p>
                            <Timeline content=education_timeline_elements/>
                        </p>
                    </div>
                    <div class="flex flex-col flex-1">
                        <h2 class="text-center">{t!(i18n, work_experience_headline)}</h2>
                        <p>
                            <Timeline content=experience_timeline_elements/>
                        </p>
                    </div>
                </div>
                // TODO: Implement this
                // <div>
                // <h2 class="text-center">{t!(i18n, side_projects_headline)}</h2>
                // <p>TODO: projects</p>
                // </div>
                <div>"TBD"</div>
            </div>
        </main>
    }
}

