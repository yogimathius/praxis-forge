use leptos::prelude::*;

#[component]
pub fn Dropdown(
    #[prop(into)] options: Signal<Vec<(String, String)>>,
    #[prop(into)] selected: Signal<String>,
    on_change: impl Fn(String) -> () + 'static,
    placeholder: String,
) -> impl IntoView {
    view! {
        <div class="relative flex gap-2">
            <select
                class="bg-white/10 border border-orange-30 rounded-lg p-4 text-steel font-medium focus:border-orange focus:shadow-orange-sm pr-10 appearance-none w-full"
                on:change=move |ev| on_change(event_target_value(&ev))
                prop:value=move || selected.get()
            >
                <option value="">{placeholder}</option>
                {move || {
                    options
                        .get()
                        .into_iter()
                        .map(|(value, label)| {
                            view! {
                                <option
                                    value=value
                                    class="py-2 px-4 hover:bg-orange-100 text-gray-800"
                                >
                                    {label}
                                </option>
                            }
                        })
                        .collect_view()
                }}
            </select>
            <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                <div class="text-orange text-lg">"▼"</div>
            </div>
        </div>
    }
}
