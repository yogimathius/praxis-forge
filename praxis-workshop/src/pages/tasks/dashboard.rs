use crate::components::task::form::TaskForm;
use crate::components::task::list::TasksList;
use crate::state::use_goals::{use_goals, GoalsState};
use crate::state::use_tasks::{use_tasks, TasksState};
use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/tasks/dashboard.module.css")]
extern "C" {}

#[component]
pub fn TasksListPage() -> impl IntoView {
    let TasksState {
        tasks,
        loading: _,
        error: _,
        create,
        update,
        delete,
        refetch,
    } = use_tasks();
    let GoalsState {
        goals,
        loading: _,
        error: _,
        create: _,
        update: _,
        delete: _,
        refetch: _,
    } = use_goals();

    view! {
        <div class="container">
            <h2 class="dashboardTitle">"Forge Operations"</h2>
            <p class="dashboardSubtitle">"Shape your tasks into achievements, one strike at a time."</p>
            <TaskForm
                create=create.clone()
                refetch=refetch.clone()
                goals=goals
            />
            <div>
                <TasksList
                    tasks=tasks.get()
                    on_toggle=update
                    on_delete=delete
                    on_edit=update
                    goals=goals
                />
            </div>
        </div>
    }
}
