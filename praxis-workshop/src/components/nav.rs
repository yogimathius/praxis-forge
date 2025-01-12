use crate::state::location::{Location, LocationState};
use leptos::*;
use leptos_router::A;

#[component]
pub fn Navigation(location_state: LocationState) -> impl IntoView {
    let (location, set_location) = (location_state.0, location_state.1);

    view! {
        <nav class="nav">
            <div class="nav-item" class:active=move || location.get() == Location::Home>
                <A
                    href="/"
                    class="link"
                    on:click=move |_| set_location.set(Location::Home)
                >
                    "Home"
                </A>
            </div>
            <div class="nav-item" class:active=move || location.get() == Location::Tasks>
                <A
                    href="/tasks"
                    class="link"
                    on:click=move |_| set_location.set(Location::Tasks)
                >
                    "Tasks"
                </A>
            </div>
        </nav>
    }
}
