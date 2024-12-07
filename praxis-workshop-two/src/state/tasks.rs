use leptos::*;

use crate::api::tasks::{create_task, fetch_tasks, Task};

pub fn use_tasks() -> Resource<(), Result<Vec<Task>, String>> {
    create_resource(|| (), |_| async move { fetch_tasks().await })
}

pub fn create_task_action() -> Action<Task, Result<Task, String>> {
    create_action(|task: &Task| {
        let task = task.clone();
        async move { create_task(task).await }
    })
}
