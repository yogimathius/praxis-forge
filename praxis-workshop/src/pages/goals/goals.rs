use crate::components::goal::item::GoalItem;
use crate::state::use_goals::GoalsState;
use leptos::prelude::*;

use crate::components::goal::form::GoalForm;
use crate::components::goal::list::GoalsList;
use crate::state::use_goals::use_goals;

#[component]
pub fn GoalsListPage() -> impl IntoView {
    let GoalsState {
        goals,
        create,
        update,
        delete,
        refetch,
        loading: _,
        error: _,
    } = use_goals();

    view! {
        <div class="max-w-7xl mx-auto p-8 my-8">
            <h2 class="text-4xl font-bold text-spark flex justify-center mb-6 animate-ember-pulse">
                "The Anvil"
            </h2>
            <p class="text-xl flex justify-center text-ash mb-12 animate-slide-up">
                "Mold your goals on the anvil of determination."
            </p>

            <div class="bg-glass-dark rounded-xl border border-spark-30 mb-12 animate-fade-in delay-300">
                <GoalForm create=create.clone() refetch=refetch.clone() />
            </div>

            <div class="bg-glass rounded-xl border border-spark-30 p-8 animate-fade-in delay-500">
                {
                    let goals = goals.clone();
                    move || -> View<_> {
                        let goals = goals.get().clone();
                        let on_delete = delete.clone();
                        let on_edit = update.clone();

                        view! {
                            <div>
                                <h3 class="text-2xl font-bold text-spark text-center mb-8 animate-ember-pulse">
                                    "Your Goals"
                                </h3>
                                {
                                    let goals_for_check = goals.clone();
                                    let goals_for_display = goals;

                                    view! {
                                        <Show
                                            when=move || !goals_for_check.is_empty()
                                            fallback=|| {
                                                view! {
                                                    <div class="border-2 border-dashed border-spark-30 rounded-xl p-8 text-center">
                                                        <h3 class="text-xl font-bold text-spark mb-4">
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
                                                {goals_for_display
                                                    .iter()
                                                    .map(|goal| {
                                                        view! {
                                                            <GoalItem
                                                                goal=goal.clone()
                                                                on_delete=on_delete
                                                                on_edit=on_edit
                                                            />
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        </Show>
                                    }
                                }
                            </div>
                        }
                            .into_view()
                    }
                }
            </div>
        </div>
    }
}
