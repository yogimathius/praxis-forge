use leptos::*;

use crate::api::tasks::{create_task, delete_task, fetch_tasks, update_task, Task};

pub fn use_tasks() -> (
    Resource<(), Result<Vec<Task>, String>>,
    // Create operation
    Action<Task, Result<Task, String>>,
    // Update operation
    Action<Task, Result<Task, String>>,
    // Delete operation
    Action<i32, Result<(), String>>,
) {
    // Create a resource that fetches tasks
    let tasks = create_resource(
        || (), // Dependencies (empty in this case)
        |_| async move { fetch_tasks().await },
    );

    // Create task action
    let create = create_action(|task: &Task| {
        let task = task.clone();
        async move { create_task(task).await }
    });

    // Update task action
    let update = create_action(|task: &Task| {
        let task = task.clone();
        async move { update_task(task).await }
    });

    // Delete task action
    let delete = create_action(|id: &i32| {
        let id = *id;
        async move { delete_task(id).await }
    });

    (tasks, create, update, delete)
}
