use leptos::*;
use wasm_bindgen_test::*;

use crate::api::tasks::Task;
use crate::components::task::item::TaskItem;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_task_item_renders() {
    let task = Task {
        id: 1,
        title: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
        status: "pending".to_string(),
        completed: false,
        goal_id: None,
    };

    let (goals, _) = create_signal(vec![]);
    let _on_toggle = Callback::new(|_| {});
    let _on_delete = Callback::new(|_| {});
    let _on_edit = Callback::new(|_| {});

    mount_to_body(move || {
        view! {
            <TaskItem
                task=task
                goals=goals
                on_toggle=_on_toggle
                on_delete=_on_delete
                on_edit=_on_edit
            />
        }
    });

    let title_element = document()
        .query_selector(".taskTitle")
        .expect("Should find title element")
        .expect("Element should exist");
    assert!(title_element.inner_html().contains("Test Task"));
}
