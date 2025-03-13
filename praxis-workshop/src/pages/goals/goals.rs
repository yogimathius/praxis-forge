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
        <div class="max-w-7xl mx-auto p-8">
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-4 items-center">
                    <h2 class="text-4xl font-bold text-spark animate-ember-pulse">"The Anvil"</h2>
                    <p class="text-xl text-ash animate-slide-up">
                        "Mold your goals on the anvil of determination."
                    </p>
                </div>

                <div class="bg-glass-dark rounded-xl border border-spark-30 p-8 animate-fade-in delay-300">
                    <GoalForm create=create.clone() refetch=refetch.clone() />
                </div>

                <div class="bg-glass rounded-xl border border-spark-30 p-8 animate-fade-in delay-500">
                    {
                        let goals = goals.clone();
                        move || {
                            let goals_data = goals.get().clone();

                            view! {
                                <div>
                                    <h3 class="text-2xl font-bold text-spark text-center mb-8 animate-ember-pulse">
                                        "Your Goals"
                                    </h3>
                                    <GoalsList goals=goals_data on_delete=delete on_edit=update />
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}
