use crate::state::use_goals::GoalsState;
use leptos::prelude::*;

use crate::components::goal::form::GoalForm;
use crate::components::goal::list::GoalsList;
use crate::state::use_goals::use_goals;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/goals/goals.module.css")]
extern "C" {}

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
        <div class="container">
            <h2 class="dashboardTitle">"The Anvil"</h2>
            <p class="dashboardSubtitle">"Mold your goals on the anvil of determination."</p>
            <GoalForm create=create.clone() refetch=refetch.clone() />
            <div>
                <GoalsList
                    goals=goals.get()
                    on_toggle=update
                    on_delete=delete
                    on_edit=update
                />
            </div>
        </div>
    }
}
