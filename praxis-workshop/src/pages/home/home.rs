use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (hover_index, set_hover_index) = signal(-1);

    let principles = vec![
        ("Praxis", "Theory into Action"),
        ("Forge", "Shape Your Future"),
        ("Intention", "Purposeful Progress"),
        ("Mastery", "Continuous Growth"),
    ];

    view! {
        <div class="max-w-7xl mx-auto p-4 md:p-8 animate-fade-in">
            <h2 class="text-4xl md:text-5xl font-bold text-center mb-6 bg-gradient-to-r from-[#ff6b35] to-[#ffd700] bg-clip-text text-transparent">
                "Welcome to Praxis Forge"
            </h2>

            <p class="text-xl text-center text-slate-300 mb-12 animate-slide-up">
                "Transform intentions into actions, actions into habits, habits into mastery."
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-12">
                {principles.into_iter().enumerate().map(|(i, (title, desc))| {
                    let i = i;
                    let hover_class = move || {
                        if hover_index.get() == i as i32 {
                            "bg-slate-900/90 border-[#ff6b35] shadow-lg shadow-[#ff6b35]/20"
                        } else {
                            "bg-slate-900/80 border-[#ff6b35]/30"
                        }
                    };

                    view! {
                        <div
                            class=move || format!("{} backdrop-blur-lg rounded-xl border p-6 md:p-8 hover:transform hover:-translate-y-1 hover:shadow-xl hover:shadow-[#ff6b35]/20 transition-all duration-300", hover_class())
                            on:mouseenter=move |_| set_hover_index.set(i as i32)
                            on:mouseleave=move |_| set_hover_index.set(-1)
                        >
                            <h3 class="text-2xl font-bold text-[#ff6b35] mb-3">{title}</h3>
                            <p class="text-slate-300">{desc}</p>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="flex justify-center">
                <a
                    href="/tasks"
                    class="bg-[#ff6b35] text-slate-900 font-semibold px-6 py-3 rounded-xl hover:bg-[#ff6b35]/90 hover:shadow-lg hover:shadow-[#ff6b35]/20 transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-[#ff6b35] focus:ring-offset-2 focus:ring-offset-slate-900"
                >
                    "Start Your Journey"
                </a>
            </div>
        </div>
    }
}
