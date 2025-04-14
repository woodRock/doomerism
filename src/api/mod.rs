pub mod quiz_data;

// Re-export commonly used types and functions
pub use quiz_data::{
    QuizQuestion,
    ResultCategory,
    ResponseOption,
    fetch_quiz_questions,
    fetch_result_categories,
    get_category_for_score,
};

// Future Supabase client configuration would go here
// pub mod supabase;