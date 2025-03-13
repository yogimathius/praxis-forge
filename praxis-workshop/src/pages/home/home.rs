use crate::components::common::Header;
use crate::components::Principles;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="max-w-7xl mx-auto p-4 md:p-8">
            <div class="flex flex-col gap-8">
                <Header
                    title="Welcome to Praxis Forge".to_string()
                    description="Transform intentions into actions, actions into habits, habits into mastery."
                        .to_string()
                />

                <Principles />

                <div class="flex justify-center">
                    <a
                        href="/tasks"
                        class="btn btn-orange hover-lift shadow-orange-sm focus:outline-none"
                    >
                        "Start Your Journey"
                    </a>
                </div>
            </div>
        </div>
    }
}
