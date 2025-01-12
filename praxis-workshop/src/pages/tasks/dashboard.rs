use leptos::*;

use crate::api::tasks::Task;
use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::tasks::use_tasks;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/tasks/dashboard.module.css")]
extern "C" {}

#[component]
pub fn TasksListPage() -> impl IntoView {
    let (tasks, create, update, delete, refetch) = use_tasks();

    let refetch_create = refetch.clone();
    let refetch_toggle = refetch.clone();
    let refetch_delete = refetch.clone();
    let refetch_update = refetch.clone();

    let on_add = move |title: String, description: String| {
        let refetch = refetch_create.clone();
        spawn_local(async move {
            create.dispatch(Task {
                id: 0,
                title,
                description: Some(description),
                status: "pending".to_string(),
                completed: false,
            });
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    let on_toggle = move |task: Task| {
        let refetch = refetch_toggle.clone();
        spawn_local(async move {
            update.dispatch(task);
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    let on_delete = move |task: Task| {
        let refetch = refetch_delete.clone();
        spawn_local(async move {
            delete.dispatch(task.id);
            // Clone before the timeout
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch(); // Use without clone here
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    let on_edit = move |task: Task| {
        let refetch = refetch_update.clone();
        spawn_local(async move {
            update.dispatch(task);
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    view! {
        <div class="container">
            <TaskForm on_add=move |title, description| on_add(title, description) />
            {
                let tasks = tasks.clone();
                move || -> View {
                    let tasks = tasks.get().clone();
                    let on_toggle = on_toggle.clone();
                    let on_delete = on_delete.clone();
                    let on_edit = on_edit.clone();

                    view! {
                        <div>
                            <TasksList
                                tasks=tasks
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        </div>
                    }.into_view()
                }
            }
        </div>
    }
}
