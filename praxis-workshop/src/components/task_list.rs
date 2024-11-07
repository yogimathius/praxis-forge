use yew::{function_component, html, Callback, Html, Properties};

use crate::api::Task;

#[derive(Properties, Clone, PartialEq)]
pub struct TaskListProps {
    pub tasks: Vec<Task>,
    pub on_toggle: Callback<Task>,
}

#[function_component(TaskList)]
pub fn task_list(props: &TaskListProps) -> Html {
    println!("Rendering TaskList with {} tasks", props.tasks.len());
    html! {
        <ul>
            {for props.tasks.iter().map(|task| {
                let on_toggle = {
                    let on_toggle = props.on_toggle.clone();
                    let task = task.clone();
                    Callback::from(move |_| on_toggle.emit(task.clone()))
                };
                html! {
                    <li key={task.id}>
                        <input
                            type="checkbox"
                            checked={task.completed}
                            onchange={on_toggle}
                        />
                        {&task.title}
                    </li>
                }
            })}
        </ul>
    }
}
