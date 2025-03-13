use leptos::prelude::*;

use crate::components::goal::item::GoalItem;
use crate::graphql::queries::goals::Goal;

#[component]
pub fn GoalsList(
    goals: Vec<Goal>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Goal, Result<Goal, String>>,
) -> impl IntoView {
    let goals_for_check = goals.clone();

    view! {
        <Show
            when=move || !goals_for_check.is_empty()
            fallback=|| {
                view! {
                    <div class="border-2 border-dashed border-spark-30 rounded-xl p-8 text-center">
                        <h3 class="text-center text-xl font-bold text-spark mb-4">
                            "No goals yet"
                        </h3>
                        <p class="text-ash opacity-80">
                            "Add your first goal to begin your journey."
                        </p>
                    </div>
                }
            }
        >
            <div class="space-y-6">
                {goals
                    .iter()
                    .map(|goal| {
                        view! { <GoalItem goal=goal.clone() on_delete=on_delete on_edit=on_edit /> }
                    })
                    .collect_view()}
            </div>
        </Show>
    }
}
