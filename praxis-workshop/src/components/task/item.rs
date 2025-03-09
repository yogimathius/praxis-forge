use leptos::prelude::*;
use wasm_bindgen_test::console_log;

use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::tasks::Task;

#[component]
pub fn TaskItem(
    task: Task,
    goals: ReadSignal<Vec<Goal>>,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
) -> impl IntoView {
    let (task, _) = signal(task);

    let (status, set_status) = signal(task.get_untracked().status);
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
        <div class="bg-glass border border-orange-30 rounded-xl p-6 hover-lift transition-all duration-300 mb-6">
            <div class="flex justify-between items-start gap-8 flex-wrap md:flex-nowrap">
                <div class="flex-1 flex flex-col gap-3">
                    <Show
                        when=move || is_editing.get()
                        fallback=move || {
                            view! {
                                <h3 class="text-xl font-bold text-ash">
                                    {move || task.get().title}
                                </h3>
                                {move || {
                                    task
                                        .get()
                                        .description
                                        .as_ref()
                                        .map(|desc| {
                                            view! {
                                                <p class="text-ash opacity-80 mt-2">{desc.clone()}</p>
                                            }
                                        })
                                }}
                            }
                        }
                    >
                        <input
                            node_ref=title_input
                            type="text"
                            class="bg-glass border border-orange-30 rounded-lg p-4 w-full text-ash mb-3 focus:border-orange focus:shadow-orange-sm"
                            value=edit_title.get()
                            on:change=move |ev| {
                                set_edit_title.set(Some(event_target_value(&ev)));
                            }
                        />
                        <input
                            node_ref=desc_input
                            type="text"
                            class="bg-glass border border-orange-30 rounded-lg p-4 w-full text-ash focus:border-orange focus:shadow-orange-sm"
                            value=edit_description.get()
                            on:change=move |ev| {
                                set_edit_description.set(event_target_value(&ev));
                            }
                        />
                    </Show>
                </div>
                <div class="flex items-center gap-4 flex-wrap md:flex-nowrap mt-2 md:mt-0">
                    <select
                        class=move || {
                            format!(
                                "task-status-select status-{}",
                                task.get().status.unwrap_or_default(),
                            )
                        }
                        prop:value=move || task.get().status.unwrap_or_default()
                        on:change=move |ev| {
                            let mut updated_task = task.get();
                            let new_status = event_target_value(&ev);
                            console_log!("Status change triggered with value: {}", &new_status);
                            updated_task.status = Some(new_status);
                            console_log!("Updated task before dispatch: {:?}", &updated_task);
                            let _ = on_toggle.dispatch(updated_task);
                        }
                    >
                        <option value="pending">"Pending"</option>
                        <option value="in_progress">"In Progress"</option>
                        <option value="completed">"Completed"</option>
                    </select>
                    <div class="flex gap-3">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <button
                                        class="btn btn-orange btn-sm"
                                        on:click=move |_| set_is_editing.set(true)
                                    >
                                        "Edit"
                                    </button>
                                }
                            }
                        >
                            <button class="btn btn-spark btn-sm" on:click=handle_save>
                                "Save"
                            </button>
                        </Show>
                        <button
                            class="btn btn-red btn-sm"
                            on:click=move |_| {
                                let _ = on_delete.dispatch(task.get().id.unwrap());
                            }
                        >
                            "Delete"
                        </button>
                    </div>
                    {move || {
                        goal_name
                            .get()
                            .map(|name| {
                                view! {
                                    <span class="bg-spark-15 text-spark border border-spark-30 rounded-full px-3 py-1 text-sm font-medium">
                                        {name}
                                    </span>
                                }
                            })
                    }}
                </div>
            </div>
        </div>
    }
}
