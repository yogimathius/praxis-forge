pub mod components;
pub mod graphql;
pub mod pages;
pub mod services;
pub mod state;
pub mod styles;

pub use components::Navigation;
pub use pages::goals::GoalsListPage;
pub use pages::home::Home;
pub use pages::tasks::tasks::TasksListPage;
use services::graphql_service::GraphQLService;
use services::service_context::ServiceContext;
use styles::tailwind::get_tailwind_bundle;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use std::sync::Arc;
use wasm_bindgen_test::console_log;
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let service = GraphQLService::new();
    provide_context(ServiceContext(Arc::new(service)));

    // Inject Tailwind styles
    let window = window().unwrap();
    let document = window.document().unwrap();
    let style_el = document.create_element("style").unwrap();
    let styles = get_tailwind_bundle();
    console_log!("Generated styles: {}", styles);
    style_el.set_text_content(Some(&styles));
    document.head().unwrap().append_child(&style_el).unwrap();

    view! {
        <Router>
            <main>
                <Navigation/>
                <Routes fallback=move || view! { <p>"Not found."</p> }>
                    <ParentRoute
                        path=path!("")
                        view=|| view! { <Outlet/> }
                    >
                        <Route
                            path=path!("")
                            view=Home
                        />
                        // <Route
                        //     path=path!("progress")
                        //     view=ProgressBarPage
                        // />
                        <Route
                            path=path!("tasks")
                            view=TasksListPage
                        />
                        <Route
                            path=path!("goals")
                            view=GoalsListPage
                        />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
