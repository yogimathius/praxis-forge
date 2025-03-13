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
    let _ = tw.trace("text-center", false);
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

    // Add these to your get_tailwind_bundle function

    // All margin utilities
    let _ = tw.trace(
        "m-0 m-1 m-2 m-3 m-4 m-5 m-6 m-8 m-10 m-12 m-16 m-20 m-24 m-32 m-40 m-48 m-56 m-64",
        false,
    );
    let _ = tw.trace("mx-0 mx-1 mx-2 mx-3 mx-4 mx-5 mx-6 mx-8 mx-10 mx-12 mx-16 mx-20 mx-24 mx-32 mx-40 mx-48 mx-56 mx-64 mx-auto", false);
    let _ = tw.trace("my-0 my-1 my-2 my-3 my-4 my-5 my-6 my-8 my-10 my-12 my-16 my-20 my-24 my-32 my-40 my-48 my-56 my-64", false);
    let _ = tw.trace("mt-0 mt-1 mt-2 mt-3 mt-4 mt-5 mt-6 mt-8 mt-10 mt-12 mt-16 mt-20 mt-24 mt-32 mt-40 mt-48 mt-56 mt-64", false);
    let _ = tw.trace("mr-0 mr-1 mr-2 mr-3 mr-4 mr-5 mr-6 mr-8 mr-10 mr-12 mr-16 mr-20 mr-24 mr-32 mr-40 mr-48 mr-56 mr-64", false);
    let _ = tw.trace("mb-0 mb-1 mb-2 mb-3 mb-4 mb-5 mb-6 mb-8 mb-10 mb-12 mb-16 mb-20 mb-24 mb-32 mb-40 mb-48 mb-56 mb-64", false);
    let _ = tw.trace("ml-0 ml-1 ml-2 ml-3 ml-4 ml-5 ml-6 ml-8 ml-10 ml-12 ml-16 ml-20 ml-24 ml-32 ml-40 ml-48 ml-56 ml-64", false);

    // All padding utilities
    let _ = tw.trace(
        "p-0 p-1 p-2 p-3 p-4 p-5 p-6 p-8 p-10 p-12 p-16 p-20 p-24 p-32 p-40 p-48 p-56 p-64",
        false,
    );
    let _ = tw.trace("px-0 px-1 px-2 px-3 px-4 px-5 px-6 px-8 px-10 px-12 px-16 px-20 px-24 px-32 px-40 px-48 px-56 px-64", false);
    let _ = tw.trace("py-0 py-1 py-2 py-3 py-4 py-5 py-6 py-8 py-10 py-12 py-16 py-20 py-24 py-32 py-40 py-48 py-56 py-64", false);
    let _ = tw.trace("pt-0 pt-1 pt-2 pt-3 pt-4 pt-5 pt-6 pt-8 pt-10 pt-12 pt-16 pt-20 pt-24 pt-32 pt-40 pt-48 pt-56 pt-64", false);
    let _ = tw.trace("pr-0 pr-1 pr-2 pr-3 pr-4 pr-5 pr-6 pr-8 pr-10 pr-12 pr-16 pr-20 pr-24 pr-32 pr-40 pr-48 pr-56 pr-64", false);
    let _ = tw.trace("pb-0 pb-1 pb-2 pb-3 pb-4 pb-5 pb-6 pb-8 pb-10 pb-12 pb-16 pb-20 pb-24 pb-32 pb-40 pb-48 pb-56 pb-64", false);
    let _ = tw.trace("pl-0 pl-1 pl-2 pl-3 pl-4 pl-5 pl-6 pl-8 pl-10 pl-12 pl-16 pl-20 pl-24 pl-32 pl-40 pl-48 pl-56 pl-64", false);

    // Responsive variants
    let _ = tw.trace("md:m-0 md:m-4 md:m-8 md:m-16 md:m-24 md:m-32", false);
    let _ = tw.trace("md:mx-auto md:mx-0 md:mx-4 md:mx-8 md:mx-16", false);
    let _ = tw.trace("md:my-0 md:my-4 md:my-8 md:my-16 md:my-24", false);
    let _ = tw.trace("md:mt-0 md:mt-4 md:mt-8 md:mt-16 md:mt-24", false);
    let _ = tw.trace("md:mb-0 md:mb-4 md:mb-8 md:mb-16 md:mb-24", false);
    let _ = tw.trace("md:p-0 md:p-4 md:p-8 md:p-16 md:p-24", false);
    let _ = tw.trace("md:py-0 md:py-4 md:py-8 md:py-16 md:py-24", false);
    let _ = tw.trace("md:px-0 md:px-4 md:px-8 md:px-16 md:px-24", false);

    // Add these to your get_tailwind_bundle function
    let _ = tw.trace("space-y-2 space-y-4 space-y-6 space-y-8", false);
    let _ = tw.trace("space-x-2 space-x-4 space-x-6 space-x-8", false);
    let _ = tw.trace("gap-2 gap-4 gap-6 gap-8", false);

    match tw.bundle() {
        Ok(css) => css,
        Err(e) => {
            console_log!("Error generating Tailwind bundle: {:?}", e);
            String::new()
        }
    }
}
