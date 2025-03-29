use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};

pub fn celebrate_task_completion() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create 50 confetti particles
    for i in 0..50 {
        if let Ok(confetti) = document.create_element("div") {
            confetti.set_class_name("absolute w-2 h-2 rounded-full pointer-events-none");

            // Generate random color with hue based on index
            let hue = (i * 10) % 360;
            let color = format!("hsl({}, 100%, 50%)", hue);

            // Random position
            let left = js_sys::Math::random() * 100.0;

            // Apply styles
            if let Some(style) = confetti.dyn_ref::<HtmlElement>().map(|e| e.style()) {
                let _ = style.set_property("background-color", &color);
                let _ = style.set_property("left", &format!("{}%", left));
                let _ = style.set_property("top", "0");
                let _ = style.set_property("z-index", "100");

                // Random animation duration
                let fall_duration = 1.0 + js_sys::Math::random() * 2.0;
                let spin_duration = 0.5 + js_sys::Math::random() * 1.0;

                let anim = format!(
                    "fall {}s ease-in forwards, spin {}s linear infinite",
                    fall_duration, spin_duration
                );
                let _ = style.set_property("animation", &anim);
            }

            if let Ok(_) = body.append_child(&confetti) {
                // Clone for later removal
                let confetti_clone = confetti.clone();
                let body_clone = body.clone();

                // Remove after animation
                let window_clone = window.clone();
                let closure = wasm_bindgen::closure::Closure::once(move || {
                    let _ = body_clone.remove_child(&confetti_clone);
                });

                window_clone
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        (3000.0) as i32,
                    )
                    .unwrap();

                closure.forget();
            }
        }
    }
}

pub fn celebrate_at_position(x: f64, y: f64) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create 30 confetti particles
    for i in 0..30 {
        if let Ok(confetti) = document.create_element("div") {
            confetti.set_class_name("absolute w-2 h-2 rounded-full pointer-events-none");

            // Generate random color with hue based on index
            let hue = (i * 12) % 360;
            let color = format!("hsl({}, 100%, 50%)", hue);

            // Random offset from click position
            let offset_x = (js_sys::Math::random() - 0.5) * 100.0;
            let offset_y = (js_sys::Math::random() - 0.5) * 100.0;

            // Apply styles
            if let Some(style) = confetti.dyn_ref::<HtmlElement>().map(|e| e.style()) {
                let _ = style.set_property("background-color", &color);
                let _ = style.set_property("left", &format!("{}px", x + offset_x));
                let _ = style.set_property("top", &format!("{}px", y + offset_y));
                let _ = style.set_property("z-index", "100");

                // Random animation duration
                let fall_duration = 0.8 + js_sys::Math::random() * 1.5;
                let spin_duration = 0.5 + js_sys::Math::random() * 1.0;

                let anim = format!(
                    "fall {}s ease-in forwards, spin {}s linear infinite",
                    fall_duration, spin_duration
                );
                let _ = style.set_property("animation", &anim);
            }

            if let Ok(_) = body.append_child(&confetti) {
                // Clone for later removal
                let confetti_clone = confetti.clone();
                let body_clone = body.clone();

                // Remove after animation
                let window_clone = window.clone();
                let closure = wasm_bindgen::closure::Closure::once(move || {
                    let _ = body_clone.remove_child(&confetti_clone);
                });

                window_clone
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        (2000.0) as i32,
                    )
                    .unwrap();

                closure.forget();
            }
        }
    }
}
