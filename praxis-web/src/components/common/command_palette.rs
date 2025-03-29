use leptos::html::Input;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use wasm_bindgen::prelude::*;

// Component for a single command item
#[component]
fn CommandItem(
    name: String,
    path: String,
    is_selected: bool,
    idx: usize,
    set_selected: WriteSignal<usize>,
    set_visibility: WriteSignal<bool>,
) -> impl IntoView {
    let bg_class = if is_selected { "bg-[#ff6b35]/15" } else { "" };
    let navigate = use_navigate();

    view! {
        <div
            class=format!("p-3 cursor-pointer flex items-center transition-colors {}", bg_class)
            on:click=move |_| {
                execute_command(&path, &navigate);
                set_visibility.set(false);
            }
            on:mouseenter=move |_| {
                set_selected.set(idx);
            }
        >
            <div class="mr-3 text-[#ff6b35]">"⟶"</div>
            <div class="text-slate-200">{name}</div>
        </div>
    }
}

#[component]
pub fn CommandPalette() -> impl IntoView {
    let (show, set_show) = signal(false);
    let (search_term, set_search_term) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    let input_ref = NodeRef::<Input>::new();
    let navigate = use_navigate();

    // Define the commands as a resource that won't be moved
    let commands = vec![
        ("Create new task", "tasks/new"),
        ("View all tasks", "tasks"),
        ("View all goals", "goals"),
        ("Go to home", "/"),
    ];
    let commands = StoredValue::new(commands);

    // Filter commands based on search term
    let filtered_commands = Memo::new(move |_| {
        let term = search_term.get().to_lowercase();
        commands.with_value(|cmds| {
            cmds.iter()
                .filter(|(cmd, _)| cmd.to_lowercase().contains(&term))
                .cloned()
                .collect::<Vec<_>>()
        })
    });

    // Set up keyboard shortcut to open command palette
    let filtered_commands_for_effect = filtered_commands.clone();
    let navigate_for_effect = navigate.clone();

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Clone navigate_for_effect before the closure
        let nav_for_closure = navigate_for_effect.clone();

        let handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.ctrl_key() && event.key() == "k" {
                event.prevent_default();
                set_show.update(|s| *s = !*s);
                set_search_term.set(String::new());

                // Focus the input if opening
                if show.get() {
                    if let Some(input) = input_ref.get() {
                        let _ = input.focus();
                    }
                }
            }

            // Close on escape
            if event.key() == "Escape" && show.get() {
                set_show.set(false);
            }

            // Navigation with arrow keys
            if show.get() {
                if event.key() == "ArrowDown" {
                    event.prevent_default();
                    set_selected_index.update(|i| {
                        let commands_len = filtered_commands_for_effect.get().len();
                        if commands_len > 0 {
                            *i = (*i + 1) % commands_len;
                        } else {
                            *i = 0;
                        }
                    });
                } else if event.key() == "ArrowUp" {
                    event.prevent_default();
                    set_selected_index.update(|i| {
                        let commands_len = filtered_commands_for_effect.get().len();
                        if commands_len > 0 {
                            if *i == 0 {
                                *i = commands_len - 1;
                            } else {
                                *i = *i - 1;
                            }
                        } else {
                            *i = 0;
                        }
                    });
                } else if event.key() == "Enter" {
                    let commands = filtered_commands_for_effect.get();
                    if let Some((_, path)) = commands.get(selected_index.get()) {
                        nav_for_closure(path, Default::default());
                        set_show.set(false);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        document
            .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
            .unwrap();

        handler.forget();
    });

    view! {
        <div
            class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]"
            class:hidden=move || !show.get()
            style="background-color: rgba(0, 0, 0, 0.5); backdrop-filter: blur(4px);"
            on:click=move |_| set_show.set(false)
        >
            <div
                class="w-1/2 max-w-xl bg-slate-800/70 backdrop-blur-lg rounded-xl shadow-[0_0_15px_rgba(255,107,53,0.2)] border border-[#ff6b35]/20 overflow-hidden"
                on:click=move |e| e.stop_propagation()
            >
                <div class="p-4 border-b border-slate-700/50">
                    <div class="flex items-center">
                        <div class="text-[#ff6b35] mr-2">"⌘"</div>
                        <input
                            type="text"
                            placeholder="Type a command or search..."
                            class="w-full bg-transparent border-none focus:outline-none text-slate-200 placeholder:text-slate-400"
                            node_ref=input_ref
                            on:input=move |e| {
                                let input = event_target_value(&e);
                                set_search_term.set(input);
                                set_selected_index.set(0);
                            }
                            prop:value=move || search_term.get()
                        />
                    </div>
                </div>

                <div class="max-h-80 overflow-y-auto">
                    <Show
                        when=move || !filtered_commands.get().is_empty()
                        fallback=move || {
                            view! {
                                <div class="p-4 text-slate-400 text-center">
                                    "No commands found. Try a different search term."
                                </div>
                            }
                        }
                    >
                        {move || {
                            let filtered = filtered_commands.get();
                            let selected = selected_index.get();
                            filtered
                                .iter()
                                .enumerate()
                                .map(|(idx, item)| {
                                    view! {
                                        <CommandItem
                                            name=item.0.to_string()
                                            path=item.1.to_string()
                                            is_selected=idx == selected
                                            idx=idx
                                            set_selected=set_selected_index
                                            set_visibility=set_show
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </Show>
                </div>

                <div class="p-2 border-t border-slate-700/50 text-xs text-slate-400 flex justify-between">
                    <div>"Press <span class='text-[#ff6b35]'>↑↓</span> to navigate"</div>
                    <div>"Press <span class='text-[#ff6b35]'>Enter</span> to select"</div>
                    <div>"Press <span class='text-[#ff6b35]'>Esc</span> to close"</div>
                </div>
            </div>
        </div>
    }
}

// Execute command based on path
fn execute_command(path: &str, navigate: &impl Fn(&str, leptos_router::NavigateOptions)) {
    navigate(path, Default::default());
}
