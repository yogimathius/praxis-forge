use leptos::prelude::*;
use std::rc::Rc;
use web_sys::Element;

use crate::components::task::item::TaskItem;
use crate::graphql::queries::tasks::Task;

#[component]
pub fn VirtualizedTasksList(
    tasks: Vec<Task>,
    #[prop(into)] on_toggle: Action<Task, Result<Task, String>>,
    #[prop(into)] on_delete: Action<cynic::Id, Result<(), String>>,
    #[prop(into)] on_edit: Action<Task, Result<Task, String>>,
    #[prop(default = 80)] item_height: usize,
    #[prop(default = 5)] buffer_size: usize,
) -> impl IntoView {
    let container_ref = NodeRef::<html::Div>::new();
    let (scroll_top, set_scroll_top) = create_signal(0);
    let (container_height, set_container_height) = create_signal(500);

    // Calculate total content height and visible window
    let total_height = move || tasks.len() * item_height;

    let visible_window = create_memo(move |_| {
        let scroll = scroll_top.get();
        let height = container_height.get();

        let start_index = scroll / item_height;
        let start_index = start_index.saturating_sub(buffer_size);

        let visible_count = (height / item_height) + (buffer_size * 2);
        let end_index = (start_index + visible_count).min(tasks.len());

        (start_index, end_index)
    });

    // Update container height on resize
    create_effect(move |_| {
        if let Some(container) = container_ref.get() {
            let height = container.client_height() as usize;
            set_container_height.set(height);
        }
    });

    // Handle scroll events
    let on_scroll = move |_| {
        if let Some(container) = container_ref.get() {
            set_scroll_top.set(container.scroll_top() as usize);
        }
    };

    view! {
        <div>
            <h3 class="text-2xl font-bold text-spark text-center mb-6 animate-ember-pulse">
                "Your Tasks"
            </h3>
            <Show
                when=move || !tasks.is_empty()
                fallback=|| {
                    view! {
                        <div class="border-2 border-dashed border-titanium-30 rounded-xl p-8 text-center">
                            <h3 class="text-xl font-bold text-spark mb-2">"No tasks yet"</h3>
                            <p class="text-ash opacity-80">
                                "Add your first task to get started on your journey."
                            </p>
                        </div>
                    }
                }
            >
                <div
                    class="space-y-4 overflow-y-auto"
                    style="height: 60vh"
                    node_ref=container_ref
                    on:scroll=on_scroll
                >
                    <div style=move || {
                        format!("height: {}px; position: relative;", total_height())
                    }>
                        {move || {
                            let (start, end) = visible_window.get();
                            tasks[start..end]
                                .iter()
                                .enumerate()
                                .map(|(i, task)| {
                                    let abs_index = start + i;

                                    view! {
                                        <div
                                            style=format!(
                                                "position: absolute; top: {}px; left: 0; right: 0;",
                                                abs_index * item_height,
                                            )
                                            data-id=format!(
                                                "task-{}",
                                                task.id.as_ref().map(|id| id.inner()).unwrap_or_default(),
                                            )
                                        >
                                            <TaskItem
                                                task=task.clone()
                                                on_toggle=on_toggle
                                                on_delete=on_delete
                                                on_edit=on_edit
                                            />
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}
