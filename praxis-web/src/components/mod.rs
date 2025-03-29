pub mod ai;
pub mod animation;
pub mod common;
pub mod debug;
pub mod goal;
pub mod home;
pub mod metrics;
pub mod navigation;
pub mod progress;
pub mod task;

pub use debug::signal_visualizer::SignalVisualizer;
pub use goal::form::GoalForm;
pub use goal::list::GoalsList;
pub use home::Principles;
pub use metrics::performance_monitor::PerformanceMonitor;
pub use navigation::Navigation;
pub use progress::ProgressBar;
pub use task::form::TaskForm;
pub use task::list::TasksList;

// New components
pub use ai::smart_suggestions::SmartSuggestions;
pub use common::command_palette::CommandPalette;
pub use common::theme_toggle::{provide_theme, ThemeToggle};
