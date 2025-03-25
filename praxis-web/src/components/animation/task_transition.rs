use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDocument};

// Store a simplified position record instead of DomRect
#[derive(Clone, Debug)]
struct ElementPosition {
    left: f64,
    top: f64,
    right: f64,
    bottom: f64,
}

// Record element positions for FLIP animations
pub struct ElementPositionTracker {
    positions: RwSignal<HashMap<String, ElementPosition>>,
}

impl ElementPositionTracker {
    pub fn new() -> Self {
        Self {
            positions: RwSignal::new(HashMap::new()),
        }
    }

    // Record element positions before update
    pub fn snapshot(&self, selector: &str) {
        let window = web_sys::window().unwrap();
        let document = window
            .document()
            .unwrap()
            .dyn_into::<HtmlDocument>()
            .unwrap();
        if let Ok(elements) = document.query_selector_all(selector) {
            self.positions.update(|positions| {
                positions.clear();

                for i in 0..elements.length() {
                    if let Some(element) = elements.item(i) {
                        if let Ok(element) = element.dyn_into::<Element>() {
                            let id = element.get_attribute("data-id").unwrap_or_default();
                            let rect = element.get_bounding_client_rect();

                            // Convert DomRect to our safe ElementPosition
                            let position = ElementPosition {
                                left: rect.left(),
                                top: rect.top(),
                                right: rect.right(),
                                bottom: rect.bottom(),
                            };

                            positions.insert(id, position);
                        }
                    }
                }
            });
        }
    }

    // Apply FLIP animation
    pub fn animate(&self, selector: &str) {
        let window = web_sys::window().unwrap();
        let document = window
            .document()
            .unwrap()
            .dyn_into::<HtmlDocument>()
            .unwrap();
        if let Ok(elements) = document.query_selector_all(selector) {
            let prev_positions = self.positions.get_untracked();

            for i in 0..elements.length() {
                if let Some(element) = elements.item(i) {
                    if let Ok(element) = element.dyn_into::<Element>() {
                        let id = element.get_attribute("data-id").unwrap_or_default();

                        // If we have a previous position
                        if let Some(prev_pos) = prev_positions.get(&id) {
                            let curr_rect = element.get_bounding_client_rect();

                            // Calculate the delta
                            let delta_x = prev_pos.left - curr_rect.left();
                            let delta_y = prev_pos.top - curr_rect.top();

                            if delta_x.abs() > 0.5 || delta_y.abs() > 0.5 {
                                // Call the animate method
                                let animate_fn = js_sys::Reflect::get(
                                    &element,
                                    &wasm_bindgen::JsValue::from_str("animate"),
                                )
                                .unwrap();
                                let animate_fn = animate_fn.dyn_into::<js_sys::Function>().unwrap();

                                // Create keyframes
                                let keyframes = js_sys::Array::new();
                                let keyframe1 = js_sys::Object::new();
                                js_sys::Reflect::set(
                                    &keyframe1,
                                    &wasm_bindgen::JsValue::from_str("transform"),
                                    &wasm_bindgen::JsValue::from_str(&format!(
                                        "translate({}px, {}px)",
                                        delta_x, delta_y
                                    )),
                                )
                                .unwrap();
                                let keyframe2 = js_sys::Object::new();
                                js_sys::Reflect::set(
                                    &keyframe2,
                                    &wasm_bindgen::JsValue::from_str("transform"),
                                    &wasm_bindgen::JsValue::from_str("translate(0, 0)"),
                                )
                                .unwrap();
                                keyframes.push(&keyframe1);
                                keyframes.push(&keyframe2);

                                // Create options
                                let options = js_sys::Object::new();
                                js_sys::Reflect::set(
                                    &options,
                                    &wasm_bindgen::JsValue::from_str("duration"),
                                    &wasm_bindgen::JsValue::from_f64(300.0),
                                )
                                .unwrap();
                                js_sys::Reflect::set(
                                    &options,
                                    &wasm_bindgen::JsValue::from_str("easing"),
                                    &wasm_bindgen::JsValue::from_str(
                                        "cubic-bezier(0.2, 0, 0.3, 1)",
                                    ),
                                )
                                .unwrap();

                                // Call animate
                                let _ = js_sys::Reflect::apply(
                                    &animate_fn,
                                    &element,
                                    &js_sys::Array::of2(&keyframes, &options),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

// Create a provider for the tracker
pub fn provide_element_tracker() {
    let tracker = ElementPositionTracker::new();
    provide_context(tracker);
}
