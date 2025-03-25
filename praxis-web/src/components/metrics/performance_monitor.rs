use js_sys::WebAssembly;
use leptos::prelude::*;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::metrics::charts::{FpsChart, MemoryChart};
use crate::components::metrics::stats::StatsDisplay;

// External JS functions
#[wasm_bindgen]
extern "C" {
    // Define memory as a getter from performance.memory
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
    let (show_details, set_show_details) = signal(false);

    let metrics_clone = metrics.clone();
    // Average values
    let avg_fps = Memo::new(move |_| {
        if let Some(metrics) = metrics_clone.as_ref() {
            let fps = metrics.fps.get();
            if fps.is_empty() {
                0.0
            } else {
                fps.iter().sum::<f64>() / fps.len() as f64
            }
        } else {
            0.0
        }
    });

    let button_text = move || {
        if show_details.get() {
            "Hide Details"
        } else {
            "Show Details"
        }
    };

    let container_class = "fixed bottom-4 left-4 bg-glass-dark rounded-xl border border-orange-30 p-4 shadow-orange-md z-20";
    let header_class = "flex justify-between items-center mb-4";
    let title_class = "text-2xl font-bold text-orange";
    let button_class = "btn btn-orange-sm";

    view! {
        <Show
            when=move || metrics.is_some()
            fallback=move || {
                view! {
                    <div class=container_class>
                        <div class="text-ash">"Performance monitoring disabled"</div>
                    </div>
                }
            }
        >
            <div class=container_class>
                <div class="text-ash">{"Performance Monitor"}</div>
            </div>
        </Show>
    }
}
