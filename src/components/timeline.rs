use leptos::*;

#[component]
pub fn Timeline(content: Vec<View>) -> impl IntoView {
    view! { <ol>{content.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ol> }
}

