use leptos::prelude::*;
use wasm_bindgen_test::console_log;

use crate::graphql::queries::tasks::Task;

#[component]
pub fn TaskItem(
    task: Task,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
) -> impl IntoView {
    let (task, _) = signal(task);
    let (is_editing, set_is_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(task.get_untracked().title);
    let (edit_description, set_edit_description) =
        signal(task.get_untracked().description.unwrap_or_default());

    let title_input = NodeRef::new();
    let desc_input = NodeRef::new();

    const STATUSES: &[&str] = &["pending", "in_progress", "completed"];

    let goal_name = Memo::new(move |_| task.get().goal.as_ref().map(|g| g.title.clone()));

    let handle_save = move |_| {
        let mut updated_task = task.get();
        updated_task.title = edit_title.get();
        updated_task.description = Some(edit_description.get());
        let _ = on_edit.dispatch(updated_task);
        set_is_editing.set(false);
    };

    view! {
        <div class="bg-glass border border-orange-30 shadow-orange-sm rounded-xl p-8 hover-lift transition-all duration-300 mb-6">
            <div class="flex flex-col gap-6">
                <div class="flex justify-between items-center gap-6">
                    <div class="flex-1">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <h3 class="text-2xl font-bold text-ash">
                                        {move || task.get().title}
                                    </h3>
                                }
                            }
                        >
                            <input
                                node_ref=title_input
                                type="text"
                                class="bg-glass border border-orange-30 rounded-lg p-4 w-full text-ash text-xl font-medium focus:border-orange focus:shadow-orange-sm"
                                value=edit_title.get()
                                on:change=move |ev| {
                                    set_edit_title.set(Some(event_target_value(&ev)));
                                }
                            />
                        </Show>
                    </div>

                    {move || {
                        goal_name
                            .get()
                            .map(|name| {
                                view! {
                                    <span class="inline-block bg-spark-15 rounded-full px-6 py-3 text-xl font-medium text-spark border border-spark-30 shadow-spark-sm">
                                        {name}
                                    </span>
                                }
                            })
                    }}
                </div>

                <Show
                    when=move || is_editing.get()
                    fallback=move || {
                        let description = task
                            .get()
                            .description
                            .clone()
                            .unwrap_or_else(|| "No description provided".to_string());
                        let description_class = if task.get().description.is_some() {
                            "text-ash text-lg"
                        } else {
                            "text-ash opacity-60 italic text-lg"
                        };

                        view! {
                            <div class="bg-glass-dark bg-opacity-30 rounded-lg p-4 border border-orange-20">
                                <p class=description_class>{description}</p>
                            </div>
                        }
                    }
                >
                    <textarea
                        node_ref=desc_input
                        class="bg-glass border border-orange-30 rounded-lg p-4 w-full text-ash min-h-[100px] text-lg focus:border-orange focus:shadow-orange-sm"
                        prop:value=edit_description.get()
                        on:change=move |ev| {
                            set_edit_description.set(event_target_value(&ev));
                        }
                    ></textarea>
                </Show>

                <div class="flex justify-between items-center gap-6 flex-wrap md:flex-nowrap">
                    <div class="w-48 relative">
                        <select
                            class=move || {
                                let status = task.get().status.unwrap_or_default();
                                format!(
                                    "task-status-select status-{} w-full rounded-lg p-4 text-lg font-medium border-2 shadow-orange-sm appearance-none",
                                    status,
                                )
                            }
                            on:change=move |ev| {
                                let mut updated_task = task.get();
                                let new_status = event_target_value(&ev);
                                console_log!("Status change triggered with value: {}", &new_status);
                                updated_task.status = Some(new_status);
                                console_log!("Updated task before dispatch: {:?}", &updated_task);
                                let _ = on_toggle.dispatch(updated_task);
                            }
                        >
                            <option
                                value="pending"
                                selected=move || task.get().status.unwrap_or_default() == "pending"
                            >
                                "Pending"
                            </option>
                            <option
                                value="in_progress"
                                selected=move || {
                                    task.get().status.unwrap_or_default() == "in_progress"
                                }
                            >
                                "In Progress"
                            </option>
                            <option
                                value="completed"
                                selected=move || {
                                    task.get().status.unwrap_or_default() == "completed"
                                }
                            >
                                "Completed"
                            </option>
                        </select>
                        <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-4 text-orange">
                            <svg
                                class="h-5 w-5"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                    </div>

                    <div class="flex gap-4">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <button
                                        class="btn btn-orange"
                                        on:click=move |_| set_is_editing.set(true)
                                    >
                                        "Edit"
                                    </button>
                                }
                            }
                        >
                            <button class="btn btn-spark" on:click=handle_save>
                                "Save"
                            </button>
                        </Show>
                        <button
                            class="btn btn-red"
                            on:click=move |_| {
                                let _ = on_delete.dispatch(task.get().id.unwrap());
                            }
                        >
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
