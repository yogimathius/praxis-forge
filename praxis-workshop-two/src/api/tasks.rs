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

    // Try to parse and print the raw text
    console_log(&format!("Raw response: {}", text));

    // Then attempt to parse it
    serde_json::from_str(&text)
        .map_err(|e| format!("Deserialize error: {}. Raw response: {}", e, text))
}

pub async fn create_task(task: Task) -> Result<Task, String> {
    // Create a wrapper struct or use serde_json::Value
    let payload = serde_json::json!({
        "task": task
    });

    Client::new()
        .post("http://localhost:4000/api/tasks")
        .json(&payload)
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
