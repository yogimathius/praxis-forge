use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub tasks_required: i32,
    pub tasks_completed: i32,
}

pub async fn fetch_goals() -> Result<Vec<Goal>, String> {
    let response = Client::new()
        .get("http://localhost:4000/api/goals")
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch goals: {}", response.status()));
    }

    let goals: Vec<Goal> = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;
    Ok(goals)
}

pub async fn create_goal(goal: Goal) -> Result<Goal, String> {
    let response = Client::new()
        .post("http://localhost:4000/api/goals")
        .json(&goal)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to create goal: {}", response.status()));
    }

    let goal: Goal = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;
    Ok(goal)
}

pub async fn update_goal(id: i32, goal: Goal) -> Result<Goal, String> {
    let response = Client::new()
        .put(format!("http://localhost:4000/api/goals/{}", id))
        .json(&goal)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to update goal: {}", response.status()));
    }

    let goal: Goal = response
        .json()
        .await
        .map_err(|e| format!("JSON parsing error: {}", e))?;
    Ok(goal)
}

pub async fn delete_goal(id: i32) -> Result<(), String> {
    let response = Client::new()
        .delete(format!("http://localhost:4000/api/goals/{}", id))
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to delete goal: {}", response.status()));
    }

    Ok(())
}
