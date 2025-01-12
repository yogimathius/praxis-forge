use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

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
    let (task, set_task) = create_signal(task);
    let (status, set_status) = create_signal(task.get().status);
    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_title, set_edit_title) = create_signal(task.get().title);
    let (edit_description, set_edit_description) =
        create_signal(task.get().description.unwrap_or_default());

    let title_input = create_node_ref();
    let desc_input = create_node_ref();

    const STATUSES: &[&str] = &["pending", "in_progress", "completed"];

    let handle_save = move |_| {
        let mut updated_task = task.get();
        updated_task.title = edit_title.get();
        updated_task.description = Some(edit_description.get());
        on_edit.call(updated_task);
        set_is_editing.set(false);
    };

    view! {
        <div class="taskItem">
            <div class="wrapper">
                <div class="taskContent">
                    {move || if is_editing.get() {
                        view! {
                            <input
                                _ref=title_input
                                type="text"
                                value=edit_title.get()
                                on:change=move |ev| {
                                    set_edit_title.set(event_target_value(&ev));
                                }
                            />
                            <input
                                _ref=desc_input
                                type="text"
                                value=edit_description.get()
                                on:change=move |ev| {
                                    set_edit_description.set(event_target_value(&ev));
                                }
                            />
                        }
                    } else {
                        view! {
                            <p class="taskTitle">{move || task.get().title}</p>
                            {move || task.get().description.as_ref().map(|desc| {
                                view! {
                                    <p class="description">{desc}</p>
                                }
                            })}
                        }
                    }}
                </div>
            </div>
            <div class="rightContent">
                <select
                    class="statusSelect"
                    on:change=move |ev| {
                        let new_status = event_target_value(&ev);
                        set_status.set(new_status.clone());
                        let mut updated_task = task.get();
                        updated_task.status = new_status;
                        on_toggle.call(updated_task);
                    }
                    prop:value=move || status.get()
                >
                    {STATUSES.iter().map(|status| {
                        view! {
                            <option value={status.to_string()}>
                                {status.to_string().replace("_", " ")}
                            </option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
                <div class="actions">
                    {move || if is_editing.get() {
                        view! {
                            <button
                                class="button saveButton"
                                on:click=handle_save
                            >
                                "Save"
                            </button>
                        }
                    } else {
                        view! {
                            <button
                                class="button editButton"
                                on:click=move |_| set_is_editing.set(true)
                            >
                                "Edit"
                            </button>
                        }
                    }}
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
