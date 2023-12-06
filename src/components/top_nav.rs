use leptos::*;
use leptos_icons::*;

use crate::app::i18n::*;

#[component]
pub fn TopNav() -> impl IntoView {
    let i18n = use_i18n();

    let set_locale_german = move |_| {
        i18n.set_locale(Locale::de);
    };

    let set_locale_english = move |_| {
        i18n.set_locale(Locale::en);
    };

    view! {
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
    }
}

