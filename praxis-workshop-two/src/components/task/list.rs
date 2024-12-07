use leptos::*;

use crate::api::tasks::Task;

#[component]
pub fn TasksList(tasks: Vec<Task>) -> impl IntoView {
    println!("TasksList: {:?}", tasks);
    view! {
        <div>
            <h2>"Tasks List"</h2>
            <ul>
                {tasks
                    .iter()
                    .map(|task| view! { <li>{task.title.clone()}</li> })
                    .collect_view()}
            </ul>
        </div>
    }
}
