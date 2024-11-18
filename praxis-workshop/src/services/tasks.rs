use gloo_net::http::Request;
use serde_json::Value;
use web_sys::console;

use crate::services::Task;

const API_BASE_URL: &str = "http://localhost:4000/api";

pub async fn fetch_tasks() -> Result<Vec<Task>, String> {
    console::log_1(&format!("Fetching tasks from {}/tasks", API_BASE_URL).into());

    let response = Request::get(&format!("{}/tasks", API_BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    console::log_1(&format!("Received response with status: {}", response.status()).into());

    let json: Value = response.json().await.map_err(|e| e.to_string())?;
    console::log_1(&format!("Parsed JSON: {:?}", json).into());

    match json {
        Value::Object(map) => {
            if let Some(Value::Array(arr)) = map.get("data") {
                let tasks: Result<Vec<Task>, _> = arr
                    .into_iter()
                    .map(|v| serde_json::from_value(v.clone()))
                    .collect();
                match tasks {
                    Ok(t) => {
                        console::log_1(&format!("Successfully parsed {} tasks", t.len()).into());
                        Ok(t)
                    }
                    Err(e) => {
                        console::error_1(&format!("Error parsing tasks: {}", e).into());
                        Err(e.to_string())
                    }
                }
            } else {
                console::error_1(&"No 'data' field found in response".into());
                Err("No 'data' field found in response".to_string())
            }
        }
        _ => {
            console::error_1(&"Unexpected JSON structure".into());
            Err("Unexpected JSON structure".to_string())
        }
    }
}

pub async fn create_task(title: &str) -> Result<Task, String> {
    let task_data = serde_json::json!({
        "task": {
            "title": title,
            "completed": false
        }
    });

    let response = Request::post(&format!("{}/tasks", API_BASE_URL))
        .json(&task_data)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    console::log_1(&format!("Create task response status: {}", response.status()).into());

    let json: Value = response.json().await.map_err(|e| e.to_string())?;
    console::log_1(&format!("Create task response JSON: {:?}", json).into());

    match json.get("data") {
        Some(task_value) => serde_json::from_value(task_value.clone()).map_err(|e| e.to_string()),
        None => Err("No 'data' field found in create task response".to_string()),
    }
}

pub async fn update_task(task: &Task) -> Result<Task, String> {
    let task_data = serde_json::json!({
        "task": {
            "title": task.title,
            "completed": task.completed
        }
    });

    let response = Request::put(&format!("{}/tasks/{}", API_BASE_URL, task.id))
        .json(&task_data)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    console::log_1(&format!("Update task response status: {}", response.status()).into());

    let json: Value = response.json().await.map_err(|e| e.to_string())?;
    console::log_1(&format!("Update task response JSON: {:?}", json).into());

    match json.get("data") {
        Some(task_value) => serde_json::from_value(task_value.clone()).map_err(|e| e.to_string()),
        None => Err("No 'data' field found in update task response".to_string()),
    }
}

pub async fn delete_task(id: &str) -> Result<(), String> {
    let response = Request::delete(&format!("{}/tasks/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        Ok(())
    } else {
        Err(format!(
            "Failed to delete task with status: {}",
            response.status()
        ))
    }
}
