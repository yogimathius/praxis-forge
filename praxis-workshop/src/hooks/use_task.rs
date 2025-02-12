use leptos::*;

use crate::graphql::queries::tasks::Task;
use crate::services::service_context::use_service;

pub fn use_task(
    initial_task: Task,
) -> (
    ReadSignal<Task>,
    WriteSignal<Task>,
    ReadSignal<bool>,
    WriteSignal<bool>,
    impl Fn() -> (), // delete handler
) {
    let service = use_service();
    let (task, set_task) = create_signal(initial_task);
    let (is_editing, set_is_editing) = create_signal(false);

    let handle_delete = {
        let service = service.0.clone(); // Clone the Arc before the move
        move || {
            let task_id = task.get().id.unwrap();
            let service = service.clone(); // Clone again for the spawn_local
            spawn_local(async move {
                let _ = service.delete_task(task_id).await;
            });
        }
    };

    (task, set_task, is_editing, set_is_editing, handle_delete)
}
