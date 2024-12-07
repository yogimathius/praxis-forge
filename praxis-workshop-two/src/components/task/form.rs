use leptos::*;

#[component]
pub fn TaskForm<F>(on_add: F) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let (task_text, set_task_text) = create_signal(String::new());

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = task_text.get_untracked();
        if current_text.is_empty() {
            return;
        }

        on_add(current_text);

        // Clear the input
        set_task_text.set(String::new());
    };

    let on_input = move |ev| {
        set_task_text.set(event_target_value(&ev));
    };

    view! {
        <form class="list.module.css:form" on:submit=on_submit>
            <div class="list.module.css:container">
                <input
                    type="text"
                    class="list.module.css:input"
                    placeholder="Add a new task..."
                    on:input=on_input
                    prop:value=move || task_text.get()
                />
                <button
                    type="submit"
                    class="list.module.css:button"
                >
                    "Add Task"
                </button>
            </div>
        </form>
    }
}
