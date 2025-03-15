use leptos::prelude::*;

#[component]
pub fn Header(title: String, description: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 items-center">
            <h2 class="text-4xl font-bold text-spark animate-ember-pulse">{title}</h2>
            <p class="text-xl text-ash animate-slide-up">{description}</p>
        </div>
    }
}
