use leptos::*;

use crate::api::goals::Goal;
use crate::components::goal::form::GoalForm;
use crate::components::goal::list::GoalsList;
use crate::state::use_goals::use_goals;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/goals/goals.module.css")]
extern "C" {}

#[component]
pub fn GoalsListPage() -> impl IntoView {
    let (goals, create, update, delete, refetch) = use_goals();

    let refetch_create = refetch.clone();
    let refetch_toggle = refetch.clone();
    let refetch_delete = refetch.clone();
    let refetch_update = refetch.clone();

    let on_add = move |title: String, description: String| {
        let refetch = refetch_create.clone();
        spawn_local(async move {
            create.dispatch(Goal {
                id: 0,
                title,
                description: Some(description),
                tasks_required: 0,
                tasks_completed: 0,
            });

            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(300),
            );
        });
    };

    let on_toggle = move |goal: Goal| {
        let refetch = refetch_toggle.clone();
        spawn_local(async move {
            update.dispatch(goal);
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    let on_delete = move |goal: Goal| {
        let refetch = refetch_delete.clone();
        spawn_local(async move {
            delete.dispatch(goal.id);
            // Clone before the timeout
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch(); // Use without clone here
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    let on_edit = move |goal: Goal| {
        let refetch = refetch_update.clone();
        spawn_local(async move {
            update.dispatch(goal);
            let refetch = refetch.clone();
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });
    };

    view! {
        <div class="container">
            <h2 class="dashboardTitle">"The Anvil"</h2>
            <p class="dashboardSubtitle">"Mold your goals on the anvil of determination."</p>
            <GoalForm on_add=move |title, description| on_add(title, description) />
            {
                let goals = goals.clone();
                move || -> View {
                    let goals = goals.get().clone();
                    let on_toggle = on_toggle.clone();
                    let on_delete = on_delete.clone();
                    let on_edit = on_edit.clone();

                    view! {
                        <div>
                            <GoalsList
                                goals=goals
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        </div>
                    }.into_view()
                }
            }
        </div>
    }
}
