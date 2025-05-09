use crate::components::home::PrincipleItem;
use leptos::prelude::*;

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

    let grid_cols = Signal::derive(move || {
        let width = window().inner_width().unwrap().as_f64().unwrap_or(0.0);
        if width > 768.0 {
            "grid-cols-2"
        } else {
            "grid-cols-1"
        }
    });

    view! {
        <div class=move || {
            format!("grid gap-6 w-full {}", grid_cols.get())
        }>
            {principles
                .into_iter()
                .enumerate()
                .map(|(i, (title, desc))| {
                    let i_clone = i as i32;
                    let hover_class = Memo::new(move |_| {
                        if hover_index.get() == i as i32 {
                            "bg-glass border-orange shadow-titanium-md".to_string()
                        } else {
                            "bg-glass border-titanium-30".to_string()
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
        </div>
    }
}
