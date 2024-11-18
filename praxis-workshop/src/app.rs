use stylist::style;
use tauri_app_ui::{routes::switch, Route};
use web_sys::console;
use yew::prelude::*;
use yew::use_effect_with;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let location = use_location();

    use_effect_with(location.clone(), move |location| {
        if let Some(location) = location {
            console::log_1(&format!("Route changed: {:?}", location.path()).into());
        } else {
            console::log_1(&"Location is not available".into());
        }
        move || ()
    });

    let container_styles = style!(
        r#"
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
        "#
    )
    .unwrap();

    // let nav_styles = style!(
    //     r#"
    //     margin-bottom: 2rem;
    //     a {
    //         margin-right: 1rem;
    //         text-decoration: none;
    //         color: #4a5568;
    //         &:hover { color: #2d3748; }
    //     }
    //     "#
    // )
    // .unwrap();

    html! {
        <BrowserRouter>
            <div class={container_styles}>
                <nav>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::Tasks}>{ "Tasks" }</Link<Route>>
                </nav>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}
