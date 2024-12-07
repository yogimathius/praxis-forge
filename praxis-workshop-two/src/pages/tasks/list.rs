use leptos::*;

use crate::api::tasks::Task;
use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::tasks::use_tasks;

#[component]
pub fn TasksListPage() -> impl IntoView {
    let (tasks, create, update, delete) = use_tasks();
    let on_add = move |title| {
        create.dispatch(Task {
            id: 0, // or generate a unique ID if needed
            title,
            description: None,
            status: "pending".to_string(),
        })
    };
    view! {
        <>
            <h1>"Tasks Manager"</h1>
            <TaskForm on_add=on_add />
            <TasksList />
        </>
    }
}
