use crate::state::location::{Location, LocationState};
use leptos::*;
use leptos_router::A;

#[component]
pub fn Navigation(location_state: LocationState) -> impl IntoView {
    let (location, set_location) = (location_state.0, location_state.1);

    view! {
        <nav class="nav">
            <A
                href="/"
                class="link"
                on:click=move |_| set_location.set(Location::Home)
            >
                "Home"
            </A>
            <A
                href="/tasks"
                class="link"
                on:click=move |_| set_location.set(Location::Tasks)
            >
                "Tasks"
            </A>
        </nav>
    }
}
