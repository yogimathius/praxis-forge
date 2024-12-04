use stylist::{style, yew::styled_component};
use yew::{html, Callback, Html, Properties};

use crate::{services::Task, task::TaskItem};

#[derive(Properties, Clone, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
    pub on_toggle: Callback<Task>,
    pub on_delete: Callback<Task>,
    pub on_edit: Callback<Task>,
}

#[styled_component]
pub fn TaskList(props: &TaskListProps) -> Html {
    let styles = style!(
        r#"
        .task-container {
            display: flex;
            gap: 1rem;
        }
        .task-item {
            display: flex;
            align-items: center;
            padding: 1rem;
            background-color: #f3f4f6;
            border-radius: 0.25rem;
        }
    "#
    )
    .unwrap();

    html! {
        <div class={styles}>
            {for props.tasks.iter().map(|task| {
                let on_toggle = {
                    let on_toggle = props.on_toggle.clone();
                    let task = task.clone();
                    Callback::from(move |_| on_toggle.emit(task.clone()))
                };
                let on_delete = {
                    let on_delete = props.on_delete.clone();
                    let task = task.clone();
                    Callback::from(move |_| on_delete.emit(task.clone()))
                };
                let on_edit = {
                    let on_edit = props.on_edit.clone();
                    let task = task.clone();
                    Callback::from(move |_| on_edit.emit(task.clone()))
                };
                html! {
                    <TaskItem task={task.clone()} on_toggle={on_toggle} on_delete={on_delete} on_edit={on_edit} />
                }
            })}
        </div>
    }
}
