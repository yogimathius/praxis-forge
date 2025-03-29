use js_sys::WebAssembly;
use leptos::prelude::*;
use leptos::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance, getter)]
    fn memory() -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn memory_usage() -> JsValue;
}

// Add a new struct to handle memory metrics
#[derive(Clone)]
pub struct MemoryMetrics {
    pub js_heap_size: Option<f64>,
    pub wasm_memory: Option<f64>,
}

impl MemoryMetrics {
    fn new() -> Self {
        Self {
            js_heap_size: None,
            wasm_memory: None,
        }
    }

    fn collect(&mut self) {
        // Try to get Chrome's performance.memory
        let window = web_sys::window().unwrap();
        let performance = window.performance().unwrap();

        // Safely try to get JS heap size
        if let Ok(memory_obj) = js_sys::Reflect::get(&performance, &JsValue::from_str("memory")) {
            if !memory_obj.is_undefined() && !memory_obj.is_null() {
                if let Ok(heap_size) =
                    js_sys::Reflect::get(&memory_obj, &JsValue::from_str("usedJSHeapSize"))
                {
                    self.js_heap_size = heap_size.as_f64().map(|size| size / (1024.0 * 1024.0));
                }
            }
        }

        // Get WASM memory usage
        let memory = wasm_bindgen::memory();
        if let Some(memory) = memory.dyn_ref::<WebAssembly::Memory>() {
            let bytes = memory
                .buffer()
                .dyn_into::<js_sys::ArrayBuffer>()
                .unwrap()
                .byte_length() as f64;
            self.wasm_memory = Some(bytes / (1024.0 * 1024.0));
        }
    }
}

// Performance metrics structure
#[derive(Clone)]
pub struct PerformanceMetrics {
    pub fps: RwSignal<VecDeque<f64>>,
    pub memory_metrics: RwSignal<MemoryMetrics>,
    pub render_times: RwSignal<VecDeque<f64>>,
    pub component_count: RwSignal<u32>,
    pub last_frame_time: RwSignal<f64>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            fps: RwSignal::new(VecDeque::with_capacity(60)),
            memory_metrics: RwSignal::new(MemoryMetrics::new()),
            render_times: RwSignal::new(VecDeque::with_capacity(60)),
            component_count: RwSignal::new(0),
            last_frame_time: RwSignal::new(0.0),
        }
    }

    // Start monitoring
    pub fn start_monitoring(&self) {
        // Sample every 1s
        set_interval(
            {
                let metrics = self.clone();
                move || metrics.sample()
            },
            std::time::Duration::from_millis(1000),
        );

        // Monitor FPS
        request_animation_frame({
            let metrics = self.clone();
            move || metrics.update_fps()
        });
    }

    // Sample metrics
    fn sample(&self) {
        // Update memory metrics
        self.memory_metrics.update(|metrics| {
            metrics.collect();
        });

        // Update component count
        let document = web_sys::window().unwrap().document().unwrap();
        if let Ok(components) = document.query_selector_all("[data-leptos-component]") {
            self.component_count.set(components.length());
        }
    }

    // Update FPS counter
    fn update_fps(&self) {
        let now = web_sys::window().unwrap().performance().unwrap().now();
        let last = self.last_frame_time.with_untracked(|last| *last);

        if last > 0.0 {
            let delta = now - last;
            let fps = 1000.0 / delta;

            self.fps.update(|fps_data| {
                fps_data.push_back(fps);
                if fps_data.len() > 60 {
                    fps_data.pop_front();
                }
            });
        }

        self.last_frame_time.set(now);

        // Request next frame
        request_animation_frame({
            let metrics = self.clone();
            move || metrics.update_fps()
        });
    }

    // Record render time for a component
    pub fn record_render_time(&self, time_ms: f64) {
        self.render_times.update(|times| {
            times.push_back(time_ms);
            if times.len() > 60 {
                times.pop_front();
            }
        });
    }
}

