use tailwind_css::TailwindBuilder;
use wasm_bindgen_test::console_log;

pub fn get_tailwind_bundle() -> String {
    let mut tw = TailwindBuilder::default();

    // Layout & Containers - These should work well
    let _ = tw.trace("max-w-7xl mx-auto p-4 md:p-8", false);
    let _ = tw.trace("grid grid-cols-1 md:grid-cols-2 gap-6", false);
    let _ = tw.trace(
        "flex flex-col md:flex-row items-center justify-between gap-4 md:gap-8",
        false,
    );
    let _ = tw.trace("flex items-center justify-center space-x-4", false);

    // Cards & Containers - These might have issues with complex values
    let _ = tw.trace(
        "bg-slate-900/80 backdrop-blur-lg rounded-xl border border-[#ff6b35]/30", // ⚠️ Arbitrary color values with opacity might not work well
        false,
    );
    let _ = tw.trace("shadow-lg shadow-[#ff6b35]/10", false); // ⚠️ Custom shadow colors might be problematic
    let _ = tw.trace("hover:transform hover:-translate-y-1 hover:shadow-xl hover:shadow-[#ff6b35]/20 transition-all duration-300", false); // ⚠️ Complex hover combinations might not work
    let _ = tw.trace("p-6 md:p-8", false);

    // Typography - These should mostly work
    let _ = tw.trace("text-slate-100 text-opacity-90", false);
    let _ = tw.trace("text-4xl md:text-5xl font-bold bg-gradient-to-r from-[#ff6b35] to-[#ffd700] bg-clip-text text-transparent", false); // ⚠️ Complex gradients with arbitrary colors might not work
    let _ = tw.trace("text-xl text-[#ff6b35]", false); // ⚠️ Arbitrary color values
    let _ = tw.trace("text-base text-slate-300", false);

    // Buttons & Interactive Elements - Hover states might be problematic
    let _ = tw.trace(
        "bg-[#ff6b35] text-slate-900 font-semibold px-6 py-3 rounded-xl", // ⚠️ Arbitrary color values
        false,
    );
    let _ = tw.trace("hover:bg-[#ff6b35]/90 hover:shadow-lg hover:shadow-[#ff6b35]/20 transition-all duration-300", false); // ⚠️ Complex hover effects
    let _ = tw.trace("disabled:opacity-50 disabled:cursor-not-allowed", false); // ⚠️ Variant modifiers might not work fully
    let _ = tw.trace("focus:outline-none focus:ring-2 focus:ring-[#ff6b35] focus:ring-offset-2 focus:ring-offset-slate-900", false); // ⚠️ Complex focus states

    // Forms & Inputs - These should mostly work
    let _ = tw.trace(
        "bg-slate-800/50 border-2 border-[#ff6b35]/30 rounded-xl p-4", // ⚠️ Opacity modifiers
        false,
    );
    let _ = tw.trace(
        "focus:border-[#ff6b35] focus:ring-2 focus:ring-[#ff6b35]/50 focus:outline-none", // ⚠️ Focus states with arbitrary values
        false,
    );
    let _ = tw.trace("placeholder:text-slate-400", false); // ⚠️ Variant modifiers

    // Task & Goal Items - These should mostly work
    let _ = tw.trace("flex items-center justify-between gap-4 p-4 bg-slate-800/30 rounded-xl border border-[#ff6b35]/20", false); // ⚠️ Opacity modifiers
    let _ = tw.trace("hover:border-[#ff6b35]/40 hover:bg-slate-800/40", false); // ⚠️ Hover with opacity

    // Status Indicators - These should mostly work
    let _ = tw.trace("px-3 py-1 rounded-full text-sm font-medium", false);
    let _ = tw.trace(
        "bg-yellow-500/20 text-yellow-300 border border-yellow-500/30", // ⚠️ Opacity modifiers
        false,
    );
    let _ = tw.trace(
        "bg-green-500/20 text-green-300 border border-green-500/30", // ⚠️ Opacity modifiers
        false,
    );
    let _ = tw.trace(
        "bg-[#ff6b35]/20 text-[#ff6b35] border border-[#ff6b35]/30", // ⚠️ Arbitrary colors with opacity
        false,
    );

    // Progress Bars - These should mostly work
    let _ = tw.trace("h-2 bg-slate-700/50 rounded-full overflow-hidden", false); // ⚠️ Opacity modifiers
    let _ = tw.trace("bg-gradient-to-r from-[#ff6b35] to-[#ffd700] h-full rounded-full transition-all duration-500", false); // ⚠️ Gradients with arbitrary colors

    // Animations - These might not be fully implemented
    let _ = tw.trace("animate-fade-in transition-opacity duration-500", false); // ⚠️ Custom animations might not work
    let _ = tw.trace("animate-slide-up transition-transform duration-500", false); // ⚠️ Custom animations might not work
    let _ = tw.trace("animate-pulse-subtle", false); // ⚠️ Custom animations might not work

    // Lists & Grids - These should work well
    let _ = tw.trace(
        "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
        false,
    );
    let _ = tw.trace("flex flex-col gap-4", false);

    // Success Messages - These should mostly work
    let _ = tw.trace(
        "bg-green-500/10 border border-green-500/30 text-green-300 rounded-xl p-4", // ⚠️ Opacity modifiers
        false,
    );

    // Error Messages - These should mostly work
    let _ = tw.trace(
        "bg-red-500/10 border border-red-500/30 text-red-300 rounded-xl p-4", // ⚠️ Opacity modifiers
        false,
    );

    // Add these specific classes from your components
    let _ = tw.trace("text-2xl font-bold text-[#ff6b35] mb-3", false); // ⚠️ Arbitrary color values

    // Navigation component classes
    let _ = tw.trace("flex justify-center gap-8 p-4 mb-8 bg-white/5 backdrop-blur border border-[#ff6b35] rounded-xl", false); // ⚠️ Opacity modifiers and arbitrary colors

    // These simple utility classes should work well
    let _ = tw.trace("rounded-md", false);
    let _ = tw.trace("transition-all", false);
    let _ = tw.trace("duration-300", false);
    let _ = tw.trace("cursor-pointer", false);

    // These complex classes might have issues
    let _ = tw.trace("hover:bg-[#ff6b35]/10", false); // ⚠️ Hover with arbitrary color and opacity
    let _ = tw.trace("border", false);
    let _ = tw.trace("border-[#ff6b35]", false); // ⚠️ Arbitrary color values
    let _ = tw.trace("hover:border-[#ff6b35]/10", false); // ⚠️ Hover with arbitrary color and opacity
    let _ = tw.trace("hover:shadow-[0_0_15px_rgba(255,107,53,0.15)]", false); // ⚠️ Very complex arbitrary shadow
    let _ = tw.trace("active:bg-[#ff6b35]/15", false); // ⚠️ Active state with arbitrary color and opacity
    let _ = tw.trace("active:shadow-[0_0_15px_rgba(255,107,53,0.2)]", false); // ⚠️ Very complex arbitrary shadow
    let _ = tw.trace("bg-[#ff6b35]/15", false); // ⚠️ Arbitrary color with opacity
    let _ = tw.trace("shadow-[0_0_15px_rgba(255,107,53,0.2)]", false); // ⚠️ Very complex arbitrary shadow

    match tw.bundle() {
        Ok(css) => css,
        Err(e) => {
            console_log!("Error generating Tailwind bundle: {:?}", e);
            String::new()
        }
    }
}
