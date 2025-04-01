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
use styles::{CUSTOM_UTILITIES, TAILWIND_BUNDLE};

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use std::sync::Arc;
use web_sys::window;

// Import our new components
use components::{provide_theme, CommandPalette, ThemeToggle};

#[component]
pub fn App() -> impl IntoView {
    let service = GraphQLService::new();
    provide_context(ServiceContext(Arc::new(service)));

    // Provide theme context
    provide_theme();

    // Inject Tailwind styles
    let window = window().unwrap();
    let document = window.document().unwrap();

    // Inject custom utility style
    let custom_style_el = document.create_element("style").unwrap();
    let custom_styles = CUSTOM_UTILITIES;
    custom_style_el.set_text_content(Some(custom_styles));
    document
        .head()
        .unwrap()
        .append_child(&custom_style_el)
        .unwrap();

    // Inject Tailwind styles
    let tailwind_style_el = document.create_element("style").unwrap();
    let tailwind_styles = TAILWIND_BUNDLE;
    tailwind_style_el.set_text_content(Some(&tailwind_styles));
    document
        .head()
        .unwrap()
        .append_child(&tailwind_style_el)
        .unwrap();

    view! {
        <Router>
            <div style="display: none;" id="tailwind-debug">
                {tailwind_styles}
            </div>

            // Add our Command Palette at the top level
            <CommandPalette />

            <div class="forge-animated-background"></div>
            <main class="min-h-screen relative">
                <Navigation />

                // Add theme toggle in the top right corner
                <div class="fixed top-4 right-4 z-10">
                    <ThemeToggle />
                </div>

                <Routes fallback=move || view! { <p>"Not found."</p> }>
                    <ParentRoute path=path!("") view=|| view! { <Outlet /> }>
                        <Route path=path!("") view=Home />
                        <Route path=path!("tasks") view=TasksListPage />
                        <Route path=path!("goals") view=GoalsListPage />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
