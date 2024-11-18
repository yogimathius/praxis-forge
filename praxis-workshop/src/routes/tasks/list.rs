use stylist::style;
use yew::{function_component, html, Html};

use crate::{routes::tasks::use_tasks::use_tasks, TaskForm, TaskList};

#[function_component]
pub fn TasksList() -> Html {
    let tasks_hook = use_tasks();

    let container_styles = style!(
        r#"
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
        "#
    )
    .unwrap();

    let title_styles = style!(
        r#"
        color: #1a202c;
        font-size: 2rem;
        margin-bottom: 2rem;
        "#
    )
    .unwrap();

    let error_styles = style!(
        r#"
        color: #dc2626;
        padding: 1rem;
        margin: 1rem 0;
        border-radius: 0.25rem;
        background-color: #fee2e2;
        "#
    )
    .unwrap();

    html! {
        <div class={container_styles}>
            <h1 class={title_styles}>{"Praxis Workshop"}</h1>
            <TaskForm on_add={tasks_hook.on_add} />
            if let Some(err) = (*tasks_hook.error).clone() {
                <p class={error_styles}>{"Error: "}{err}</p>
            }
            <TaskList tasks={(*tasks_hook.tasks).clone()} on_toggle={tasks_hook.on_toggle.clone()} />
        </div>
    }
}
