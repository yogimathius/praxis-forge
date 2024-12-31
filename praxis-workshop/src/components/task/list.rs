use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::api::tasks::Task;
use crate::components::task::item::TaskItem;

#[wasm_bindgen(module = "/src/components/task/list.module.css")]
extern "C" {}

#[component]
pub fn TasksList(
    tasks: Vec<Task>,
    #[prop(into)] on_toggle: Callback<Task>,
    #[prop(into)] on_delete: Callback<Task>,
    #[prop(into)] on_edit: Callback<Task>,
) -> impl IntoView {
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
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
