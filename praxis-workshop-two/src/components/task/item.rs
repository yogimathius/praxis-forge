use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::api::tasks::Task;

#[wasm_bindgen(module = "/src/components/task/item.module.css")]
extern "C" {}

#[component]
pub fn TaskItem(
    task: Task,
    #[prop(into)] on_toggle: Callback<Task>,
    #[prop(into)] on_delete: Callback<Task>,
    #[prop(into)] on_edit: Callback<Task>,
) -> impl IntoView {
    let task = create_rw_signal(task);

    view! {
        <div class="taskItem">
            <div class="taskContent">
                <input
                    type="checkbox"
                    class="checkbox"
                    prop:checked=move || task.get().completed
                    on:change=move |_| on_toggle.call(task.get())
                />

                <div class="taskInfo">
                    <p class="taskTitle">{move || task.get().title}</p>

                    {move || task.get().description.as_ref().map(|desc| {
                        view! {
                            <p class="description">{desc}</p>
                        }
                    })}

                    <p class="status">{move || task.get().status}</p>
                </div>
            </div>

            <div class="actions">
                <button
                    class="button editButton"
                    on:click=move |_| on_edit.call(task.get())
                >
                    "Edit"
                </button>
                <button
                    class="button deleteButton"
                    on:click=move |_| on_delete.call(task.get())
                >
                    "Delete"
                </button>
            </div>
        </div>
    }
}
