use leptos::prelude::*;
use leptos_router::{components::*, hooks::use_navigate};
use thaw::*;

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
        <div class="bg-glass px-8 md:px-0">
            <Flex
                justify=FlexJustify::Start
                class="gap-8 p-4 mb-8 border border-orange rounded-xl max-w-7xl mx-auto"
            >
                <Button
                    appearance=Signal::derive(move || ButtonAppearance::Subtle)
                    class=Signal::derive(move || {
                        if location.get() == Location::Home {
                            "text-orange font-bold border-b-2 border-orange shadow-orange-md px-6 py-3 rounded-xl hover:text-white"
                                .to_string()
                        } else {
                            "text-ash hover:text-orange hover:shadow-orange-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                .to_string()
                        }
                    })
                    on_click=move |_| {
                        let _ = handle_navigation.dispatch(Location::Home);
                    }
                >
                    "Home"
                </Button>
                <Button
                    appearance=Signal::derive(move || ButtonAppearance::Subtle)
                    class=Signal::derive(move || {
                        if location.get() == Location::Tasks {
                            "text-orange font-bold border-b-2 border-orange shadow-orange-md px-6 py-3 rounded-xl hover:text-white"
                                .to_string()
                        } else {
                            "text-ash hover:text-orange hover:shadow-orange-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                .to_string()
                        }
                    })
                    on_click=move |_| {
                        let _ = handle_navigation.dispatch(Location::Tasks);
                    }
                >
                    "Tasks"
                </Button>
                <Button
                    appearance=Signal::derive(move || ButtonAppearance::Subtle)
                    class=Signal::derive(move || {
                        if location.get() == Location::Goals {
                            "text-spark font-bold border-b-2 border-spark shadow-spark-md px-6 py-3 rounded-xl hover:text-white"
                                .to_string()
                        } else {
                            "text-ash hover:text-spark hover:shadow-spark-sm transition-all duration-300 px-6 py-3 rounded-xl"
                                .to_string()
                        }
                    })
                    on_click=move |_| {
                        let _ = handle_navigation.dispatch(Location::Goals);
                    }
                >
                    "Goals"
                </Button>
            </Flex>
        </div>
    }
}
