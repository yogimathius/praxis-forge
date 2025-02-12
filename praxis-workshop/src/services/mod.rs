pub mod graphql_service;
pub mod service_context;
use crate::graphql::queries::{goals::Goal, tasks::Task};

// Define the trait that both implementations will follow
#[async_trait::async_trait]
pub trait DataService {
    async fn fetch_tasks(&self) -> Result<Vec<Task>, String>;
    async fn create_task(&self, task: Task) -> Result<Task, String>;
    async fn update_task(&self, id: cynic::Id, task: Task) -> Result<Task, String>;
    async fn delete_task(&self, id: cynic::Id) -> Result<(), String>;

    async fn fetch_goals(&self) -> Result<Vec<Goal>, String>;
    async fn create_goal(&self, goal: Goal) -> Result<Goal, String>;
    async fn update_goal(&self, id: cynic::Id, goal: Goal) -> Result<Goal, String>;
    async fn delete_goal(&self, id: cynic::Id) -> Result<(), String>;
}
