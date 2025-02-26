use crate::graphql::queries::{goals::Goal, tasks::Task};
use leptos::prelude::*;
use thaw::*;

// XP values for different actions
const TASK_COMPLETION_XP: i32 = 20;
const GOAL_COMPLETION_XP: i32 = 100;
const XP_THRESHOLD: i32 = 1000; // Adjust based on your game design

#[component]
pub fn ProgressBar(
    #[prop(into)] tasks: Signal<Vec<Task>>,
    #[prop(into)] goals: Signal<Vec<Goal>>,
) -> impl IntoView {
    let (level, set_level) = signal(1);
    let (xp, set_xp) = signal(0);

    // Create an effect to track completed tasks and goals
    Effect::new(move |_| {
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

    // Calculate the progress as a value between 0 and 1
    let progress = move || xp.get() as f64 / XP_THRESHOLD as f64;

    view! {
        <div class="progress-container">
            <h3>"Level " {move || level.get()}</h3>
            <thaw::ProgressBar
                value=Signal::derive(progress)
                color=Signal::derive(move || ProgressBarColor::Brand)
            />
            <p>"XP: " {move || xp.get()} " / " {XP_THRESHOLD}</p>
        </div>
    }
}
