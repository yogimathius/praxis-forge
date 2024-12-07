use leptos::*;
mod api;
mod app;
mod components;
mod pages;
mod utils;
use app::App;

pub fn main() {
    _ = console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
