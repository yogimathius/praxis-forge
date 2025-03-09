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
        <Flex
            justify=FlexJustify::Center
            class="gap-8 p-4 mb-8 bg-glass border border-orange rounded-xl"
        >
            <Button
                appearance=Signal::derive(move || {
                    if location.get() == Location::Home {
                        ButtonAppearance::Primary
                    } else {
                        ButtonAppearance::Subtle
                    }
                })
                class="text-orange hover-glow-orange"
                on_click=move |_| { let _ = handle_navigation.dispatch(Location::Home); }
            >
                "Home"
            </Button>
            <Button
                appearance=Signal::derive(move || {
                    if location.get() == Location::Tasks {
                        ButtonAppearance::Primary
                    } else {
                        ButtonAppearance::Subtle
                    }
                })
                class="text-orange hover-glow-orange"
                on_click=move |_| { let _ = handle_navigation.dispatch(Location::Tasks); }
            >
                "Tasks"
            </Button>
            <Button
                appearance=Signal::derive(move || {
                    if location.get() == Location::Goals {
                        ButtonAppearance::Primary
                    } else {
                        ButtonAppearance::Subtle
                    }
                })
                class="text-orange hover-glow-orange"
                on_click=move |_| { let _ = handle_navigation.dispatch(Location::Goals); }
            >
                "Goals"
            </Button>
        </Flex>
    }
}
