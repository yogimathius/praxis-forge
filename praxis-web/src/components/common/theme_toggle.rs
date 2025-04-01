use leptos::prelude::*;

// Theme preference stored in state
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Dark,
    Light,
}

// Global theme context
pub fn provide_theme() {
    let (theme, set_theme) = signal(Theme::Dark);
    provide_context(ThemeContext { theme, set_theme });

    // Initialize theme from localStorage if available
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(saved_theme)) = storage.get_item("praxis-theme") {
                    if saved_theme == "light" {
                        set_theme.set(Theme::Light);
                    }
                }
            }
        }
    });

    // Apply theme changes to the DOM
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    match theme.get() {
                        Theme::Dark => {
                            html.class_list().remove_1("light").unwrap_or_default();
                            html.class_list().add_1("dark").unwrap_or_default();

                            // Save preference
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("praxis-theme", "dark");
                            }
                        }
                        Theme::Light => {
                            html.class_list().remove_1("dark").unwrap_or_default();
                            html.class_list().add_1("light").unwrap_or_default();

                            // Save preference
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("praxis-theme", "light");
                            }
                        }
                    }
                }
            }
        }
    });
}

#[derive(Clone)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_context = use_context::<ThemeContext>().expect("ThemeContext not provided");

    let toggle_theme = move |_| {
        theme_context.set_theme.update(|theme| match theme {
            Theme::Dark => *theme = Theme::Light,
            Theme::Light => *theme = Theme::Dark,
        });
    };

    // Compute classes based on the theme
    let toggle_position = move || {
        if theme_context.theme.get() == Theme::Light {
            "translate-x-6"
        } else {
            ""
        }
    };

    let toggle_bg = move || {
        if theme_context.theme.get() == Theme::Dark {
            "bg-[#00b2ca]"
        } else {
            "bg-yellow-400"
        }
    };

    view! {
        <button
            class="relative p-2 rounded-full bg-slate-800/50 border border-[#00b2ca]/20 transition-all hover:shadow-[0_0_10px_rgba(255,107,53,0.2)]"
            on:click=toggle_theme
        >
            <div class="w-12 h-6 rounded-full bg-slate-700/50 relative">
                <div class=move || {
                    format!(
                        "absolute top-0.5 w-5 h-5 rounded-full transition-all duration-300 flex items-center justify-center overflow-hidden {} {}",
                        toggle_position(),
                        toggle_bg(),
                    )
                }>
                    {move || match theme_context.theme.get() {
                        Theme::Dark => view! { <span class="text-xs">"🌙"</span> },
                        Theme::Light => view! { <span class="text-xs">"☀️"</span> },
                    }}
                </div>
            </div>
        </button>
    }
}
