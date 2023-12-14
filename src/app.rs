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
            title: String::from("Senior Software Developer"),
            subtitle: String::from("Enpal"),
            start_year: 2023,
            end_year: None,
            content: None,
        },
        TimelineElement {
            title: String::from("Software Developer"),
            subtitle: String::from("DEVDEER GmbH"),
            start_year: 2020,
            end_year: Some(2023),
            content: None,
        },
        TimelineElement {
            title: String::from("Training as IT specialist - application development"),
            subtitle: String::from("AV-TEST GmbH"),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineElement {
            title: String::from("Voluntary Ecological Year"),
            subtitle: String::from(
                "Stiftung Umwelt, Natur- und Klimaschutz des Landes Sachsen-Anhalt",
            ),
            start_year: 2016,
            end_year: Some(2017),
            content: None,
        },
        TimelineElement {
            title: String::from("Dual study - Auditor"),
            subtitle: String::from("PricewaterhouseCoopers GmbH"),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
    ];

    let education_timeline_elements = vec![
        TimelineElement {
            title: String::from("Study - B. Sc. Business Informatics"),
            subtitle: String::from("AKAD University"),
            start_year: 2020,
            end_year: None,
            content: None,
        },
        TimelineElement {
            title: String::from("Training as an IT specialist - application development"),
            subtitle: String::from("Berufsbildende Schulen „Otto von Guericke"),
            start_year: 2017,
            end_year: Some(2020),
            content: None,
        },
        TimelineElement {
            title: String::from("Dual study - Auditor"),
            subtitle: String::from("Duale Hochschule Baden-Württemberg"),
            start_year: 2015,
            end_year: Some(2016),
            content: None,
        },
        TimelineElement {
            title: String::from("General university entrance qualification"),
            subtitle: String::from("Berufsbildende Schulen 1 Goslar -Am Stadtgarten-"),
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
            <h1 class="text-center">Robin Thöne</h1>
            <div class="flex justify-center">
                <p>TODO: Profile</p>
            </div>
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
            <h2 class="text-center">{t!(i18n, side_projects_headline)}</h2>
            <p>TODO: projects</p>
        </main>
    }
}

