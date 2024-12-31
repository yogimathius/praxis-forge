use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/components/task/form.module.css")]
extern "C" {}

#[component]
pub fn TaskForm<F>(on_add: F) -> impl IntoView
where
    F: Fn(String, String) + 'static,
{
    let (task_text, set_task_text) = create_signal(String::new());
    let (task_description, set_task_description) = create_signal(String::new());
    let (task_status, set_task_status) = create_signal("pending".to_string());
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (show_success, set_show_success) = create_signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = task_text.get_untracked();
        let current_description = task_description.get_untracked();
        if current_text.is_empty() {
            return;
        }

        set_is_submitting.set(true);

        on_add(current_text, current_description);

        set_task_text.set(String::new());
        set_task_description.set(String::new());
        set_task_status.set("pending".to_string());
        set_show_success.set(true);
        set_is_submitting.set(false);

        set_timeout(
            move || {
                set_show_success.set(false);
            },
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <form class="form" on:submit=on_submit>
            <div class="formGroup">
                <input
                    type="text"
                    class="input"
                    placeholder="Add a new task..."
                    on:input=move |ev| set_task_text.set(event_target_value(&ev))
                    prop:value=move || task_text.get()
                />
                <input
                    type="text"
                    class="input"
                    placeholder="Add a description..."
                    on:input=move |ev| set_task_description.set(event_target_value(&ev))
                    prop:value=move || task_description.get()
                />
                <select
                    class="select"
                    on:input=move |ev| set_task_status.set(event_target_value(&ev))
                    prop:value=move || task_status.get()
                >
                    <option value="pending">"Pending"</option>
                    <option value="in_progress">"In Progress"</option>
                    <option value="completed">"Completed"</option>
                </select>
                <button
                    type="submit"
                    class="button"
                    disabled=move || is_submitting.get()
                >
                    {move || if is_submitting.get() { "Adding..." } else { "Add Task" }}
                </button>
            </div>

            {move || show_success.get().then(|| view! {
                <div class="successMessage">
                    "Task added successfully!"
                </div>
            })}
        </form>
    }
}
