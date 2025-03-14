use leptos::prelude::*;

#[derive(Clone)]
pub struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Clone)]
pub struct Goal {
    id: usize,
    title: String,
    tasks_required: usize,
    tasks_completed: usize,
}

#[component]
pub fn ProgressBar(
    #[prop(optional)] goals: Option<Vec<crate::graphql::queries::goals::Goal>>,
) -> impl IntoView {
    // Use provided goals or empty vec if None
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
                        <div class="flex flex-col gap-3">
                            <div class="flex justify-between items-center">
                                <span class="text-ash font-medium">Progress</span>
                                <span class="text-ash text-sm">
                                    {tasks_completed} "/" {tasks_required} " tasks"
                                </span>
                            </div>

                            <div class="bg-glass h-8 rounded-full overflow-hidden border border-spark-30">
                                <div
                                    class="bg-gradient-steel h-full rounded-full transition-all duration-500"
                                    style=format!("width: {}%;", progress_percentage)
                                ></div>
                                <div class="relative -top-8 text-center text-ash font-medium">
                                    {progress_percentage}"% Complete"
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
