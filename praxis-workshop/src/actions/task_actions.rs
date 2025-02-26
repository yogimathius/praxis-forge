use crate::graphql::queries::tasks::Task;
use crate::services::service_context::ServiceContext;
use leptos::prelude::*;

#[allow(warnings)]
pub fn create_task_action(service: ServiceContext) -> Action<Task, Result<Task, String>> {
    create_action(move |task: &Task| {
        let task = task.clone();
        let service = service.clone();
        async move { service.0.create_task(task).await }
    })
}

#[allow(warnings)]
pub fn update_task_action(service: ServiceContext) -> Action<Task, Result<Task, String>> {
    create_action(move |task: &Task| {
        let task = task.clone();
        let service = service.clone();
        async move {
            let id = task.id.clone().unwrap();
            service.0.update_task(id, task).await
        }
    })
}

#[allow(warnings)]
pub fn delete_task_action(service: ServiceContext) -> Action<cynic::Id, Result<(), String>> {
    create_action(move |id: &cynic::Id| {
        let id = id.clone();
        let service = service.clone();
        async move { service.0.delete_task(id).await }
    })
} 