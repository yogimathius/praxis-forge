use leptos::*;
mod app;
mod components;
mod pages;
mod state;
mod utils;
use app::App;

pub fn main() {
    _ = console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
