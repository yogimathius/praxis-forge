use leptos::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[component]
pub fn TaskRecommendations(cx: Scope, task: Task) -> impl IntoView {
    let (recommendation, set_recommendation) = create_signal(cx, String::new());
    let (loading, set_loading) = create_signal(cx, false);

    let get_recommendation = create_action(cx, move |_| {
        set_loading.set(true);

        let task_data = format!(
            "Task: {}\nDescription: {}\nStatus: {}",
            task.title, task.description, task.status
        );

        async move {
            // Get API key from local storage
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
                return;
            }

            // Create request
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

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        let json: serde_json::Value = res.json().await.unwrap_or_default();
                        let content = json["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or("No recommendation available")
                            .to_string();
                        set_recommendation.set(content);
                    } else {
                        set_recommendation.set(format!("Error: {}", res.status()));
                    }
                }
                Err(e) => {
                    set_recommendation.set(format!("Error: {}", e));
                }
            }

            set_loading.set(false);
        }
    });

    // View code here...
}
