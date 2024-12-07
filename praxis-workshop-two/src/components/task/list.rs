use leptos::*;

use crate::api::tasks::Task;
use crate::components::task::item::TaskItem;
#[component]
pub fn TasksList(tasks: Vec<Task>) -> impl IntoView {
    println!("TasksList: {:?}", tasks);
    view! {
        <div>
            <h2>"Tasks List"</h2>
            <ul>
                {tasks
                    .iter()
                    .map(|task| view! { <TaskItem task=task.clone() /> })
                    .collect_view()}
            </ul>
        </div>
    }
}
