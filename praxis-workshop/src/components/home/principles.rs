use crate::components::home::PrincipleItem;
use leptos::prelude::*;
use thaw::{Grid, GridItem};

#[component]
pub fn Principles() -> impl IntoView {
    let (hover_index, set_hover_index) = signal(-1);

    // Create an action to update the hover index
    let update_hover_index = Action::new_unsync(move |index: &i32| {
        set_hover_index.set(*index);
        async {} // Return an empty async block to satisfy the Future requirement
    });

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
                    let i_clone = i as i32;
                    let hover_class = create_memo(move |_| {
                        if hover_index.get() == i as i32 {
                            "bg-glass border-orange shadow-orange-md".to_string()
                        } else {
                            "bg-glass border-orange-30".to_string()
                        }
                    });

                    view! {
                        <PrincipleItem
                            title=title.to_string()
                            description=desc.to_string()
                            index=i_clone
                            hover_class=hover_class
                            set_hover_index=update_hover_index
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </Grid>
    }
}
