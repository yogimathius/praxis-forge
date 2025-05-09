use crate::components::progress::ProgressBar;
use crate::graphql::queries::goals::Goal;
use leptos::prelude::*;

#[component]
pub fn GoalItem(
    goal: Goal,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Goal, Result<Goal, String>>,
) -> impl IntoView {
    let (goal, _) = signal(goal);
    let (is_editing, set_is_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(goal.get_untracked().title);
    let (edit_description, set_edit_description) =
        signal(goal.get_untracked().description.unwrap_or_default());

    let title_input = NodeRef::new();
    let desc_input = NodeRef::new();

    let handle_save = move |_| {
        let mut updated_goal = goal.get();
        updated_goal.title = edit_title.get();
        updated_goal.description = Some(edit_description.get());
        let _ = on_edit.dispatch(updated_goal);
        set_is_editing.set(false);
    };

    view! {
        <div class="bg-glass border border-spark-30 p-8 hover-lift transition-all duration-300 shadow-spark-sm w-full rounded-lg">
            <div class="flex flex-col gap-6">
                <div class="flex justify-between items-center gap-6">
                    <div class="flex-1">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <h3 class="text-2xl font-bold text-spark">
                                        {move || goal.get().title}
                                    </h3>
                                }
                            }
                        >
                            <input
                                node_ref=title_input
                                type="text"
                                class="bg-glass border border-spark-30 rounded-lg p-4 w-full text-ash text-xl font-medium focus:border-spark focus:shadow-spark-sm"
                                value=edit_title.get()
                                on:change=move |ev| {
                                    set_edit_title.set(Some(event_target_value(&ev)));
                                }
                            />
                        </Show>
                    </div>
                    <div class="flex gap-4">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <button
                                        class="btn btn-spark"
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
                                let _ = on_delete.dispatch(goal.get().id.unwrap());
                            }
                        >
                            "Delete"
                        </button>
                    </div>
                </div>

                <Show
                    when=move || is_editing.get()
                    fallback=move || {
                        let description = goal
                            .get()
                            .description
                            .clone()
                            .unwrap_or_else(|| "No description provided".to_string());
                        let description_class = if goal.get().description.is_some() {
                            "text-ash text-lg"
                        } else {
                            "text-ash opacity-60 italic text-lg"
                        };

                        view! {
                            <div class="bg-glass-dark bg-opacity-30 rounded-lg p-4 border border-spark-20">
                                <p class=description_class>{description}</p>
                            </div>
                        }
                    }
                >
                    <textarea
                        node_ref=desc_input
                        class="bg-glass border border-spark-30 rounded-lg p-4 w-full text-ash min-h-[100px] text-lg focus:border-spark focus:shadow-spark-sm"
                        prop:value=edit_description.get()
                        on:change=move |ev| {
                            set_edit_description.set(event_target_value(&ev));
                        }
                    ></textarea>
                </Show>

                <div class="bg-glass-darker rounded-lg p-4 border border-spark-30">
                    {move || {
                        let current_goal = goal.get();
                        let goals_vec = vec![current_goal.clone()];
                        view! { <ProgressBar goals=goals_vec /> }
                    }}
                </div>
            </div>
        </div>
    }
}
