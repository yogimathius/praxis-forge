use std::rc::Rc;

use leptos::{ev, prelude::*, task::spawn_local};

use crate::graphql::queries::goals::Goal;

#[component]
pub fn GoalForm(
    create: Action<Goal, Result<Goal, String>>,
    refetch: Rc<dyn Fn()>,
) -> impl IntoView {
    let (goal_text, set_goal_text) = signal(String::new());
    let (goal_description, set_goal_description) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (show_success, set_show_success) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = goal_text.get_untracked();
        let current_description = goal_description.get_untracked();
        if current_text.is_empty() {
            return;
        }

        set_is_submitting.set(true);
        let goal = Goal {
            id: None,
            title: Some(current_text),
            description: Some(current_description),
            tasks_required: None,
            tasks_completed: None,
            tasks: None,
        };

        let refetch_fn = refetch.clone();

        spawn_local(async move {
            let _ = create.dispatch(goal);
            set_timeout(
                move || {
                    refetch_fn();
                },
                std::time::Duration::from_millis(100),
            );
        });

        set_goal_text.set(String::new());
        set_goal_description.set(String::new());
        set_show_success.set(true);
        set_is_submitting.set(false);

        set_timeout(
            move || {
                set_show_success.set(false);
            },
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <form class="bg-glass-dark rounded-xl p-8 shadow-spark-sm" on:submit=on_submit>
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-6">
                    <div class="flex flex-col gap-2">
                        <label for="goal-title" class="text-ash text-sm font-medium">
                            Goal Title
                        </label>
                        <input
                            id="goal-title"
                            type="text"
                            class="bg-white/10 border border-spark-30 rounded-lg p-4 text-steel font-medium focus:border-spark focus:shadow-spark-sm"
                            placeholder="Add a new goal..."
                            on:input=move |ev| set_goal_text.set(event_target_value(&ev))
                            prop:value=move || goal_text.get()
                        />
                    </div>

                    <div class="flex flex-col gap-2">
                        <label for="goal-description" class="text-ash text-sm font-medium">
                            Description
                        </label>
                        <textarea
                            id="goal-description"
                            class="bg-white/10 border border-spark-30 rounded-lg p-4 text-steel font-medium focus:border-spark focus:shadow-spark-sm min-h-[80px] resize-y"
                            placeholder="Add a description..."
                            on:input=move |ev| set_goal_description.set(event_target_value(&ev))
                            prop:value=move || goal_description.get()
                        ></textarea>
                    </div>
                </div>

                <div class="flex justify-start">
                    <button
                        type="submit"
                        class="btn btn-spark hover-lift shadow-spark-sm focus:outline-none"
                        disabled=move || is_submitting.get()
                    >
                        {move || if is_submitting.get() { "Adding..." } else { "Add Goal" }}
                    </button>
                </div>
            </div>

            {move || {
                show_success
                    .get()
                    .then(|| {
                        view! {
                            <div class="bg-spark-20 border border-spark rounded-xl p-4 text-center animate-fade-in mt-8">
                                "Goal added successfully!"
                            </div>
                        }
                    })
            }}
        </form>
    }
}
