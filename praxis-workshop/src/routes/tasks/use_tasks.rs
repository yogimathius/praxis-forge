use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew::{use_effect_with, use_state};

use crate::{create_task, delete_task, fetch_tasks, update_task, Task};

pub struct UseTasks {
    pub tasks: UseStateHandle<Vec<Task>>,
    pub error: UseStateHandle<Option<String>>,
    pub on_add: Callback<String>,
    pub on_toggle: Callback<Task>,
    pub on_delete: Callback<Task>,
}

#[hook]
pub fn use_tasks() -> UseTasks {
    let tasks = use_state(|| Vec::new());
    let error = use_state(|| None);

    // Fetch tasks on mount
    {
        let tasks = tasks.clone();
        let error = error.clone();
        console::log_1(&"Fetching tasks on component mount".into());
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_tasks().await {
                    Ok(fetched_tasks) => {
                        console::log_1(&format!("Fetched tasks: {:?}", fetched_tasks).into());
                        tasks.set(fetched_tasks);
                    }
                    Err(err) => {
                        console::error_1(&format!("Error fetching tasks: {}", err).into());
                        error.set(Some(err));
                    }
                }
            });
            || ()
        });
    }

    let on_add = {
        let tasks = tasks.clone();
        let error = error.clone();
        Callback::from(move |title: String| {
            let tasks = tasks.clone();
            let error = error.clone();
            spawn_local(async move {
                match create_task(&title).await {
                    Ok(new_task) => {
                        tasks.set([(*tasks).clone(), vec![new_task]].concat());
                        error.set(None);
                    }
                    Err(err) => error.set(Some(err)),
                }
            });
        })
    };

    let on_toggle = {
        let tasks = tasks.clone();
        let error = error.clone();
        Callback::from(move |task: Task| {
            let tasks = tasks.clone();
            let error = error.clone();
            spawn_local(async move {
                let mut updated_task = task.clone();
                updated_task.completed = !updated_task.completed;
                match update_task(&updated_task).await {
                    Ok(updated_task) => {
                        tasks.set(
                            (*tasks)
                                .iter()
                                .map(|t| {
                                    if t.id == updated_task.id {
                                        updated_task.clone()
                                    } else {
                                        t.clone()
                                    }
                                })
                                .collect(),
                        );
                        error.set(None);
                    }
                    Err(err) => error.set(Some(err)),
                }
            });
        })
    };

    let on_delete = {
        let tasks = tasks.clone();
        let error = error.clone();
        Callback::from(move |task: Task| {
            let tasks = tasks.clone();
            let error = error.clone();
            spawn_local(async move {
                match delete_task(&task.id.to_string()).await {
                    Ok(_) => tasks.set(
                        (*tasks)
                            .clone()
                            .into_iter()
                            .filter(|t| t.id != task.id)
                            .collect(),
                    ),
                    Err(err) => error.set(Some(err)),
                }
            });
        })
    };

    UseTasks {
        tasks,
        error,
        on_add,
        on_toggle,
        on_delete,
    }
}
