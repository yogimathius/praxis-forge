use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

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
        <div class="backdrop-blur-lg border-b sticky top-0 z-10
        dark:bg-slate-800/70 dark:border-[#00b2ca]/20
        light:bg-white/70 light:border-[#00b2ca]/10">
            <div class="flex justify-between items-center p-4 max-w-7xl mx-auto">
                <div class="flex gap-8">
                    <button
                        class=move || {
                            if location.get() == Location::Home {
                                "text-titanium font-bold border-b-2 border-orange shadow-titanium-md px-6 py-3 rounded-xl hover:text-white"
                                    .to_string()
                            } else {
                                "text-ash hover:text-titanium hover:shadow-titanium-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                    .to_string()
                            }
                        }
                        on:click=move |_| {
                            let _ = handle_navigation.dispatch(Location::Home);
                        }
                    >
                        "Home"
                    </button>
                    <button
                        class=move || {
                            if location.get() == Location::Tasks {
                                "text-titanium font-bold border-b-2 border-orange shadow-titanium-md px-6 py-3 rounded-xl hover:text-white"
                                    .to_string()
                            } else {
                                "text-ash hover:text-titanium hover:shadow-titanium-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                    .to_string()
                            }
                        }
                        on:click=move |_| {
                            let _ = handle_navigation.dispatch(Location::Tasks);
                        }
                    >
                        "Tasks"
                    </button>
                    <button
                        class=move || {
                            if location.get() == Location::Goals {
                                "text-spark font-bold border-b-2 border-spark shadow-spark-md px-6 py-3 rounded-xl hover:text-white"
                                    .to_string()
                            } else {
                                "text-ash hover:text-spark hover:shadow-spark-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                    .to_string()
                            }
                        }
                        on:click=move |_| {
                            let _ = handle_navigation.dispatch(Location::Goals);
                        }
                    >
                        "Goals"
                    </button>
                </div>

                <div class="text-titanium font-bold text-xl">Praxis Forge</div>
            </div>
        </div>
    }
}
