use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    #[prop(optional)] goals: Option<Vec<crate::graphql::queries::goals::Goal>>,
) -> impl IntoView {
    let goals = goals.unwrap_or_default();

    view! {
        <div>
            {goals
                .iter()
                .map(|goal| {
                    let tasks_completed = goal.tasks_completed.unwrap_or(0);
                    let tasks_required = goal.tasks_required.unwrap_or(1);
                    let progress_percentage = if tasks_required > 0 {
                        (tasks_completed as f32 / tasks_required as f32 * 100.0).round() as i32
                    } else {
                        0
                    };

                    view! {
                        <div class="flex flex-col gap-3 mb-4">
                            <div class="flex justify-between items-center">
                                <span class="text-ash font-medium">Progress</span>
                                <span class="text-ash text-sm bg-glass px-2 py-1 rounded-md border border-spark-30">
                                    {tasks_completed} "/" {tasks_required} " tasks"
                                </span>
                            </div>

                            // Progress bar using the provided CSS classes
                            <div class="progress-bar">
                                <div
                                    class="progress-fill"
                                    style=format!("width: {}%;", progress_percentage)
                                ></div>
                                <div class="progress-text">{progress_percentage}"% Complete"</div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
