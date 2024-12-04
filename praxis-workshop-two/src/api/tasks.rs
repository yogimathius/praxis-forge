use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}

pub async fn fetch_tasks() -> Result<Vec<Task>, String> {
    Client::new()
        .get("http://localhost:4000/api/tasks")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Task>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_task(task: Task) -> Result<Task, String> {
    Client::new()
        .post("http://localhost:4000/api/tasks")
        .json(&task)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Task>()
        .await
        .map_err(|e| e.to_string())
}

// Then in your state/tasks.rs, you'd use these with Leptos resources/actions:
use leptos::*;

pub fn use_tasks() -> Resource<(), Result<Vec<Task>, String>> {
    create_resource(|| (), |_| async move { fetch_tasks().await })
}

pub fn create_task_action() -> Action<Task, Result<Task, String>> {
    create_action(|task: &Task| {
        let task = task.clone();
        async move { create_task(task).await }
    })
}
