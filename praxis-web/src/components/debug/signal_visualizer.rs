use leptos::prelude::*;
use std::collections::HashMap;

// Signal Tracker structure
#[derive(Clone, PartialEq)]
pub struct SignalMetrics {
    updates: RwSignal<u32>,
    dependents: RwSignal<Vec<String>>,
    value_history: RwSignal<Vec<String>>,
    last_update_time: RwSignal<f64>,
    creation_time: f64,
}

// Global registry for signals
#[derive(Clone)]
pub struct SignalRegistry(pub RwSignal<HashMap<String, SignalMetrics>>);

// Create a singleton registry
pub fn provide_signal_registry() {
    let registry = SignalRegistry(RwSignal::new(HashMap::new()));
    provide_context(registry);
}

// Custom wrapper function to create a tracked signal
pub fn create_tracked_signal<T: Clone + std::fmt::Debug + 'static>(
    name: &str,
    initial: T,
) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Send + Sync + 'static,
{
    let (read, write) = signal(initial.clone());

    // Get the registry
    if let Some(registry) = use_context::<SignalRegistry>() {
        let now = get_current_time();

        // Create metrics for this signal
        let metrics = SignalMetrics {
            updates: RwSignal::new(0),
            dependents: RwSignal::new(Vec::new()),
            value_history: RwSignal::new(vec![format!("{:?}", initial)]),
            last_update_time: RwSignal::new(now),
            creation_time: now,
        };

        // Store in registry
        registry.0.update(|reg| {
            reg.insert(name.to_string(), metrics);
        });

        // Create a new signal that will be our tracked signal
        let (tracked_read, tracked_write) = signal(initial);
        let name_clone = name.to_string();
        let registry_clone = registry.clone();

        // Create effect to update original signal when tracked signal changes
        Effect::new(move |_| {
            let val = tracked_read.get();

            // Record update in registry
            registry_clone.0.update(|reg| {
                if let Some(metrics) = reg.get_mut(&name_clone) {
                    let now = get_current_time();
                    metrics.updates.update(|count| *count += 1);
                    metrics.last_update_time.set(now);
                    metrics.value_history.update(|history| {
                        history.push(format!("{:?}", val.clone()));
                        if history.len() > 10 {
                            history.remove(0);
                        }
                    });
                }
            });

            // Forward the update to the original signal
            write.set(val);
        });

        (read, tracked_write)
    } else {
        (read, write)
    }
}

// Helper function to get current time
fn get_current_time() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

// Component to visualize signals
#[component]
pub fn SignalVisualizer() -> impl IntoView {
    let registry = use_context::<SignalRegistry>();
    let signals = create_read_slice(
        registry
            .map(|r| r.0)
            .unwrap_or_else(|| RwSignal::new(HashMap::new())),
        |map| map.clone(),
    );

    // Sort by update count
    let sorted_signals = Memo::new(move |_| {
        let signals = signals.get();
        let mut signal_vec: Vec<(String, SignalMetrics)> = signals
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        signal_vec.sort_by(|a, b| b.1.updates.get().cmp(&a.1.updates.get()));
        signal_vec
    });

    view! {
        <div class="fixed bottom-4 right-4 w-1/3 h-2/3 bg-glass-dark rounded-xl border border-spark-30 p-4 overflow-y-auto shadow-spark-md z-20 transition-all duration-500">
            <h3 class="text-2xl font-bold text-spark mb-4">Signal Debugger</h3>
            <div class="space-y-4">
                {move || {
                    sorted_signals
                        .get()
                        .iter()
                        .map(|(name, metrics)| {
                            let updates = metrics.updates.get();
                            let history = metrics.value_history.get();
                            let last_update = metrics.last_update_time.get();
                            let current_time = get_current_time();
                            let age = current_time - metrics.creation_time;
                            let name_str = name.to_string();

                            view! {
                                <div class="bg-glass p-3 rounded-lg border border-spark-30">
                                    <div class="flex justify-between items-center mb-2">
                                        <h4 class="text-orange font-bold">{name_str}</h4>
                                        <span class="text-ash text-sm">{updates} " updates"</span>
                                    </div>
                                    <div class="text-ash text-sm mb-2">
                                        {"Age: "}{(age / 1000.0).round() as i32}
                                        {" sec | Last update: "}
                                        {((current_time - last_update) / 1000.0).round() as i32}
                                        {" sec ago"}
                                    </div>
                                    <div class="text-xs space-y-1 bg-glass-dark p-2 rounded-md max-h-24 overflow-y-auto">
                                        {history
                                            .into_iter()
                                            .map(|val| {
                                                view! { <div class="text-steel">{val}</div> }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
