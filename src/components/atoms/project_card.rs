use leptos::*;

#[component]
pub fn ProjectCard() -> impl IntoView {
    view! {
        <div class="flex flex-col border border-slate-400 dark:border-slate-100 rounded-lg p-3 gap-3 max-w-max">
            <div class="flex justify-between">
                <div>PROJECT NAME</div>
                <div class="flex gap-3">
                    <div>GH LINK</div>
                    <div>WEBSITE URL</div>
                </div>
            </div>
            <div class="flex gap-3">
                <div>Tech 1</div>
                <div>Tech 2</div>
                <div>Tech 3</div>
            </div>
            <div>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
            </div>
        </div>
    }
}

