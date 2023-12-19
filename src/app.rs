use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::i18n::*;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
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
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

