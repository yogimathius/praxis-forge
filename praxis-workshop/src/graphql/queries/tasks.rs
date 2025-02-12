use crate::graphql::schema::schema;

#[derive(cynic::QueryFragment, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct Task {
    pub id: Option<cynic::Id>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub status: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct TasksQuery {
    pub tasks: Option<Vec<Task>>,
}
