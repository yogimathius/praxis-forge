use leptos::prelude::*;
use std::sync::Arc;

use crate::graphql::queries::tasks::Task;
use crate::services::service_context::ServiceContext;

/// Type alias for refetch functions
pub type RefetchFn = Arc<dyn Fn() + Send + Sync>;

/// Manages task-related queries using Leptos Resources
pub struct TaskQueries;

impl TaskQueries {
    /// Creates a resource that fetches all tasks
    pub fn create_resource(
        service: ServiceContext,
    ) -> Resource<(), Result<Vec<Task>, ServerFnError>> {
        let (trigger, set_trigger) = signal(());

        let refetch: RefetchFn = Arc::new(move || set_trigger.update(|_| ()));
        provide_context(refetch);

        Resource::<(), Result<Vec<Task>, ServerFnError>>::new(
            move || trigger.get(),
            move |_| {
                let service = service.clone();
                async move {
                    service
                        .0
                        .fetch_tasks()
                        .await
                        .map_err(|e| ServerFnError::ServerError(e.to_string()))
                }
            },
        )
    }

    /// Gets the refetch function from context
    pub fn get_refetch() -> Arc<dyn Fn() + Send + Sync> {
        use_context::<Arc<dyn Fn() + Send + Sync>>().expect("No refetch function in context")
    }

    /// Initializes task queries and returns the tasks resource
    /// This is a convenience method that creates the resource and taskss the initial fetch
    pub fn init(service: ServiceContext) -> Resource<(), Result<Vec<Task>, ServerFnError>> {
        let resource = Resource::new(service);

        // tasks initial fetch
        let refetch = Self::get_refetch();
        refetch();

        resource
    }
}

/// Provides a hook-like API for components that need task data
pub fn use_tasks(
    service: ServiceContext,
) -> (
    Resource<(), Result<Vec<Task>, ServerFnError>>,
    Arc<dyn Fn() + Send + Sync>,
) {
    let resource = Resource::new(service);
    let refetch = TaskQueries::get_refetch();

    // tasks initial fetch
    refetch();

    (resource, refetch)
}
