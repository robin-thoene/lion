leptos_i18n::load_locales!();

use i18n::*;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;

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

    let set_locale_german = move |_| {
        i18n.set_locale(Locale::de);
    };

    let set_locale_english = move |_| {
        i18n.set_locale(Locale::en);
    };

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
            <div class="max-w-screen-2xl fixed inset-x-0 mx-auto w-full flex justify-between bg-white dark:bg-black top-0 left-0 p-6">
                <div class="flex flex-row">
                    <a href="https://github.com/robin-thoene" target="_blank" rel="noreferrer">
                        <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaGithubBrands)/>
                    </a>
                    <a
                        class="ml-4"
                        href="https://linkedin.com/in/robin-thöne-681870205"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaLinkedinBrands)/>
                    </a>
                    <a
                        class="ml-4"
                        href="https://www.xing.com/profile/Robin_Thoene"
                        target="_blank"
                        rel="noreferrer"
                    >
                        <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaXingBrands)/>
                    </a>
                </div>
                <div>
                    <button
                        class="mr-4"
                        class=("text-primary", move || i18n.get_locale() == Locale::en)
                        on:click=set_locale_english
                    >
                        [en]
                    </button>
                    <button
                        class=("text-primary", move || i18n.get_locale() == Locale::de)
                        on:click=set_locale_german
                    >
                        [de]
                    </button>
                </div>
            </div>
            <h1>Robin Thöne</h1>
            <p>TODO: Profile</p>
            <h2>{t!(i18n, education_headline)}</h2>
            <p>TODO: Education timeline</p>
            <h2>{t!(i18n, work_experience_headline)}</h2>
            <p>TODO: work experience timeline</p>
            <h2>{t!(i18n, side_projects_headline)}</h2>
            <p>TODO: projects</p>
            <p>
                <div>regular</div>
                <div>
                    <i>italic</i>
                </div>
                <div>
                    <b>bold</b>
                </div>
                <div>
                    <i>
                        <b>italic and bold</b>
                    </i>
                </div>
                <div class="font-extrabold">extra bold</div>
                <div class="font-extrabold italic">italic and extra bold</div>
                <div class="font-extralight">extra light</div>
                <div class="font-extralight italic">italic and extra light</div>
                <div class="font-light">light</div>
                <div class="font-light italic">italic and light</div>
                <div class="font-medium">medium</div>
                <div class="font-medium italic">italic and medium</div>
                <div class="font-semibold">semibold</div>
                <div class="font-semibold italic">italic and semibold</div>
                <div class="font-thin">thin</div>
                <div class="font-thin italic">italic and thin</div>
            </p>
        </main>
    }
}

