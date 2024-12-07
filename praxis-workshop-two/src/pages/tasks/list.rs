use leptos::*;

use crate::components::task::form::TaskForm;

#[component]
pub fn TasksListPage() -> impl IntoView {
    let on_add = move |text: String| {
        // TODO: Handle task creation here
    };

    view! {
        <>
            <h1>"Tasks List"</h1>
            <TaskForm on_add=on_add />
        </>
    }
}
