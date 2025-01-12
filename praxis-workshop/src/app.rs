use leptos::*;
use leptos_router::*;

use crate::components::nav::Navigation;
use crate::pages::home::HomePage;
use crate::pages::tasks::dashboard::TasksListPage;
use crate::state::location::use_location;

#[component]
pub fn App() -> impl IntoView {
    let location_state = use_location();

    view! {
        <Router>
            <main>
                <Navigation location_state=location_state/>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/tasks" view=TasksListPage/>
                </Routes>
            </main>
        </Router>
    }
}
