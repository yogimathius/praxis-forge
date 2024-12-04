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
        "#
    )
    .unwrap();

    let error_styles = style!(
        r#"
        color: #dc2626;
        padding: 0 1rem;
        margin: 1rem 0;
        border-radius: 0.25rem;
        background-color: #fee2e2;
        "#
    )
    .unwrap();

    html! {
        <div class={container_styles}>
            <TaskForm on_add={tasks_hook.on_add} />
            if let Some(err) = (*tasks_hook.error).clone() {
                <p class={error_styles}>{"Error: "}{err}</p>
            }
            <TaskList
                tasks={(*tasks_hook.tasks).clone()}
                on_toggle={tasks_hook.on_toggle.clone()}
                on_delete={tasks_hook.on_delete.clone()}
                on_edit={tasks_hook.on_edit.clone()}
            />
        </div>
    }
}
