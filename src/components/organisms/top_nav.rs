use leptos::*;
use leptos_icons::*;

use crate::components::atoms::button::Button;
use crate::i18n::*;

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
                <a
                    aria-label="GitHub"
                    href="https://github.com/robin-thoene"
                    target="_blank"
                    rel="noreferrer"
                >
                    <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaGithubBrands)/>
                </a>
                <a
                    aria-label="LinkedIn"
                    class="ml-4"
                    href="https://linkedin.com/in/robin-thöne-681870205"
                    target="_blank"
                    rel="noreferrer"
                >
                    <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaLinkedinBrands)/>
                </a>
                <a
                    aria-label="XING"
                    class="ml-4"
                    href="https://www.xing.com/profile/Robin_Thoene"
                    target="_blank"
                    rel="noreferrer"
                >
                    <Icon class="h-5 w-5" icon=Icon::from(FaIcon::FaXingBrands)/>
                </a>
            </div>
            <div class="flex">
                <div class="mr-4">
                    <Button
                        is_primary=move || i18n.get_locale() == Locale::en
                        text="en".to_string()
                        click=set_locale_english
                    />
                </div>
                <Button
                    is_primary=move || i18n.get_locale() == Locale::de
                    text="de".to_string()
                    click=set_locale_german
                />
            </div>
        </div>
    }
}

