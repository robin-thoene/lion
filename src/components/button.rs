use leptos::*;

#[component]
pub fn Button<F, Fp>(text: String, click: F, is_primary: Fp) -> impl IntoView
where
    F: Fn(ev::MouseEvent) -> () + 'static,
    Fp: Fn() -> bool + 'static,
{
    view! {
        <button class=("text-primary", is_primary) on:click=click>
            "["
            {text}
            "]"
        </button>
    }
}

