use std::{rc::Rc, sync::Arc};

use crate::graphql::queries::goals::Goal;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::services::service_context::ServiceContext;
pub struct GoalsState {
    pub goals: ReadSignal<Vec<Goal>>,
    pub loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
    pub create: Action<Goal, Result<Goal, String>>,
    pub update: Action<Goal, Result<Goal, String>>,
    pub delete: Action<cynic::Id, Result<(), String>>,
    pub refetch: Arc<dyn Fn()>,
}

pub fn use_goals() -> GoalsState {
    let service = use_context::<ServiceContext>().expect("No service context found");
    let (goals, set_goals) = signal(Vec::new());
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
            set_loading.update(|l| *l = true);
            match service.0.fetch_goals().await {
                Ok(data) => set_goals.update(|goals| *goals = data),
                Err(e) => set_error.update(|err| *err = Some(e)),
            }
            set_loading.update(|l| *l = false);
        });
    });

    // Actions
    #[allow(warnings)]
    let create = create_action(move |goal: &Goal| {
        let goal = goal.clone();
        let service = service_create.clone();
        async move { service.0.create_goal(goal).await }
    });

    #[allow(warnings)]
    let update = create_action(move |goal: &Goal| {
        let goal = goal.clone();
        let service = service_update.clone();
        async move {
            let id = goal.id.clone().unwrap();
            service.0.update_goal(id, goal).await
        }
    });

    #[allow(warnings)]
    let delete = create_action(move |id: &cynic::Id| {
        let id = id.clone();
        let service = service_delete.clone();
        async move { service.0.delete_goal(id).await }
    });

    GoalsState {
        goals,
        loading,
        error,
        create,
        update,
        delete,
        refetch: Arc::new(move || {
            let service = service_refetch.clone();
            spawn_local(async move {
                set_loading.update(|l| *l = true);
                if let Ok(data) = service.0.fetch_goals().await {
                    set_goals.update(|goals| *goals = data);
                }
                set_loading.update(|l| *l = false);
            });
        }),
    }
}
