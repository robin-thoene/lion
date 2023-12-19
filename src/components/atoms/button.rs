use leptos::*;

#[component]
pub fn Button<F, Fp>(text: String, click: F, is_primary: Fp) -> impl IntoView
where
    F: Fn(ev::MouseEvent) -> () + 'static,
    Fp: Fn() -> bool + 'static,
{
    view! {
        <button
            class=move || { if is_primary() { "text-primary-alt dark:text-primary" } else { "" } }

            on:click=click
        >
            "["
            {text}
            "]"
        </button>
    }
}

