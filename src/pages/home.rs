use yew::prelude::*;
use yew::{html, Callback};
use log::info;
use gloo::console::log;
use std::collections::HashMap;
use std::rc::Rc;

use crate::api::{
    QuizQuestion, 
    ResultCategory, 
    ResponseOption,
    fetch_quiz_questions, 
    fetch_result_categories,
    get_category_for_score
};

use crate::components::{
    QuizQuestionComponent,
    ResultsTable,
    ScoreDisplay,
};

/// Home - Main quiz page component
#[function_component(Home)]
pub fn home() -> Html {
    // State hooks
    let questions = use_state(|| Vec::<QuizQuestion>::new());
    let categories = use_state(|| Vec::<ResultCategory>::new());
    let responses = use_state(|| HashMap::<String, ResponseOption>::new());
    let show_results = use_state(|| false);
    let loading = use_state(|| true);
    
    // Calculate total score based on responses
    let total_score = {
        let responses = &*responses;
        responses.values().map(|&option| option as i32).sum::<i32>()
    };
    
    // Get the category based on the score
    let result_category = {
        let categories = &*categories;
        get_category_for_score(total_score, categories)
    };
    
    // Effect to load questions and categories on mount
    {
        let questions = questions.clone();
        let categories = categories.clone();
        let loading = loading.clone();
        
        use_effect_with_deps(
            move |_| {
                let questions_clone = questions.clone();
                let categories_clone = categories.clone();
                let loading_clone = loading.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    // Fetch questions and categories in parallel
                    let quiz_questions = fetch_quiz_questions().await;
                    let result_categories = fetch_result_categories().await;
                    
                    questions_clone.set(quiz_questions);
                    categories_clone.set(result_categories);
                    loading_clone.set(false);
                });
                
                || ()
            },
            (),
        );
    }
    
    // Callback for when a question option is selected
    let on_question_selection = {
        let responses = responses.clone();
        
        Callback::from(move |(question_id, option): (String, ResponseOption)| {
            let mut new_responses = (*responses).clone();
            new_responses.insert(question_id, option);
            responses.set(new_responses);
        })
    };
    
    // Callback for revealing results
    let on_reveal_results = {
        let show_results = show_results.clone();
        let responses_count = responses.len();
        let questions_count = questions.len();
        
        Callback::from(move |_| {
            // Optional: Check if all questions are answered
            if responses_count < questions_count {
                log!("Not all questions answered");
                // You could show a notification here
            }
            
            show_results.set(true);
        })
    };
    
    html! {
        <div class="quiz-container">
            <div class="quiz-header">
                <h1>{ "How doomer are you?" }</h1>
                <p>{ "For each term, respond to how much your beliefs align with the term, and the definition provided here."}</p>
                <p>{ "Select your answer for each question, then click 'Reveal My Score' at the end!"}</p>
            </div>
            
            <div class="quiz-content">
                {
                    if *loading {
                        html! { <div class="loading">{ "Loading quiz questions..." }</div> }
                    } else {
                        html! {
                            <>
                                // Render all questions
                                {
                                    questions.iter().map(|question| {
                                        let selected_value = responses.get(&question.id).cloned();
                                        
                                        html! {
                                            <QuizQuestionComponent 
                                                question={question.clone()}
                                                selected_value={selected_value}
                                                on_selection={on_question_selection.clone()}
                                            />
                                        }
                                    }).collect::<Html>()
                                }
                                
                                // Finished section
                                <div class="finished-section">
                                    <h1>{ "Finished?" }</h1>
                                    <p>{ "Click the button below to see your score!" }</p>
                                    <button class="reveal-button" onclick={on_reveal_results}>
                                        { "Reveal My Score" }
                                    </button>
                                </div>
                                
                                // Score display (only shown when results are revealed)
                                {
                                    if let Some(category) = &result_category {
                                        html! {
                                            <ScoreDisplay 
                                                score={total_score}
                                                category={category.clone()}
                                                visible={*show_results}
                                            />
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                // Results table
                                <ResultsTable 
                                    categories={(*categories).clone()}
                                    current_score={if *show_results { Some(total_score) } else { None }}
                                />
                            </>
                        }
                    }
                }
            </div>
        </div>
    }
}