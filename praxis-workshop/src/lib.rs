use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;
use std::sync::Arc;

pub mod components;
pub mod graphql;
pub mod pages;
pub mod services;
pub mod state;
pub mod tests;
pub mod utils;

use crate::components::nav::Navigation;
use crate::pages::goals::goals::GoalsListPage;
use crate::pages::home::HomePage;
use crate::pages::tasks::dashboard::TasksListPage;
use crate::services::graphql_service::GraphQLService;
use crate::services::service_context::ServiceContext;
use crate::state::location::use_location;

#[component]
pub fn App() -> impl IntoView {
    let location_state = use_location();
    let service = GraphQLService::new();

    provide_context(ServiceContext(Arc::new(service)));

    view! {
        <Router>
            <main>
                <Navigation location_state=location_state/>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/tasks") view=TasksListPage/>
                    <Route path=path!("/goals") view=GoalsListPage/>
                </Routes>
            </main>
        </Router>
    }
}
