use leptos::*;

use crate::api::tasks::Task;

#[component]
pub fn TaskItem(task: Task) -> impl IntoView {
    view! {
        <div class="item.module.css:taskItem">
            <h3 class="item.module.css:title">{task.title}</h3>

            {move || task.description.as_ref().map(|desc| {
                view! {
                    <p class="item.module.css:description">{desc}</p>
                }
            })}

            <p class="item.module.css:status" data-status={task.status.clone()}>{task.status}</p>
            <p class="item.module.css:completed" data-completed={task.completed.to_string()}>
                {"Completed: "}{if task.completed { "Yes" } else { "No" }}
            </p>
        </div>
    }
}
