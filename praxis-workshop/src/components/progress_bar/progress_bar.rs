use crate::graphql::queries::{goals::Goal, tasks::Task};
use leptonic::prelude::*;
use leptos::*;

// XP values for different actions
const TASK_COMPLETION_XP: i32 = 20;
const GOAL_COMPLETION_XP: i32 = 100;
const XP_THRESHOLD: i32 = 1000; // Adjust based on your game design

#[component]
pub fn ProgressBar(
    #[prop(into)] tasks: Signal<Vec<Task>>,
    #[prop(into)] goals: Signal<Vec<Goal>>,
) -> impl IntoView {
    let (level, set_level) = create_signal(1);
    let (xp, set_xp) = create_signal(0);

    // Create an effect to track completed tasks and goals
    create_effect(move |_| {
        let completed_tasks = tasks
            .get()
            .iter()
            .filter(|task| task.status.as_ref() == Some(&"completed".to_string()))
            .count() as i32;

        let completed_goals = goals
            .get()
            .iter()
            .filter(|goal| goal.tasks_completed.unwrap_or(0) == goal.tasks_required.unwrap_or(0))
            .count() as i32;

        let total_xp =
            (completed_tasks * TASK_COMPLETION_XP) + (completed_goals * GOAL_COMPLETION_XP);

        // Calculate level and remaining XP
        let new_level = (total_xp / XP_THRESHOLD) + 1;
        let remaining_xp = total_xp % XP_THRESHOLD;

        set_level.set(new_level);
        set_xp.set(remaining_xp);
    });

    view! {
        <div class="progress-container">
            <h3>"Level " {move || level.get()}</h3>
            <Progress>
                <ProgressBar
                    value=move || xp.get() as f64
                    max=XP_THRESHOLD as f64
                    color=ProgressBarColor::Primary
                    striped=true
                    animated=true
                />
            </Progress>
            <p>"XP: " {move || xp.get()} " / " {XP_THRESHOLD}</p>
        </div>
    }
}
