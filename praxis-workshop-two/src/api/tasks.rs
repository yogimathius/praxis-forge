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

pub async fn update_task(task: Task) -> Result<Task, String> {
    Client::new()
        .put(&format!("http://localhost:4000/api/tasks/{}", task.id))
        .json(&task)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Task>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn delete_task(id: i32) -> Result<(), String> {
    Client::new()
        .delete(&format!("http://localhost:4000/api/tasks/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?;
    Ok(())
}
