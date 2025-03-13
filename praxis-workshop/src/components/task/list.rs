use leptos::prelude::*;

use crate::components::task::item::TaskItem;
use crate::graphql::queries::goals::Goal;
use crate::graphql::queries::tasks::Task;

#[component]
pub fn TasksList(
    tasks: Vec<Task>,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
) -> impl IntoView {
    let tasks_clone = tasks.clone();

    view! {
        <div>
            <h3 class="text-2xl font-bold text-spark text-center mb-6 animate-ember-pulse">
                "Your Tasks"
            </h3>
            <Show
                when=move || !tasks_clone.is_empty()
                fallback=|| {
                    view! {
                        <div class="border-2 border-dashed border-orange-30 rounded-xl p-8 text-center">
                            <h3 class="text-xl font-bold text-spark mb-2">"No tasks yet"</h3>
                            <p class="text-ash opacity-80">
                                "Add your first task to get started on your journey."
                            </p>
                        </div>
                    }
                }
            >
                <div class="space-y-4">
                    {tasks
                        .iter()
                        .map(|task| {
                            view! {
                                <TaskItem
                                    task=task.clone()
                                    on_toggle=on_toggle
                                    on_delete=on_delete
                                    on_edit=on_edit
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </Show>
        </div>
    }
}
