use leptos::*;
use leptos::prelude::*;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub status: String,
}

#[component]
pub fn TaskRecommendations(task: Task) -> impl IntoView {
    let (recommendation, set_recommendation) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);

    let recommendations = Action::new(move |_: &()| {
        set_loading.set(true);

        let task_data = format!(
            "Task: {}\nDescription: {}\nStatus: {}",
            task.title, task.description, task.status
        );

        async move {
            let api_key = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .get_item("openai_api_key")
                .unwrap()
                .unwrap_or_default();

            if api_key.is_empty() {
                set_recommendation.set("Please set your API key in settings".to_string());
                set_loading.set(false);
                return String::new();
            }

            let client = Client::new();
            let response = client
                .post("https://api.openai.com/v1/chat/completions")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&serde_json::json!({
                    "model": "gpt-3.5-turbo",
                    "messages": [
                        {
                            "role": "system",
                            "content": "You are an AI assistant for a productivity app."
                        },
                        {
                            "role": "user",
                            "content": task_data
                        }
                    ],
                    "temperature": 0.7,
                    "max_tokens": 300
                }))
                .send()
                .await;

            let result = match response {
                Ok(res) => {
                    if res.status().is_success() {
                        let json: serde_json::Value = res.json().await.unwrap_or_default();
                        json["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or("No recommendation available")
                            .to_string()
                    } else {
                        format!("Error: {}", res.status())
                    }
                }
                Err(e) => {
                    format!("Error: {}", e)
                }
            };

            set_recommendation.set(result.clone());
            set_loading.set(false);
            result
        }
    });

    view! {
        <div class="p-4 bg-slate-800/40 backdrop-blur rounded-xl border border-[#ff6b35]/20 mt-4">
            <h3 class="text-[#ff6b35] font-bold mb-3">"AI Task Recommendations"</h3>

            <Show
                when=move || !loading.get()
                fallback=move || {
                    view! { <div class="text-slate-400">"Loading recommendations..."</div> }
                }
            >
                <div class="text-slate-200 mb-3">{move || recommendation.get()}</div>
            </Show>

            <button
                class="px-3 py-1 bg-[#ff6b35]/20 hover:bg-[#ff6b35]/30 text-[#ff6b35] rounded-md transition-colors"
                on:click=move |_| {
                    recommendations.dispatch(());
                }
            >
                "Get Recommendations"
            </button>
        </div>
    }
}
