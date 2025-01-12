use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    Tasks,
}

#[derive(Copy, Clone)]
pub struct LocationState(pub ReadSignal<Location>, pub WriteSignal<Location>);

pub fn use_location() -> LocationState {
    let (location, set_location) = create_signal(Location::Home);
    LocationState(location, set_location)
}
