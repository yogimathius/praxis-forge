use stylist::{style, yew::styled_component};
use yew::{html, Callback, Html, Properties};

use crate::Task;

#[derive(Properties, Clone, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
    pub on_toggle: Callback<Task>,
    pub on_delete: Callback<Task>,
    pub on_edit: Callback<Task>,
}

#[styled_component]
pub fn TaskItem(props: &TaskItemProps) -> Html {
    let task_item_styles = style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.5rem;
        margin-bottom: 0.5rem;
        background-color: white;
        border-radius: 0.375rem;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
    "#
    )
    .unwrap();

    let checkbox_styles = style!(
        r#"
        height: 1rem;
        width: 1rem;
        color: #2563eb;
        border-radius: 0.25rem;
        cursor: pointer;
    "#
    )
    .unwrap();

    let text_styles = style!(
        r#"
        margin-left: 0.75rem;
        color: #1f2937;
        "#
    )
    .unwrap();

    let completed_text_styles = style!(
        r#"
        margin-left: 0.75rem;
        text-decoration: line-through;
        color: #6b7280;
        "#
    )
    .unwrap();

    let delete_button_styles = style!(
        r#"
        margin-left: 0.75rem;
        "#
    )
    .unwrap();

    let edit_button_styles = style!(
        r#"
        margin-left: 0.75rem;
        "#
    )
    .unwrap();

    let handle_delete = {
        let on_delete = props.on_delete.clone();
        let task = props.task.clone();
        Callback::from(move |_| on_delete.emit(task.clone()))
    };

    let handle_edit = {
        let on_edit = props.on_edit.clone();
        let task = props.task.clone();
        Callback::from(move |_| on_edit.emit(task.clone()))
    };

    html! {
        <div class={task_item_styles}>
            <div>
                <input
                    type="checkbox"
                    checked={props.task.completed}
                    class={checkbox_styles}
                    onchange={let task = props.task.clone();
                            let on_toggle = props.on_toggle.clone();
                            Callback::from(move |_| on_toggle.emit(task.clone()))}
                />
                <span class={if props.task.completed { completed_text_styles } else { text_styles }}>
                    {&props.task.title}
                </span>
            </div>
            <button class={delete_button_styles} onclick={handle_delete}>{"Delete"}</button>
            <button class={edit_button_styles} onclick={handle_edit}>{"Edit"}</button>
        </div>
    }
}
