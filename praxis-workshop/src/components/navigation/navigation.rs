use leptos::prelude::*;
use leptos_router::{components::*, hooks::use_navigate};

// Let's define our own Location enum since we need it
#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    Tasks,
    Goals,
    Progress,
}

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
        <nav class="flex justify-center gap-8 p-4 mb-8 bg-white/5 backdrop-blur border border-[#ff6b35] rounded-xl">
            <button
                class=move || {
                    let base = "rounded-md transition-all duration-300 cursor-pointer hover:bg-[#ff6b35]/10 hover:border hover:border-[#ff6b35] hover:shadow-[0_0_15px_rgba(255,107,53,0.15)] active:bg-[#ff6b35]/15 active:shadow-[0_0_15px_rgba(255,107,53,0.2)]";
                    let active = if location.get() == Location::Home {
                        "bg-[#ff6b35]/15 shadow-[0_0_15px_rgba(255,107,53,0.2)] border border-[#ff6b35]"
                    } else {
                        ""
                    };
                    format!("{} {}", base, active)
                }
                on:click=move |_| { handle_navigation.dispatch(Location::Home); }
            >
                "Home"
            </button>
            <button
                class=move || {
                    let base = "rounded-md transition-all duration-300 cursor-pointer hover:bg-[#ff6b35]/10 hover:border hover:border-[#ff6b35] hover:shadow-[0_0_15px_rgba(255,107,53,0.15)] active:bg-[#ff6b35]/15 active:shadow-[0_0_15px_rgba(255,107,53,0.2)]";
                    let active = if location.get() == Location::Tasks {
                        "bg-[#ff6b35]/15 shadow-[0_0_15px_rgba(255,107,53,0.2)] border border-[#ff6b35]"
                    } else {
                        ""
                    };
                    format!("{} {}", base, active)
                }
                on:click=move |_| { handle_navigation.dispatch(Location::Tasks); }
            >
                "Tasks"
            </button>
            <button
                class=move || {
                    let base = "rounded-md transition-all duration-300 cursor-pointer hover:bg-[#ff6b35]/10 hover:border hover:border-[#ff6b35] hover:shadow-[0_0_15px_rgba(255,107,53,0.15)] active:bg-[#ff6b35]/15 active:shadow-[0_0_15px_rgba(255,107,53,0.2)]";
                    let active = if location.get() == Location::Goals {
                        "bg-[#ff6b35]/15 shadow-[0_0_15px_rgba(255,107,53,0.2)] border border-[#ff6b35]"
                    } else {
                        ""
                    };
                    format!("{} {}", base, active)
                }
                on:click=move |_| { handle_navigation.dispatch(Location::Goals); }
            >
                "Goals"
            </button>
        </nav>
    }
}
