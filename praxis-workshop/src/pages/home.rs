use crate::components::progress_bar::progress_bar::Progressbar;
use crate::state::use_goals::{use_goals, GoalsState};
use crate::state::use_tasks::{use_tasks, TasksState};
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen(module = "/src/pages/home.module.css")]
extern "C" {}

#[component]
pub fn HomePage() -> impl IntoView {
    let (hover_index, set_hover_index) = create_signal(-1);
    let tasks = use_tasks(); // Your existing hook
    let goals = use_goals();
    let principles = vec![
        ("Praxis", "Theory into Action"),
        ("Forge", "Shape Your Future"),
        ("Intention", "Purposeful Progress"),
        ("Mastery", "Continuous Growth"),
    ];

    view! {
        <div class="home-container">
            <h2 class="home-title fade-in">"Welcome to Praxis Forge"</h2>

            <p class="subtitle slide-in">
                "Transform intentions into actions, actions into habits, habits into mastery."
            </p>
            <Progressbar tasks=tasks.tasks goals=goals.goals />
            <div class="principles-grid">
                {principles.into_iter().enumerate().map(|(i, (title, desc))| {
                    let i = i;
                    view! {
                        <div
                            class="principle-card"
                            class:active=move || hover_index.get() == i as i32
                            on:mouseenter=move |_| set_hover_index.set(i as i32)
                            on:mouseleave=move |_| set_hover_index.set(-1)
                        >
                            <h3 class="principle-title">{title}</h3>
                            <p class="principle-desc">{desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="cta-section">
                <a href="/tasks" class="cta-button">
                    "Start Your Journey"
                </a>
            </div>
        </div>
    }
}
