use crate::graphql::mutations::goals::{
    CreateGoalMutation, DeleteGoalMutation, UpdateGoalMutation,
};
use crate::graphql::mutations::tasks::{
    CreateTaskMutation, DeleteTaskMutation, UpdateTaskMutation,
};
use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::goals::GoalsQuery;
use crate::graphql::queries::tasks::Task;
use crate::graphql::queries::tasks::TasksQuery;
use crate::services::DataService;
use cynic::{Operation, QueryBuilder, QueryFragment};
use reqwest::Client;

pub struct GraphQLService {
    client: Client,
    endpoint: String,
}

#[async_trait::async_trait]
impl DataService for GraphQLService {
    async fn fetch_tasks(&self) -> Result<Vec<Task>, String> {
        let operation = Operation::<TasksQuery, ()>::new(TasksQuery::build(()));
        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await?;
        GraphQLService::unwrap_response(response)
    }

    async fn create_task(&self, task: Task) -> Result<Task, String> {
        let mutation = CreateTaskMutation::build(task);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(response.create_task)
    }

    async fn update_task(&self, id: cynic::Id, task: Task) -> Result<Task, String> {
        let mutation = UpdateTaskMutation::build(id, task);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(response.update_task)
    }

    async fn delete_task(&self, id: cynic::Id) -> Result<(), String> {
        let mutation = DeleteTaskMutation::build(id);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(())
    }

    async fn fetch_goals(&self) -> Result<Vec<Goal>, String> {
        let query = GoalsQuery::build(());
        let response = self.client.post(&self.endpoint).json(&query).send().await?;
        Ok(response.goals)
    }

    async fn create_goal(&self, goal: Goal) -> Result<Goal, String> {
        let mutation = CreateGoalMutation::build(goal);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(response.create_goal)
    }

    async fn update_goal(&self, id: cynic::Id, goal: Goal) -> Result<Goal, String> {
        let mutation = UpdateGoalMutation::build(id, goal);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(response.update_goal)
    }

    async fn delete_goal(&self, id: cynic::Id) -> Result<(), String> {
        let mutation = DeleteGoalMutation::build(id);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&mutation)
            .send()
            .await?;
        Ok(())
    }
}

impl GraphQLService {
    fn unwrap_response<T: cynic::QueryFragment>(
        response: cynic::GraphQlResponse<T>,
    ) -> Result<T, String> {
        if let Some(data) = response.data {
            Ok(data)
        } else if let Some(errors) = response.errors {
            Err(format!("GraphQL Errors: {:?}", errors))
        } else {
            Err("No data or errors returned".into())
        }
    }
}
