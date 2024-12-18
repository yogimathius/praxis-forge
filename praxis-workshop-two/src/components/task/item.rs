use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

use crate::api::tasks::Task;

#[wasm_bindgen(module = "/src/components/task/item.module.css")]
extern "C" {}

// Helper function for logging
fn console_log(msg: &str) {
    console::log_1(&msg.into());
}

#[component]
pub fn TaskItem(
    task: Task,
    #[prop(into)] on_toggle: Callback<Task>,
    #[prop(into)] on_delete: Callback<Task>,
    #[prop(into)] on_edit: Callback<Task>,
) -> impl IntoView {
    let task = create_rw_signal(task);

    // Log the initial task status
    console_log(&format!("Initial task status: {}", task.get().status));

    // Create a local signal for status, initialized with the task's current status
    let status = create_rw_signal(task.get().status.clone());

    // Define available statuses
    const STATUSES: &[&str] = &["pending", "in_progress", "completed"];

    // Log available statuses and current selection
    console_log(&format!("Available statuses: {:?}", STATUSES));
    console_log(&format!("Current status signal value: {}", status.get()));

    view! {
        <div class="taskItem">
            <div class="wrapper">
                <div class="taskContent">
                    <p class="taskTitle">{move || task.get().title}</p>
                    {move || task.get().description.as_ref().map(|desc| {
                        view! {
                            <p class="description">{desc}</p>
                        }
                    })}
                </div>
            </div>
            <div class="rightContent">
                <select
                    class="statusSelect"
                    on:change=move |ev| {
                        let new_status = event_target_value(&ev);
                        console_log(&format!("Selected new status: {}", new_status));
                        status.set(new_status.clone());

                        let mut updated_task = task.get();
                        updated_task.status = new_status;
                        on_toggle.call(updated_task);
                    }
                    prop:value=move || {
                        let current = status.get();
                        console_log(&format!("Current status in select: {}", current));
                        current
                    }
                >
                    {STATUSES.iter().map(|status| {
                        view! {
                            <option
                                value={status.to_string()}
                                selected={move || status == &task.get().status}
                            >
                                {status.to_string().replace("_", " ")}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
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
        </div>
    }
}
