use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use leptos::*;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;

use crate::graphql::mutations::tasks::{create_task, delete_task, update_task, Task};
use crate::graphql::queries::tasks::Task;
use crate::services::service_context::use_service;

pub fn use_tasks() -> (
    ReadSignal<Vec<Task>>,
    // Create operation
    Action<Task, Result<Task, String>>,
    // Update operation
    Action<Task, Result<Task, String>>,
    // Delete operation
    Action<i32, Result<(), String>>,
    impl Fn() + Clone + 'static,
) {
    // Create a signal to store tasks
    let (tasks, set_tasks) = create_signal(Vec::new());

    // Initialize WebSocket connection
    let ws = WebSocket::open("ws://localhost:4000/socket/websocket").unwrap();
    let (mut write, mut read) = ws.split();

    // Handle WebSocket communication
    spawn_local(async move {
        // Join channel
        write
            .send(Message::Text(
                r#"{"topic": "tasks:updates", "event": "phx_join", "payload": {}, "ref": "1"}"#
                    .to_string(),
            ))
            .await
            .unwrap();

        // Read messages
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    if let Some(payload) = value.get("payload") {
                        if let Ok(updated_task) = serde_json::from_value::<Task>(payload.clone()) {
                            let mut current_tasks = tasks.get();
                            if let Some(idx) = current_tasks
                                .iter()
                                .position(|t: &Task| t.id == updated_task.id)
                            {
                                current_tasks[idx] = updated_task;
                                set_tasks.set(current_tasks);
                            }
                        }
                    }
                }
            }
        }
    });

    // Create a resource that fetches tasks
    spawn_local(async move {
        if let Ok(task_list) = fetch_tasks().await {
            set_tasks.set(task_list);
        }
    });

    // Create refetch function
    let refetch = move || {
        spawn_local(async move {
            if let Ok(task_list) = fetch_tasks().await {
                set_tasks.set(task_list);
            }
        });
    };

    // Create task action
    let create = create_action(|task: &Task| {
        let task = task.clone();
        async move { create_task(task).await }
    });

    // Update task action
    let update = create_action(|task: &Task| {
        let task = task.clone();
        async move { update_task(task.id, task).await }
    });

    // Delete task action
    let delete = create_action(|id: &i32| {
        let id = *id;
        async move { delete_task(id).await }
    });

    (tasks, create, update, delete, refetch)
}
