use leptos::prelude::*;
use thaw::{Grid, GridItem};

#[component]
pub fn Principles() -> impl IntoView {
    let (hover_index, set_hover_index) = signal(-1);

    let principles = vec![
        ("Praxis", "Theory into Action"),
        ("Forge", "Shape Your Future"),
        ("Intention", "Purposeful Progress"),
        ("Mastery", "Continuous Growth"),
    ];
    view! {
        <Grid
            cols=Signal::derive(move || {
                let width = window().inner_width().unwrap().as_f64().unwrap_or(0.0);
                if width > 768.0 { 2 } else { 1 }
            })
            x_gap=Signal::derive(|| 6)
            y_gap=Signal::derive(|| 6)
            class="w-full"
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
    }
}
