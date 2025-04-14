use crate::api::{QuizQuestion, ResponseOption};
use yew::prelude::*;
use log::info;

#[derive(Properties, PartialEq, Clone)]
pub struct QuizQuestionProps {
    pub question: QuizQuestion,
    pub selected_value: Option<ResponseOption>,
    pub on_selection: Callback<(String, ResponseOption)>,
}

#[function_component(QuizQuestionComponent)]
pub fn quiz_question(props: &QuizQuestionProps) -> Html {
    let question = &props.question;
    let selected_value = props.selected_value;
    let on_selection = &props.on_selection;
    
    let handle_option_click = {
        let question_id = question.id.clone();
        let on_selection = on_selection.clone();
        
        Callback::from(move |option: ResponseOption| {
            info!("{} - Selected: {:?} ({})", 
                question_id, 
                option, 
                option as i32
            );
            on_selection.emit((question_id.clone(), option));
        })
    };
    
    html! {
        <div class="question-block">
            <h3>{ &question.term }</h3>
            <p>{ &question.definition }</p>
            
            <div id="position-buttons-for-user">
                {
                    ResponseOption::all_options().into_iter().map(|option| {
                        let is_selected = selected_value == Some(option);
                        let option_class = match option {
                            ResponseOption::StronglyDisagree => "strongly-disagree",
                            ResponseOption::Disagree => "disagree",
                            ResponseOption::Neutral => "neutral",
                            ResponseOption::Agree => "agree",
                            ResponseOption::StronglyAgree => "strongly-agree",
                        };
                        
                        let class = if is_selected {
                            format!("position-button {} selected", option_class)
                        } else {
                            format!("position-button {}", option_class)
                        };
                        
                        let option_clone = option;
                        let handle_click = handle_option_click.clone();
                        
                        html! {
                            <button 
                                class={class}
                                onclick={Callback::from(move |_| handle_click.emit(option_clone))}
                            >
                                { option_clone.to_string() }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}