// Provider for metrics
pub fn provide_performance_metrics() {
    let metrics = PerformanceMetrics::new();
    metrics.start_monitoring();
    provide_context(metrics);
}

// Component to display metrics
#[component]
pub fn PerformanceMonitor() -> impl IntoView {
    let metrics = use_context::<PerformanceMetrics>();
    let (show_details, set_show_details) = create_signal(false);

    // Store metrics in a StoredValue to avoid ownership problems
    let stored_metrics = store_value(metrics);

    // Create memos for derived values to avoid closures that move values
    let metrics_exists = create_memo(move |_| stored_metrics.with_value(|m| m.is_some()));

    let avg_fps = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .map(|m| {
                    let fps = m.fps.get();
                    if fps.is_empty() {
                        0.0
                    } else {
                        fps.iter().sum::<f64>() / fps.len() as f64
                    }
                })
                .unwrap_or(0.0)
        })
    });

    let has_js_heap = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .map(|m| m.memory_metrics.get().js_heap_size.is_some())
                .unwrap_or(false)
        })
    });

    let js_heap_text = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .and_then(|m| m.memory_metrics.get().js_heap_size)
                .map(|v| format!("{:.1} MB", v))
                .unwrap_or_else(|| "N/A".to_string())
        })
    });

    let has_wasm_memory = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .map(|m| m.memory_metrics.get().wasm_memory.is_some())
                .unwrap_or(false)
        })
    });

    let wasm_memory_text = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .and_then(|m| m.memory_metrics.get().wasm_memory)
                .map(|v| format!("{:.1} MB", v))
                .unwrap_or_else(|| "N/A".to_string())
        })
    });

    let component_count = create_memo(move |_| {
        stored_metrics.with_value(|metrics| {
            metrics
                .as_ref()
                .map(|m| m.component_count.get())
                .unwrap_or(0)
        })
    });

    let button_text = move || {
        if show_details.get() {
            "Hide Details"
        } else {
            "Show Details"
        }
    };

    let container_class = "fixed bottom-4 left-4 bg-slate-800/70 backdrop-blur rounded-xl border border-[#ff6b35]/20 p-4 shadow-lg z-20";
    let header_class = "flex justify-between items-center mb-4";
    let title_class = "text-lg font-bold text-[#ff6b35]";
    let button_class =
        "px-2 py-1 bg-[#ff6b35]/20 hover:bg-[#ff6b35]/30 text-[#ff6b35] rounded text-sm";

    view! {
        <Show
            when=move || metrics_exists.get()
            fallback=move || {
                view! {
                    <div class=container_class>
                        <div class="text-slate-400">"Performance monitoring disabled"</div>
                    </div>
                }
            }
        >
            <div class=container_class>
                <div class=header_class>
                    <div class=title_class>"Performance Monitor"</div>
                    <button
                        class=button_class
                        on:click=move |_| set_show_details.update(|v| *v = !*v)
                    >
                        {button_text}
                    </button>
                </div>

                <Show when=move || show_details.get()>
                    <div class="grid grid-cols-2 gap-2 text-sm">
                        <div class="text-slate-400">"FPS:"</div>
                        <div class="text-slate-200">{move || format!("{:.1}", avg_fps.get())}</div>

                        <Show when=move || has_js_heap.get()>
                            <div class="text-slate-400">"JS Heap:"</div>
                            <div class="text-slate-200">{move || js_heap_text.get()}</div>
                        </Show>

                        <Show when=move || has_wasm_memory.get()>
                            <div class="text-slate-400">"WASM Memory:"</div>
                            <div class="text-slate-200">{move || wasm_memory_text.get()}</div>
                        </Show>

                        <div class="text-slate-400">"Components:"</div>
                        <div class="text-slate-200">{move || component_count.get()}</div>
                    </div>
                </Show>
            </div>
        </Show>
    }
}
