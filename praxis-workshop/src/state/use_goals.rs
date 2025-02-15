use crate::graphql::queries::goals::Goal;
use crate::services::service_context::use_service;
use leptos::*;
use wasm_bindgen_futures::spawn_local;

pub fn use_goals() -> (
    ReadSignal<Vec<Goal>>,
    // Create operation
    Action<Goal, Result<Goal, String>>,
    // Update operation
    Action<Goal, Result<Goal, String>>,
    // Delete operation
    Action<cynic::Id, Result<(), String>>,
    impl Fn() + Clone + 'static,
) {
    let service = use_service();
    let (goals, set_goals) = create_signal(Vec::new());

    // Initial fetch using service
    spawn_local({
        let service = service.0.clone();
        async move {
            if let Ok(goal_list) = service.fetch_goals().await {
                set_goals.set(goal_list);
            }
        }
    });

    // Actions using service
    let create = create_action(|goal: &Goal| {
        let goal = goal.clone();
        let service = service.0.clone();
        async move { service.create_goal(goal).await }
    });

    let update = create_action(|goal: &Goal| {
        let goal = goal.clone();
        let service = service.0.clone();
        async move { service.update_goal(goal.id.unwrap_or_default(), goal).await }
    });

    let delete = create_action(|id: &cynic::Id| {
        let id = id.clone();
        let service = service.0.clone();
        async move { service.delete_goal(id).await }
    });

    let refetch = {
        let service = service.0.clone();
        move || {
            let service = service.clone();
            spawn_local(async move {
                if let Ok(goal_list) = service.fetch_goals().await {
                    set_goals.set(goal_list);
                }
            });
        }
    };

    (goals, create, update, delete, refetch)
}
