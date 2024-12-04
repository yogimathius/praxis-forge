use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn TaskForm<F>(on_add: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let input_ref = create_node_ref::<html::Input>();

    let onsubmit = move |e: SubmitEvent| {
        e.prevent_default();
        if let Some(input) = input_ref.get() {
            let value = input.value();
            on_add(value);
            input.set_value("");
        }
    };

    view! {
        <form on:submit=onsubmit style="margin-bottom: 1.5rem;">
            <div style="display: flex; gap: 1rem;">
                <input
                    type="text"
                    placeholder="Add a new task..."
                    node_ref=input_ref
                    style="flex: 1; padding: 0.5rem 1rem; border: 1px solid #e2e8f0; border-radius: 0.25rem; &:focus { outline: none; box-shadow: 0 0 0 2px #3b82f6; }"
                />
                <button
                    type="submit"
                    style="background-color: #3b82f6; color: white; padding: 0.5rem 1rem; border-radius: 0.25rem; border: none; cursor: pointer; &:hover { background-color: #2563eb; }"
                >
                    "Add Task"
                </button>
            </div>
        </form>
    }
}
