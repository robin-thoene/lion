use leptos::*;

use crate::components::organisms::{
    education_timeline::EducationTimeline, experience_timeline::ExperienceTimeline,
    side_projects::SideProjects, top_nav::TopNav,
};
use crate::i18n::*;

#[component]
pub fn Home() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <main class="w-screen max-w-screen-2xl p-6">
            <TopNav/>
            <div class="flex flex-col gap-20">
                <div class="flex justify-center">
                    <img
                        alt=t!(i18n, profile_image_alt_text)
                        src="/profile.webp"
                        class="h-64 w-64 object-cover rounded-full overflow-hidden"
                    />
                </div>
                <h1 class="text-center">Robin Thöne</h1>
                <div class="flex flex-col md:flex-row w-full gap-6">
                    <div class="flex flex-col flex-1">
                        <h2 class="text-center mb-8">{t!(i18n, education_headline)}</h2>
                        <EducationTimeline/>
                    </div>
                    <div class="border"></div>
                    <div class="flex flex-col flex-1">
                        <h2 class="text-center mb-8">{t!(i18n, work_experience_headline)}</h2>
                        <ExperienceTimeline/>
                    </div>
                </div>
                <div>
                    <h2 class="text-center mb-8">{t!(i18n, side_projects_headline)}</h2>
                    <SideProjects/>
                </div>
            </div>
        </main>
    }
}

