// use leptos::prelude::*;
// use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

// use crate::api::tasks::Task;
// use crate::state::tasks::use_tasks;

// #[cfg(test)]
// mod tests {
//     use crate::api::tasks::Task;
//     use crate::state::tasks::use_tasks;
//     use leptos::prelude::*;
//     use wasm_bindgen_test::*;

//     #[wasm_bindgen_test]
//     async fn test_tasks_state() {
//         let runtime = create_runtime();
//         let (tasks, create, _, _, _) = use_tasks();

//         // Test initial state
//         assert_eq!(tasks.get().len(), 0);

//         // Test task creation
//         create.dispatch(Task {
//             id: 0,
//             title: "New Task".to_string(),
//             description: None,
//             status: "pending".to_string(),
//             completed: false,
//             goal_id: None,
//         });

//         // Wait for the action to complete
//         create.value();

//         // Verify task was added
//         assert_eq!(tasks.get().len(), 1);
//         runtime.dispose();
//     }
// }
