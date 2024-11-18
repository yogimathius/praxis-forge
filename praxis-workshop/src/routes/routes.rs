use super::{tasks::list::TasksList, Home, Settings};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tasks")]
    Tasks,
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Tasks => html! { <TasksList /> },
        Route::Settings => html! { <Settings /> },
        Route::NotFound => html! { <h1>{"404"}</h1> },
    }
}
