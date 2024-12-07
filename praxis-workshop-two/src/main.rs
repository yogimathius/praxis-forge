use app::App;
use leptos::{mount_to_body, view};

mod api;
mod app;
mod components;
mod pages;
mod state;

fn main() {
    mount_to_body(|| view! { <App/> })
}
