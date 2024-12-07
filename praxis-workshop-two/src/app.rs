use crate::pages::home::HomePage;
use crate::pages::tasks::list::TasksListPage;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
            <main class="container">
                <h1 class="title">"Praxis Workshop"</h1>
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
        </Router>
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
