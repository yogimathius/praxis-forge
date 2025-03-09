use tailwind_css::TailwindBuilder;

pub fn get_tailwind_bundle() -> String {
    let mut tw = TailwindBuilder::default();

    // Layout & Containers
    let _ = tw.trace("max-w-7xl mx-auto p-4 md:p-8", false);
    let _ = tw.trace("grid grid-cols-1 md:grid-cols-2 gap-6", false);
    let _ = tw.trace(
        "flex flex-col md:flex-row items-center justify-between gap-4 md:gap-8",
        false,
    );
    let _ = tw.trace("flex items-center justify-center space-x-4", false);

    // Cards & Containers
    let _ = tw.trace(
        "bg-slate-900/80 backdrop-blur-lg rounded-xl border border-[#ff6b35]/30",
        false,
    );
    let _ = tw.trace("shadow-lg shadow-[#ff6b35]/10", false);
    let _ = tw.trace("hover:transform hover:-translate-y-1 hover:shadow-xl hover:shadow-[#ff6b35]/20 transition-all duration-300", false);
    let _ = tw.trace("p-6 md:p-8", false);

    // Typography
    let _ = tw.trace("text-slate-100 text-opacity-90", false);
    let _ = tw.trace("text-4xl md:text-5xl font-bold bg-gradient-to-r from-[#ff6b35] to-[#ffd700] bg-clip-text text-transparent", false);
    let _ = tw.trace("text-xl text-[#ff6b35]", false);
    let _ = tw.trace("text-base text-slate-300", false);

    // Buttons & Interactive Elements
    let _ = tw.trace(
        "bg-[#ff6b35] text-slate-900 font-semibold px-6 py-3 rounded-xl",
        false,
    );
    let _ = tw.trace("hover:bg-[#ff6b35]/90 hover:shadow-lg hover:shadow-[#ff6b35]/20 transition-all duration-300", false);
    let _ = tw.trace("disabled:opacity-50 disabled:cursor-not-allowed", false);
    let _ = tw.trace("focus:outline-none focus:ring-2 focus:ring-[#ff6b35] focus:ring-offset-2 focus:ring-offset-slate-900", false);

    // Forms & Inputs
    let _ = tw.trace(
        "bg-slate-800/50 border-2 border-[#ff6b35]/30 rounded-xl p-4",
        false,
    );
    let _ = tw.trace(
        "focus:border-[#ff6b35] focus:ring-2 focus:ring-[#ff6b35]/50 focus:outline-none",
        false,
    );
    let _ = tw.trace("placeholder:text-slate-400", false);

    // Task & Goal Items
    let _ = tw.trace("flex items-center justify-between gap-4 p-4 bg-slate-800/30 rounded-xl border border-[#ff6b35]/20", false);
    let _ = tw.trace("hover:border-[#ff6b35]/40 hover:bg-slate-800/40", false);

    // Status Indicators
    let _ = tw.trace("px-3 py-1 rounded-full text-sm font-medium", false);
    let _ = tw.trace(
        "bg-yellow-500/20 text-yellow-300 border border-yellow-500/30",
        false,
    ); // In Progress
    let _ = tw.trace(
        "bg-green-500/20 text-green-300 border border-green-500/30",
        false,
    ); // Completed
    let _ = tw.trace(
        "bg-[#ff6b35]/20 text-[#ff6b35] border border-[#ff6b35]/30",
        false,
    ); // Pending

    // Progress Bars
    let _ = tw.trace("h-2 bg-slate-700/50 rounded-full overflow-hidden", false);
    let _ = tw.trace("bg-gradient-to-r from-[#ff6b35] to-[#ffd700] h-full rounded-full transition-all duration-500", false);

    // Animations
    let _ = tw.trace("animate-fade-in transition-opacity duration-500", false);
    let _ = tw.trace("animate-slide-up transition-transform duration-500", false);
    let _ = tw.trace("animate-pulse-subtle", false);

    // Lists & Grids
    let _ = tw.trace(
        "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
        false,
    );
    let _ = tw.trace("flex flex-col gap-4", false);

    // Success Messages
    let _ = tw.trace(
        "bg-green-500/10 border border-green-500/30 text-green-300 rounded-xl p-4",
        false,
    );

    // Error Messages
    let _ = tw.trace(
        "bg-red-500/10 border border-red-500/30 text-red-300 rounded-xl p-4",
        false,
    );

    tw.bundle().expect("Failed to bundle Tailwind styles")
}
