use crate::graphql::mutations::goals::{
    CreateGoalMutation, CreateGoalVariables, DeleteGoalMutation, DeleteGoalVariables,
    UpdateGoalMutation, UpdateGoalVariables,
};
use crate::graphql::mutations::tasks::{
    CreateTaskMutation, CreateTaskVariables, DeleteTaskMutation, DeleteTaskVariables,
    UpdateTaskMutation, UpdateTaskVariables,
};
use crate::graphql::queries::goals::{Goal, GoalsQuery};
use crate::graphql::queries::tasks::{Task, TasksQuery};
use cynic::{MutationBuilder, QueryBuilder};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use wasm_bindgen_test::console_log;

pub struct GraphQLService {
    client: Client,
    endpoint: String,
}

impl GraphQLService {
    pub fn new() -> Self {
        // Create a client with default headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            endpoint: "http://localhost:4000/api/graphql".to_string(),
        }
    }

    pub async fn fetch_tasks(&self) -> Result<Vec<Task>, String> {
        let query = TasksQuery::build(());

        let response = self
            .client
            .post(&self.endpoint)
            .json(&query)
            .send()
            .await
            .map_err(|e| {
                console_log!("Error sending request: {:?}", e);
                e.to_string()
            })?
            .json::<cynic::GraphQlResponse<TasksQuery>>()
            .await
            .map_err(|e| e.to_string())?;

        let data = Self::unwrap_response(response)?;

        Ok(data
            .tasks
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect())
    }

    pub async fn create_task(&self, task: Task) -> Result<Task, String> {
        let operation = CreateTaskMutation::build(CreateTaskVariables {
            title: task.title.unwrap_or_default(),
            description: task.description,
            goal_id: task.goal.and_then(|g| g.id),
            status: task.status,
            completed: task.completed,
        });

        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<CreateTaskMutation>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(response.data.unwrap().create_task.unwrap())
    }

    pub async fn update_task(&self, id: cynic::Id, task: Task) -> Result<Task, String> {
        console_log!("Starting update_task with id: {:?}", id);
        console_log!("Task to update: {:?}", task);

        let operation = UpdateTaskMutation::build(UpdateTaskVariables {
            id,
            title: task.title.unwrap_or_default(),
            description: task.description,
            goal_id: task.goal.and_then(|g| g.id),
            status: task.status.clone(),
            completed: task.completed,
        });

        console_log!("Built mutation variables: {:?}", operation);

        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| {
                console_log!("Error sending request: {:?}", e);
                e.to_string()
            })?
            .json::<cynic::GraphQlResponse<UpdateTaskMutation>>()
            .await
            .map_err(|e| {
                console_log!("Error parsing response: {:?}", e);
                e.to_string()
            })?;

        console_log!("Received response: {:?}", response);

        // Safer unwrapping with error messages
        let data = response.data.ok_or_else(|| {
            console_log!("No data in response");
            "No data received from server".to_string()
        })?;
        let updated_task = data.update_task.ok_or_else(|| {
            console_log!("No task in response data");
            "No task returned from update".to_string()
        })?;

        console_log!("Successfully updated task: {:?}", updated_task);
        Ok(updated_task)
    }

    pub async fn delete_task(&self, id: cynic::Id) -> Result<(), String> {
        let operation = DeleteTaskMutation::build(DeleteTaskVariables { id });
        self.client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<DeleteTaskMutation>>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn fetch_goals(&self) -> Result<Vec<Goal>, String> {
        let query = GoalsQuery::build(());
        let response = self
            .client
            .post(&self.endpoint)
            .json(&query)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<GoalsQuery>>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(response
            .data
            .unwrap()
            .goals
            .unwrap()
            .into_iter()
            .flatten()
            .collect())
    }

    pub async fn create_goal(&self, goal: Goal) -> Result<Goal, String> {
        let operation = CreateGoalMutation::build(CreateGoalVariables {
            title: goal.title.unwrap_or_default(),
            description: goal.description,
            tasks_required: goal.tasks_required.unwrap_or(0),
        });
        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<CreateGoalMutation>>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(response.data.unwrap().create_goal.unwrap())
    }

    pub async fn update_goal(&self, id: cynic::Id, goal: Goal) -> Result<Goal, String> {
        let operation = UpdateGoalMutation::build(UpdateGoalVariables {
            id,
            title: goal.title.unwrap_or_default(),
            description: goal.description,
            tasks_required: goal.tasks_required.unwrap_or(0),
        });
        let response = self
            .client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<UpdateGoalMutation>>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(response.data.unwrap().update_goal.unwrap())
    }

    pub async fn delete_goal(&self, id: cynic::Id) -> Result<(), String> {
        let operation = DeleteGoalMutation::build(DeleteGoalVariables { id });
        self.client
            .post(&self.endpoint)
            .json(&operation)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<cynic::GraphQlResponse<DeleteGoalMutation>>()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

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
