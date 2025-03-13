use leptos::prelude::*;
use thaw::{Button, Card};

use crate::components::progress::ProgressBar;
use crate::graphql::queries::goals::Goal;

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
        <Card class="bg-glass border border-spark-30 p-6 hover-lift transition-all duration-300 shadow-spark-sm">
            <div class="flex flex-col gap-6">
                <div class="flex justify-between items-start gap-4 flex-wrap md:flex-nowrap">
                    <div class="flex-1">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <div class="flex flex-col gap-2">
                                        <h3 class="text-2xl font-bold text-spark">
                                            {move || goal.get().title}
                                        </h3>
                                        {move || {
                                            goal.get()
                                                .description
                                                .as_ref()
                                                .map(|desc| {
                                                    view! { <p class="text-ash opacity-80">{desc.clone()}</p> }
                                                })
                                        }}
                                    </div>
                                }
                            }
                        >
                            <div class="flex flex-col gap-3">
                                <input
                                    node_ref=title_input
                                    type="text"
                                    class="bg-white/10 border border-spark-30 rounded-lg p-4 w-full text-ash focus:border-spark focus:shadow-spark-sm"
                                    value=edit_title.get()
                                    on:change=move |ev| {
                                        set_edit_title.set(Some(event_target_value(&ev)));
                                    }
                                />
                                <textarea
                                    node_ref=desc_input
                                    class="bg-white/10 border border-spark-30 rounded-lg p-4 w-full text-ash focus:border-spark focus:shadow-spark-sm min-h-[80px]"
                                    prop:value=edit_description.get()
                                    on:change=move |ev| {
                                        set_edit_description.set(event_target_value(&ev));
                                    }
                                ></textarea>
                            </div>
                        </Show>
                    </div>
                    <div class="flex gap-3">
                        <Show
                            when=move || is_editing.get()
                            fallback=move || {
                                view! {
                                    <Button
                                        class="btn btn-spark btn-sm"
                                        on:click=move |_| set_is_editing.set(true)
                                    >
                                        "Edit"
                                    </Button>
                                }
                            }
                        >
                            <Button class="btn btn-spark btn-sm" on:click=handle_save>
                                "Save"
                            </Button>
                        </Show>
                        <Button
                            class="btn btn-red btn-sm"
                            on:click=move |_| {
                                let _ = on_delete.dispatch(goal.get().id.unwrap());
                            }
                        >
                            "Delete"
                        </Button>
                    </div>
                </div>

                <div class="bg-glass-darker rounded-lg p-4 border border-orange-30">
                    {move || {
                        let current_goal = goal.get();
                        let goals_vec = vec![current_goal.clone()];
                        view! { <ProgressBar goals=goals_vec /> }
                    }}
                </div>
            </div>
        </Card>
    }
}
