#[component]
pub fn GoalsListPage() -> impl IntoView {
    let client = GraphQLClient::new("http://localhost:4000/api/graphql".to_string());
    let ws_url = "ws://localhost:4000/api/graphql/websocket";

    let tasks = create_resource(
        || (),
        move |_| async move {
            let query = TasksQuery::build(());
            client.query(query).await.ok()
        },
    );

    // Set up subscription
    create_effect(async move |_| {
        if let Ok(mut stream) = subscribe_to_task_updates(ws_url.to_string()).await {
            spawn_local(async move {
                while let Some(Ok(updated_task)) = stream.next().await {
                    // Handle real-time task updates
                    log::info!("Task updated: {:?}", updated_task);
                    // Update your UI state here
                }
            });
        }
    });

    view! {
        <div>
            <TaskForm/>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                // ... rest of your component
            </Suspense>
        </div>
    }
}
