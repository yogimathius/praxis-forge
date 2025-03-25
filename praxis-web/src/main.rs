use leptos::prelude::*;
use praxis_shop_ui::{
    components::{
        animation::task_transition::provide_element_tracker,
        debug::signal_visualizer::{provide_signal_registry, SignalVisualizer},
        metrics::performance_monitor::provide_performance_metrics,
        PerformanceMonitor,
    },
    App,
};

fn main() {
    console_error_panic_hook::set_once();

    // Set up debug tools
    provide_signal_registry();
    provide_performance_metrics();
    provide_element_tracker();

    mount_to_body(|| {
        view! {
            <App />

            // Add debug tools - only in development mode if desired
            <SignalVisualizer />
            <PerformanceMonitor />
        }
    })
}
