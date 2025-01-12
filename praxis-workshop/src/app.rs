use crate::pages::home::HomePage;
use crate::pages::tasks::dashboard::TasksListPage;
use leptos::*;
use leptos_dom::logging::console_log;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[wasm_bindgen(module = "/src/app.css")]
extern "C" {}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <MainContent />
        </Router>
    }
}

#[component]
fn MainContent() -> impl IntoView {
    let (location, set_location) = create_signal("/");
    let message = format!("location: {:?}", location.get());

    console::log_1(&JsValue::from_str(&message));

    let location = use_location();

    let current_location = move || location.pathname.get();

    console_log(&format!("Current location: {}", current_location()));

    let title = match current_location().as_str() {
        "/" => "Home - Praxis Workshop",
        "/tasks" => "Forge Operations - Praxis Workshop",
        _ => "404 Not Found - Praxis Workshop",
    };

    view! {
        <main class="container">
            <h1 class="title">{title}</h1>
            <nav class="nav">
                <A class="link" href="/">"Home"</A>
                <A class="link" href="/tasks">"Tasks"</A>
            </nav>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/tasks" view=TasksListPage/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </main>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"404 Not Found"</h1>
        </div>
    }
}
