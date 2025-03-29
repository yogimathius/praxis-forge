use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn SmartSuggestions() -> impl IntoView {
    let (suggestions, set_suggestions) = signal(Vec::<String>::new());
    let (is_loading, set_is_loading) = signal(true);

    // Simulate AI suggestions - replace with your actual API call
    Effect::new(move |_| {
        set_timeout(
            move || {
                set_suggestions.set(vec![
                    "Break down your goal into smaller tasks".to_string(),
                    "Schedule a focused work session tomorrow".to_string(),
                    "Connect this task to your 'Learn Rust' goal".to_string(),
                    "Consider adding a deadline to make progress measurable".to_string(),
                ]);
                set_is_loading.set(false);
            },
            Duration::from_millis(1500),
        );
    });

    view! {
        <div class="p-5 bg-slate-800/40 backdrop-blur rounded-xl border border-[#ff6b35]/20 shadow-[0_0_15px_rgba(255,107,53,0.1)] mt-6">
            <h3 class="text-[#ff6b35] font-bold mb-3 flex items-center gap-2">
                <span class="text-lg">"✨"</span>
                <span>"AI Suggestions"</span>
            </h3>

            <div class="relative">
                <Show
                    when=move || !is_loading.get()
                    fallback=move || {
                        view! {
                            <div class="flex items-center justify-center p-4">
                                <div class="animate-pulse-subtle text-slate-300">
                                    "Analyzing your tasks and goals..."
                                </div>
                            </div>
                        }
                    }
                >
                    <ul class="space-y-2">
                        {move || {
                            suggestions
                                .get()
                                .into_iter()
                                .map(|suggestion| {
                                    view! {
                                        <li class="flex items-start gap-2 p-2 rounded-lg hover:bg-[#ff6b35]/10 transition-colors cursor-pointer border border-transparent hover:border-[#ff6b35]/20">
                                            <span class="text-[#ff6b35] text-lg pt-0.5">"↗"</span>
                                            <span class="text-slate-200">{suggestion}</span>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </ul>
                </Show>
            </div>

            <div class="mt-4 flex justify-end">
                <button class="text-sm text-slate-400 hover:text-[#ff6b35] transition-colors flex items-center gap-1">
                    <span>"Refresh suggestions"</span>
                    <span class="text-lg">"↻"</span>
                </button>
            </div>
        </div>
    }
}

// Function to apply a suggestion to a task or goal
// You'll implement this later based on your data model
fn _apply_suggestion(suggestion: &str) {
    // Implementation will depend on your app's state management
    println!("Applying suggestion: {}", suggestion);
}
