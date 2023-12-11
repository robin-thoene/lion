use leptos::*;

pub struct TimelineElement {
    pub title: String,
    pub subtitle: String,
    pub time: String,
    pub content: View,
}

#[component]
pub fn Element(data: TimelineElement) -> impl IntoView {
    view! {
        <div class="relative pl-8 sm:pl-32 py-6 group">
            <div class="mb-1 sm:mb-0">{data.title}</div>
            <div class="flex flex-col sm:flex-row items-start mb-1 group-last:before:hidden before:absolute before:left-2 sm:before:left-0 before:h-full before:px-px before:bg-current sm:before:ml-[6.5rem] before:self-start before:-translate-x-1/2 before:translate-y-3 after:absolute after:left-2 sm:after:left-0 after:w-4 after:h-4 after:bg-current after:box-content after:rounded-full sm:after:ml-[6.5rem] after:-translate-x-1/2 after:translate-y-1.5">
                <time class="sm:absolute left-0 translate-y-0.5 inline-flex items-center justify-center w-20 h-6 mb-3 sm:mb-0 rounded-full bg-primary">
                    {data.time}
                </time>
                <div>{data.subtitle}</div>
            </div>
            <div>{data.content}</div>
        </div>
    }
}

#[component]
pub fn Timeline(content: Vec<TimelineElement>) -> impl IntoView {
    view! { <div>{content.into_iter().map(|c| view! { <Element data=c/> }).collect_view()}</div> }
}

