use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::task::item::TaskItem;
use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::tasks::Task;

#[wasm_bindgen(module = "/src/components/task/list.module.css")]
extern "C" {}

#[component]
pub fn TasksList(tasks: Vec<Task>, goals: ReadSignal<Vec<Goal>>) -> impl IntoView {
    view! {
        <div class="tasksContainer">
            <h2 class="listTitle">"Your Tasks"</h2>
            <div class="tasksList">
                {tasks
                    .iter()
                    .map(|task| {
                        view! {
                            <TaskItem
                                task=task.clone()
                                goals=goals
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
