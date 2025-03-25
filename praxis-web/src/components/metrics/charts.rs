use leptos::prelude::*;
use std::collections::VecDeque;

use super::performance_monitor::MemoryMetrics;

#[component]
pub fn FpsChart(fps_data: ReadSignal<VecDeque<f64>>) -> impl IntoView {
    view! {
        <div class="bg-glass p-4 rounded-lg">
            <h4 class="text-lg font-bold text-orange mb-2">FPS History</h4>
            <div class="h-32 bg-glass-dark rounded-md p-2 flex items-end">
                {move || {
                    fps_data
                        .get()
                        .iter()
                        .map(|fps| {
                            let height = (*fps / 60.0 * 100.0).min(100.0);
                            let color = if *fps < 30.0 {
                                "bg-red-500/60"
                            } else if *fps < 50.0 {
                                "bg-yellow-500/60"
                            } else {
                                "bg-green-500/60"
                            };

                            view! {
                                <div
                                    class=format!("w-2 mx-0.5 {}", color)
                                    style=format!("height: {}%", height)
                                ></div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[component]
pub fn MemoryChart(memory_metrics: ReadSignal<MemoryMetrics>) -> impl IntoView {
    view! {
        <div class="bg-glass p-4 rounded-lg mt-4">
            <h4 class="text-lg font-bold text-orange mb-2">Memory Usage</h4>
            <div class="h-32 bg-glass-dark rounded-md p-2 flex items-end">
                {move || {
                    let metrics = memory_metrics.get();
                    let max_mem = metrics
                        .js_heap_size
                        .into_iter()
                        .chain(metrics.wasm_memory.into_iter())
                        .fold(0.0, f64::max)
                        .max(1.0);
                    let mut bars = Vec::new();
                    if let Some(js_mem) = metrics.js_heap_size {
                        let js_height = (js_mem / max_mem * 100.0).min(100.0);
                        bars.push(
                            view! {
                                <div
                                    class="w-4 mx-1 bg-orange/60"
                                    style=format!("height: {}%", js_height)
                                    title=format!("JS Heap: {:.1}MB", js_mem)
                                ></div>
                            },
                        );
                    }
                    if let Some(wasm_mem) = metrics.wasm_memory {
                        let wasm_height = (wasm_mem / max_mem * 100.0).min(100.0);
                        bars.push(
                            view! {
                                <div
                                    class="w-4 mx-1 bg-blue-500/60"
                                    style=format!("height: {}%", wasm_height)
                                    title=format!("WASM: {:.1}MB", wasm_mem)
                                ></div>
                            },
                        );
                    }
                    bars
                }}
            </div>
        </div>
    }
}
