use leptos::prelude::*;
use thaw::{Grid, GridItem};

#[component]
pub fn Home() -> impl IntoView {
    let (hover_index, set_hover_index) = signal(-1);

    let principles = vec![
        ("Praxis", "Theory into Action"),
        ("Forge", "Shape Your Future"),
        ("Intention", "Purposeful Progress"),
        ("Mastery", "Continuous Growth"),
    ];

    view! {
        <div class="max-w-7xl mx-auto p-4 md:p-8">
            <h2 class="text-4xl font-bold text-spark flex justify-center mb-8 animate-ember-pulse">
                "Welcome to Praxis Forge"
            </h2>

            <p class="text-xl flex justify-center text-ash mb-12 animate-slide-up">
                "Transform intentions into actions, actions into habits, habits into mastery."
            </p>
            <Grid
                cols=Signal::derive(move || {
                    let width = window().inner_width().unwrap().as_f64().unwrap_or(0.0);
                    if width > 768.0 { 2 } else { 1 }
                })
                x_gap=Signal::derive(|| 6)
                y_gap=Signal::derive(|| 6)
                class="mb-12"
            >

                {principles
                    .into_iter()
                    .enumerate()
                    .map(|(i, (title, desc))| {
                        let i = i;
                        let hover_class = move || {
                            if hover_index.get() == i as i32 {
                                "bg-glass border-orange shadow-orange-md"
                            } else {
                                "bg-glass border-orange-30"
                            }
                        };

                        view! {
                            <GridItem column=Signal::derive(|| 1) offset=Signal::derive(|| 0)>
                                <div
                                    class=move || {
                                        format!(
                                            "{} rounded-xl border p-8 hover-lift transition-all duration-300",
                                            hover_class(),
                                        )
                                    }
                                    on:mouseenter=move |_| set_hover_index.set(i as i32)
                                    on:mouseleave=move |_| set_hover_index.set(-1)
                                >
                                    <h3 class="text-3xl font-bold text-orange mb-4">{title}</h3>
                                    <p class="text-ash text-lg">{desc}</p>
                                </div>
                            </GridItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </Grid>

            <div class="flex justify-center">
                <a
                    href="/tasks"
                    class="btn btn-orange hover-lift shadow-orange-sm focus:outline-none"
                >
                    "Start Your Journey"
                </a>
            </div>
        </div>
    }
}
