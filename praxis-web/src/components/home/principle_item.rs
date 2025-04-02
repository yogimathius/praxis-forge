use leptos::prelude::*;

#[component]
pub fn PrincipleItem(
    title: String,
    description: String,
    index: i32,
    hover_class: Memo<String>,
    set_hover_index: Action<i32, ()>,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                format!(
                    "{} rounded-xl border p-8 hover-lift transition-all duration-300",
                    hover_class.get(),
                )
            }
            on:mouseenter=move |_| {
                let _ = set_hover_index.dispatch(index);
            }
            on:mouseleave=move |_| {
                let _ = set_hover_index.dispatch(-1);
            }
        >
            <h3 class="text-3xl font-bold text-titanium mb-4">{title}</h3>
            <p class="text-ash text-lg">{description}</p>
        </div>
    }
}
