use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/settings")]
    Settings,
    #[at("/tasks")]
    Tasks,
    #[not_found]
    #[at("/404")]
    NotFound,
}
