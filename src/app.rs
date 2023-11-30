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
        <Html class="dark"/>
        <main>
            <div class="max-w-screen-2xl">
                <div class="w-full flex justify-between fixed bg-white dark:bg-black top-0 left-0 p-6">
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
            </div>
        </main>
    }
}

