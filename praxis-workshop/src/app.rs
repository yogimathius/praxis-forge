use serde::{Deserialize, Serialize};
use tauri_app_ui::{TaskForm, TaskList};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew::{use_effect_with, use_state};

use tauri_app_ui::api::{create_task, fetch_tasks, update_task, Task};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let tasks = use_state(|| Vec::new());
    let error = use_state(|| None);
    console::log_1(&"Initializing App component".into());
    // Fetch tasks on component mount
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
                        println!("Updated task: {:?}", updated_task);
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

    html! {
        <div>
            <h1>{"Yew Task List"}</h1>
            <TaskForm on_add={on_add} />
            if let Some(err) = (*error).clone() {
                <p style="color: red;">{"Error: "}{err}</p>
            }
            <TaskList tasks={(*tasks).clone()} on_toggle={on_toggle.clone()} />
        </div>
    }
}
