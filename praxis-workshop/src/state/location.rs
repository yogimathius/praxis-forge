use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    Home,
    Tasks,
    Goals,
}

#[derive(Copy, Clone)]
pub struct LocationState(pub ReadSignal<Location>, pub WriteSignal<Location>);

pub fn use_location() -> LocationState {
    let (location, set_location) = signal(Location::Home);
    LocationState(location, set_location)
}
