use super::tasks::Task;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "../forge-core/schema.graphql",
    graphql_type = "RootQueryType"
)]
pub struct GoalsQuery {
    pub goals: Option<Vec<Goal>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema_path = "../forge-core/schema.graphql")]
pub struct Goal {
    pub id: Option<cynic::Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tasks_required: Option<i32>,
    pub tasks_completed: Option<i32>,
    pub tasks: Option<Vec<Task>>,
}
