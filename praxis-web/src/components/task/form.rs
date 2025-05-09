use std::rc::Rc;

use leptos::{ev, prelude::*, task::spawn_local};

use crate::components::common::Dropdown;
use crate::graphql::queries::{goals::Goal, tasks::Task};

#[component]
pub fn TaskForm(
    create: Action<Task, Result<Task, String>>,
    refetch: Rc<dyn Fn()>,
    goals: ReadSignal<Vec<Goal>>,
) -> impl IntoView {
    let (task_text, set_task_text) = signal(String::new());
    let (task_description, set_task_description) = signal(String::new());
    let (selected_goal, set_selected_goal) = signal(None::<i32>);
    let (task_status, set_task_status) = signal("pending".to_string());
    let (is_submitting, set_is_submitting) = signal(false);
    let (show_success, set_show_success) = signal(false);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let current_text = task_text.get_untracked();
        let current_description = task_description.get_untracked();
        let current_goal = selected_goal.get_untracked();

        if current_text.is_empty() {
            return;
        }

        set_is_submitting.set(true);

        let task = Task {
            id: None,
            title: Some(current_text),
            description: Some(current_description),
            completed: Some(false),
            status: Some(task_status.get_untracked()),
            goal: current_goal.map(|id| Goal {
                id: Some(cynic::Id::new(id.to_string())),
                title: None,
                description: None,
                tasks_required: None,
                tasks_completed: None,
                tasks: None,
            }),
        };

        let refetch = refetch.clone();
        spawn_local(async move {
            let _ = create.dispatch(task);
            set_timeout(
                move || {
                    refetch();
                },
                std::time::Duration::from_millis(100),
            );
        });

        set_task_text.set(String::new());
        set_task_description.set(String::new());
        set_task_status.set("pending".to_string());
        set_show_success.set(true);
        set_is_submitting.set(false);

        set_timeout(
            move || {
                set_show_success.set(false);
            },
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <form
            class="bg-glass-dark rounded-xl p-8 shadow-titanium-sm
            dark:bg-glass-dark
            light:bg-white/80"
            on:submit=on_submit
        >
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-6">
                    <div class="flex flex-col gap-2">
                        <label
                            for="task-title"
                            class="text-sm font-medium dark:text-ash light:text-steel"
                        >
                            Task Title
                        </label>
                        <input
                            id="task-title"
                            type="text"
                            class="bg-white/10 border border-titanium-30 rounded-lg p-4 font-medium focus:border-orange focus:shadow-titanium-sm
                            dark:bg-glass-dark dark:text-ash
                            light:bg-white/90 light:text-steel"
                            placeholder="Add a new task..."
                            on:input=move |ev| set_task_text.set(event_target_value(&ev))
                            prop:value=move || task_text.get()
                        />
                    </div>

                    <div class="flex flex-col gap-2">
                        <label
                            for="task-description"
                            class="text-sm font-medium dark:text-ash light:text-steel"
                        >
                            Description
                        </label>
                        <textarea
                            id="task-description"
                            class="bg-white/10 border border-titanium-30 rounded-lg p-4 font-medium focus:border-orange focus:shadow-titanium-sm min-h-[80px] resize-y
                            dark:bg-glass-dark dark:text-ash
                            light:bg-white/90 light:text-steel"
                            placeholder="Add a description..."
                            on:input=move |ev| set_task_description.set(event_target_value(&ev))
                            prop:value=move || task_description.get()
                        ></textarea>
                    </div>
                </div>

                <div class="flex flex-col md:flex-row gap-6">
                    <div class="flex flex-col gap-2 flex-1">
                        <label
                            for="task-goal"
                            class="text-sm font-medium dark:text-ash light:text-steel"
                        >
                            Related Goal
                        </label>
                        <Dropdown
                            options=Memo::new(move |_| {
                                goals
                                    .get()
                                    .into_iter()
                                    .filter_map(|goal| {
                                        let id = goal.id.as_ref()?.inner().to_string();
                                        let title = goal.title.clone().unwrap_or_default();
                                        Some((id, title))
                                    })
                                    .collect::<Vec<_>>()
                            })
                            selected=Memo::new(move |_| {
                                selected_goal.get().map(|id| id.to_string()).unwrap_or_default()
                            })
                            on_change=move |value| set_selected_goal.set(value.parse::<i32>().ok())
                            placeholder="Select a goal".to_string()
                        />
                    </div>

                    <div class="flex flex-col gap-2 md:w-1/3">
                        <label
                            for="task-status"
                            class="text-sm font-medium dark:text-ash light:text-steel"
                        >
                            Status
                        </label>
                        <Dropdown
                            options=Memo::new(move |_| {
                                vec![
                                    ("pending".to_string(), "Pending".to_string()),
                                    ("in_progress".to_string(), "In Progress".to_string()),
                                    ("completed".to_string(), "Completed".to_string()),
                                ]
                            })
                            selected=Memo::new(move |_| { task_status.get() })
                            on_change=move |value| set_task_status.set(value)
                            placeholder="Select a status".to_string()
                        />
                    </div>
                </div>

                <div class="flex justify-start">
                    <button
                        type="submit"
                        class="btn btn-orange hover-lift shadow-titanium-sm focus:outline-none"
                        disabled=move || is_submitting.get()
                    >
                        {move || if is_submitting.get() { "Adding..." } else { "Add Task" }}
                    </button>
                </div>
            </div>

            {move || {
                show_success
                    .get()
                    .then(|| {
                        view! {
                            <div class="bg-green-20 border border-green-500 rounded-xl p-4 text-center animate-fade-in mt-8">
                                "Task added successfully!"
                            </div>
                        }
                    })
            }}
        </form>
    }
}
