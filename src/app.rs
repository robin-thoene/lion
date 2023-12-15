use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::timeline::{Timeline, TimelineElement};
use crate::components::top_nav::TopNav;
use crate::i18n::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let i18n = use_i18n();

    let experience_timeline_elements = vec![
        TimelineElement {
            title: view! {<>Senior {t!(i18n, work_title_software_developer)}</>}.into_view(),
            subtitle: view! {"Enpal"}.into_view(),
            start_year: 2023,
            end_year: None,
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, work_title_software_developer)}</>}.into_view(),
            subtitle: view! {"DEVDEER GmbH"}.into_view(),
            start_year: 2020,
            end_year: Some(2023),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, apprenticeship_title_software_developer)}</>}.into_view(),
            subtitle: view! {"AV-TEST GmbH"}.into_view(),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, work_title_ecological_volunteer)}</>}.into_view(),
            subtitle: view! {
                "Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt"
            }
            .into_view(),
            start_year: 2016,
            end_year: Some(2017),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, dual_study_auditor)}</>}.into_view(),
            subtitle: view! {"PricewaterhouseCoopers GmbH"}.into_view(),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
    ];

    let education_timeline_elements = vec![
        TimelineElement {
            title: view! {<>{t!(i18n, education_university_business_informatics)}</>}.into_view(),
            subtitle: view! {"AKAD University"}.into_view(),
            start_year: 2020,
            end_year: None,
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, apprenticeship_title_software_developer)}</>}.into_view(),
            subtitle: view! {"Berufsbildende Schulen „Otto von Guericke“"}.into_view(),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, dual_study_auditor)}</>}.into_view(),
            subtitle: view! {"Duale Hochschule Baden-Württemberg"}.into_view(),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
        TimelineElement {
            title: view! {<>{t!(i18n, education_a_levels)}</>}.into_view(),
            subtitle: view! {"Berufsbildende Schulen 1 Goslar -Am Stadtgarten-"}.into_view(),
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
        <main class="w-screen max-w-screen-2xl p-6">
            <TopNav/>
            <div class="flex flex-col gap-20">
                <div class="flex justify-center">
                    <img
                        alt=t!(i18n, profile_image_alt_text)
                        src="/profile.png"
                        class="h-64 w-64 object-cover rounded-full"
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
            </div>
        </main>
    }
}

