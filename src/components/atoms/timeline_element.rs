use leptos::*;

pub struct TimelineEntry {
    pub title: View,
    pub subtitle: View,
    pub start_year: u32,
    pub end_year: Option<u32>,
    pub content: Option<View>,
}

#[component]
pub fn TimelineElement(data: TimelineEntry) -> impl IntoView {
    let time_display = if let Some(end_year) = data.end_year {
        format!("{} - {}", data.start_year, end_year)
    } else {
        data.start_year.to_string()
    };

    view! {
        <div class="flex flex-col border border-slate-400 dark:border-slate-100 rounded-lg p-3 gap-3">
            <div class="bg-primary text-black w-max rounded-2xl px-3 py-1">{time_display}</div>
            <div class="font-bold">{data.title}</div>
            <div>{data.subtitle}</div>
            {data.content}
        </div>
    }
}

