/// Components - mod.rs
/// ===================
///
/// This file is like __init__.py in Python. It is the entry point for the module.
/// It makes the each component module available to the rest of the application.
pub mod return_home;
pub use return_home::ReturnHome;

pub mod navigation;
pub use navigation::Navigation;

pub mod quiz_question;
pub mod results_table;
pub mod score_display;

// Re-export components for easier imports
pub use quiz_question::QuizQuestionComponent;
pub use results_table::ResultsTable;
pub use score_display::ScoreDisplay;