use stylist::style;
use tauri_app_ui::{routes::switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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
