use leptos::*;
use leptos_dom::logging::console_log;

use crate::api::tasks::Task;
use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::tasks::use_tasks;

#[component]
pub fn TasksListPage() -> impl IntoView {
    let (tasks, create, update, delete) = use_tasks();
    let on_add = move |title| {
        create.dispatch(Task {
            id: 0,
            title,
            description: None,
            status: "pending".to_string(),
            completed: false,
        })
    };

    view! {
        <>
            <h1>"Tasks Manager"</h1>
            <TaskForm on_add=on_add />
            {move || match tasks.get() {
                None => view! { <div>"Loading..."</div> },
                Some(Ok(tasks)) => {
                    console_log(&format!("Tasks received: {:?}", tasks));
                    view! { <div><TasksList tasks=tasks /></div> }
                },
                Some(Err(err)) => {
                    console_log(&format!("Error details: {:?}", err));
                    view! { <div>"Error loading tasks: " {err}</div> }
                },
            }}
        </>
    }
}
