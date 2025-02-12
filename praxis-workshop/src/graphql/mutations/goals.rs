use crate::graphql::queries::goals::Goal;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "../forge-core/schema.graphql")]
pub struct CreateGoalMutation {
    #[arguments(title: title, description: description, tasksRequired: tasks_required)]
    pub create_goal: Option<Goal>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "../forge-core/schema.graphql")]
pub struct UpdateGoalMutation {
    #[arguments(id: id, title: title, description: description, tasksRequired: tasks_required)]
    pub update_goal: Goal,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "../forge-core/schema.graphql")]
pub struct DeleteGoalMutation {
    #[arguments(id: id)]
    pub delete_goal: Goal,
}
