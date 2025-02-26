use std::rc::Rc;
use std::sync::Arc;

use crate::graphql::queries::tasks::Task;
use crate::services::service_context::ServiceContext;
use leptos::prelude::*;
use leptos::task::spawn_local;

type Refetch = Box<dyn Fn() + 'static>;

pub struct TasksState {
    pub tasks: ReadSignal<Vec<Task>>,
    pub loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
    pub create: Action<Task, Result<Task, String>>,
    pub update: Action<Task, Result<Task, String>>,
    pub delete: Action<cynic::Id, Result<(), String>>,
    pub refetch: Arc<dyn Fn()>,
}

pub fn use_tasks() -> TasksState {
    let service = use_context::<ServiceContext>().expect("No service context found");
    let (tasks, set_tasks) = signal(Vec::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let service_create = service.clone();
    let service_update = service.clone();
    let service_delete = service.clone();
    let service_refetch = service.clone();

    // Initial fetch
    Effect::new(move |_| {
        let service = service.clone();
        spawn_local(async move {
            set_loading.set(true);
            match service.0.fetch_tasks().await {
                Ok(data) => set_tasks.update(|tasks| *tasks = data),
                Err(e) => set_error.update(|err| *err = Some(e)),
            }
            set_loading.update(|l| *l = false);
        });
    });

    // Actions
    #[allow(warnings)]
    let create = create_action(move |task: &Task| {
        let task = task.clone();
        let service = service_create.clone();
        async move { service.0.create_task(task).await }
    });

    #[allow(warnings)]
    let update = create_action(move |task: &Task| {
        let task = task.clone();
        let service = service_update.clone();
        async move {
            let id = task.id.clone().unwrap();
            service.0.update_task(id, task).await
        }
    });

    #[allow(warnings)]
    let delete = create_action(move |id: &cynic::Id| {
        let id = id.clone();
        let service = service_delete.clone();
        async move { service.0.delete_task(id).await }
    });

    TasksState {
        tasks,
        loading,
        error,
        create,
        update,
        delete,
        refetch: Arc::new(move || {
            let service = service_refetch.clone();
            spawn_local(async move {
                set_loading.update(|l| *l = true);
                if let Ok(data) = service.0.fetch_tasks().await {
                    set_tasks.update(|tasks| *tasks = data);
                }
                set_loading.update(|l| *l = false);
            });
        }),
    }
}
