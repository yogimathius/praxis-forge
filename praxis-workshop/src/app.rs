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
        background-color: #f0f4f8;
        "#
    )
    .unwrap();

    let nav_styles = style!(
        r#"
        margin-bottom: 2rem;
        "#
    )
    .unwrap();
    let link_styles = style!(
        r#"
        margin-right: 1rem;
        text-decoration: none;
        color: #007acc;
        &:hover { color: #005f99; }
        "#
    )
    .unwrap();

    let title_styles = style!(
        r#"
        color: #333333;
        font-size: 2rem;
        margin-bottom: 2rem;
        "#
    )
    .unwrap();

    html! {
        <BrowserRouter>
            <div class={container_styles}>
                <h1 class={title_styles}>{"Praxis Workshop"}</h1>
                <nav class={nav_styles}>
                    <Link<Route> to={Route::Home} classes={link_styles.clone()}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::Tasks} classes={link_styles.clone()}>{ "Tasks" }</Link<Route>>
                </nav>
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}
