use leptos::prelude::*;
use leptos_router::{components::*, hooks::use_navigate};
use wasm_bindgen::prelude::*;

// Let's define our own Location enum since we need it
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    Tasks,
    Goals,
    Progress,
}

#[wasm_bindgen(module = "/src/components/navigation/navigation.module.css")]
extern "C" {}

#[component]
pub fn Navigation() -> impl IntoView {
    // Create signals for location state
    let (location, set_location) = signal(Location::Home);
    // Get the navigate function from leptos_router
    let navigate = use_navigate();

    // Create an action for the navigation handler
    let handle_navigation = Action::new(move |new_location: &Location| {
        let new_location = new_location.clone();
        let navigate = navigate.clone();

        async move {
            set_location.set(new_location.clone());

            let path = match new_location {
                Location::Home => "/",
                Location::Tasks => "/tasks",
                Location::Goals => "/goals",
                Location::Progress => "/progress",
            };

            navigate(path, Default::default())
        }
    });

    view! {
        <nav class="nav">
            <button
                class="nav-item"
                class:active=move || location.get() == Location::Home
                on:click=move |_| { handle_navigation.dispatch(Location::Home); }
            >
                "Home"
            </button>
            <button
                class="nav-item"
                class:active=move || location.get() == Location::Tasks
                on:click=move |_| { handle_navigation.dispatch(Location::Tasks); }
            >
                "Tasks"
            </button>
            <button
                class="nav-item"
                class:active=move || location.get() == Location::Goals
                on:click=move |_| { handle_navigation.dispatch(Location::Goals); }
            >
                "Goals"
            </button>
        </nav>
    }
}
