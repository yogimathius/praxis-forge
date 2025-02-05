use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::api::goals::Goal;
use crate::components::goal::item::GoalItem;

#[wasm_bindgen(module = "/src/components/goal/list.module.css")]
extern "C" {}

#[component]
pub fn GoalsList(
    goals: Vec<Goal>,
    #[prop(into)] on_toggle: Callback<Goal>,
    #[prop(into)] on_delete: Callback<Goal>,
    #[prop(into)] on_edit: Callback<Goal>,
) -> impl IntoView {
    view! {
        <div class="goalsContainer">
            <h2 class="listTitle">"Your Goals"</h2>
            <div class="goalsList">
                {goals
                    .iter()
                    .map(|goal| {
                        view! {
                            <GoalItem
                                goal=goal.clone()
                                on_toggle=on_toggle
                                on_delete=on_delete
                                on_edit=on_edit
                            />
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
