use super::tasks::Task;
use crate::graphql::schema::schema;

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema = "forge", graphql_type = "RootQueryType")]
pub struct GoalsQuery {
    pub goals: Option<Vec<Option<Goal>>>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct Goal {
    pub id: Option<cynic::Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tasks_required: Option<i32>,
    pub tasks_completed: Option<i32>,
    pub tasks: Option<Vec<Option<Task>>>,
}
