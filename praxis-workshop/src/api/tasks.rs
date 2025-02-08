use leptos::leptos_dom::logging::console_log;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub description: Option<String>,
    pub status: String,
    pub goal_id: Option<i32>,
}

pub async fn fetch_tasks() -> Result<Vec<Task>, String> {
    let response = Client::new()
        .get("http://localhost:4000/api/tasks")
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    // Get the response body as text first
    let text = response
        .text()
        .await
        .map_err(|e| format!("Error reading response text: {}", e))?;

    // Then attempt to parse it
    serde_json::from_str(&text)
        .map_err(|e| format!("Deserialize error: {}. Raw response: {}", e, text))
}

pub async fn create_task(task: Task) -> Result<Task, String> {
    let payload = serde_json::json!({
        "task": {
            "title": task.title,
            "description": task.description,
            "status": task.status,
            "completed": task.completed,
            "goal_id": task.goal_id
        }
    });

    let response = Client::new()
        .post("http://localhost:4000/api/tasks")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // Get the response body as text first for debugging
    let text = response
        .text()
        .await
        .map_err(|e| format!("Error reading response text: {}", e))?;

    console_log(&format!("Create task response: {}", text));

    // Then parse it
    serde_json::from_str(&text)
        .map_err(|e| format!("Deserialize error: {}. Raw response: {}", e, text))
}

pub async fn update_task(id: i32, task: Task) -> Result<Task, String> {
    let payload = serde_json::json!({
        "task": {
            "title": task.title,
            "completed": task.completed,
            "description": task.description,
            "status": task.status
        }
    });
    console_log(&format!("Sending update payload: {}", payload));

    Client::new()
        .put(&format!("http://localhost:4000/api/tasks/{}", id))
        .json(&payload)
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
