use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::api::goals::Goal;

#[wasm_bindgen(module = "/src/components/goal/item.module.css")]
extern "C" {}

#[component]
pub fn GoalItem(
    goal: Goal,
    #[prop(into)] on_toggle: Callback<Goal>,
    #[prop(into)] on_delete: Callback<Goal>,
    #[prop(into)] on_edit: Callback<Goal>,
) -> impl IntoView {
    let (goal, _) = create_signal(goal);
    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_title, set_edit_title) = create_signal(goal.get().title);
    let (edit_description, set_edit_description) =
        create_signal(goal.get().description.unwrap_or_default());

    let title_input = create_node_ref();
    let desc_input = create_node_ref();

    let handle_save = move |_| {
        let mut updated_goal = goal.get();
        updated_goal.title = edit_title.get();
        updated_goal.description = Some(edit_description.get());
        on_edit.call(updated_goal);
        set_is_editing.set(false);
    };

    view! {
        <div class="goalItem">
            <div class="wrapper">
                <div class="goalContent">
                    {move || if is_editing.get() {
                        view! {
                            <input
                                _ref=title_input
                                type="text"
                                class="editInput"
                                value=edit_title.get()
                                on:change=move |ev| {
                                    set_edit_title.set(event_target_value(&ev));
                                }
                            />
                            <input
                                _ref=desc_input
                                type="text"
                                class="editInput"
                                value=edit_description.get()
                                on:change=move |ev| {
                                    set_edit_description.set(event_target_value(&ev));
                                }
                            />
                        }
                    } else {
                        view! {
                            <h3 class="goalTitle">{move || goal.get().title}</h3>
                            {move || goal.get().description.as_ref().map(|desc| {
                                view! {
                                    <p class="description">{desc}</p>
                                }
                            })}
                            <div class="progressInfo">
                                <span class="progressText">
                                    "Tasks completed: "
                                    {move || goal.get().tasks_completed}
                                    " / "
                                    {move || goal.get().tasks_required}
                                </span>
                                <div class="progressBar">
                                    <div
                                        class="progressFill"
                                        style:width=move || {
                                            let progress = if goal.get().tasks_required > 0 {
                                                (goal.get().tasks_completed as f32 / goal.get().tasks_required as f32 * 100.0) as i32
                                            } else {
                                                0
                                            };
                                            format!("{}%", progress)
                                        }
                                    ></div>
                                </div>
                            </div>
                        }
                    }}
                </div>
            </div>
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
                    on:click=move |_| on_delete.call(goal.get())
                >
                    "Delete"
                </button>
            </div>
        </div>
    }
}
