use leptos::*;
use praxis_workshop_ui::app::App;

pub fn main() {
    _ = console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
