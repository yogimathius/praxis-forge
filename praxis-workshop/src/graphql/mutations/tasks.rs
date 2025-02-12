use crate::graphql::queries::tasks::Task;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql")]
pub struct CreateTaskMutation {
    #[arguments(title: title, description: description)]
    pub create_task: Option<Task>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql")]
pub struct UpdateTaskMutation {
    #[arguments(id: id, title: title, description: description)]
    pub update_task: Task,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "schema.graphql")]
pub struct DeleteTaskMutation {
    #[arguments(id: id)]
    pub delete_task: Task,
}

impl cynic::Operation for DeleteTaskMutation {
    type Variables = cynic::Id;
}
