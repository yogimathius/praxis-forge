use leptos::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::goals::{create_goal, delete_goal, fetch_goals, update_goal, Goal};

pub fn use_goals() -> (
    ReadSignal<Vec<Goal>>,
    // Create operation
    Action<Goal, Result<Goal, String>>,
    // Update operation
    Action<Goal, Result<Goal, String>>,
    // Delete operation
    Action<i32, Result<(), String>>,
    impl Fn() + Clone + 'static,
) {
    // Create a signal to store goals
    let (goals, set_goals) = create_signal(Vec::new());

    // Initial fetch of goals
    spawn_local(async move {
        if let Ok(goal_list) = fetch_goals().await {
            set_goals.set(goal_list);
        }
    });

    // Create refetch function
    let refetch = move || {
        spawn_local(async move {
            if let Ok(goal_list) = fetch_goals().await {
                set_goals.set(goal_list);
            }
        });
    };

    // Create goal action
    let create = create_action(|goal: &Goal| {
        let goal = goal.clone();
        async move { create_goal(goal).await }
    });

    // Update goal action
    let update = create_action(|goal: &Goal| {
        let goal = goal.clone();
        async move { update_goal(goal.id, goal).await }
    });

    // Delete goal action
    let delete = create_action(|id: &i32| {
        let id = *id;
        async move { delete_goal(id).await }
    });

    (goals, create, update, delete, refetch)
}
