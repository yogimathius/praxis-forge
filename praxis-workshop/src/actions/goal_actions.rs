use crate::graphql::queries::goals::Goal;
use crate::services::service_context::ServiceContext;
use leptos::prelude::*;

#[allow(warnings)]
pub fn create_goal_action(service: ServiceContext) -> Action<Goal, Result<Goal, String>> {
    create_action(move |goal: &Goal| {
        let goal = goal.clone();
        let service = service.clone();
        async move { service.0.create_goal(goal).await }
    })
}

// Define update and delete similarly 