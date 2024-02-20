use leptos::*;

use crate::components::atoms::{TimelineElement, TimelineEntry};

#[component]
pub fn Timeline(content: Vec<TimelineEntry>) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            {content.into_iter().map(|c| view! { <TimelineElement data=c/> }).collect_view()}
        </div>
    }
}

