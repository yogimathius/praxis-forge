use leptos::*;
use leptos_router::*;
use std::sync::Arc;

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
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/tasks" view=TasksListPage/>
                    <Route path="/goals" view=GoalsListPage/>
                </Routes>
            </main>
        </Router>
    }
}
