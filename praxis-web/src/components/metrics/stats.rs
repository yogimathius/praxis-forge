use crate::components::metrics::performance_monitor::MemoryMetrics;
use leptos::prelude::*;

#[component]
pub fn StatsDisplay(
    avg_fps: ReadSignal<f64>,
    memory_metrics: ReadSignal<MemoryMetrics>,
    component_count: ReadSignal<u32>,
) -> impl IntoView {
    view! {
        <div class="flex justify-between mb-4">
            <div class="text-ash">
                {"FPS: "}
                <span class="text-orange font-bold">{move || format!("{:.1}", avg_fps.get())}</span>
            </div>
            <div class="text-ash">
                {"Memory: "}
                <span class="text-orange font-bold">
                    {move || {
                        let metrics = memory_metrics.get();
                        let js_mem = metrics
                            .js_heap_size
                            .map(|m| format!("JS: {:.1}MB", m))
                            .unwrap_or_default();
                        let wasm_mem = metrics
                            .wasm_memory
                            .map(|m| format!("WASM: {:.1}MB", m))
                            .unwrap_or_default();
                        format!("{} {}", js_mem, wasm_mem)
                    }}
                </span>
            </div>
            <div class="text-ash">
                {"Components: "}
                <span class="text-orange font-bold">{move || component_count.get()}</span>
            </div>
        </div>
    }
}
