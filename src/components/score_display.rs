use crate::api::ResultCategory;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ScoreDisplayProps {
    pub score: i32,
    pub category: ResultCategory,
    #[prop_or(false)]
    pub visible: bool,
}

#[function_component(ScoreDisplay)]
pub fn score_display(props: &ScoreDisplayProps) -> Html {
    if !props.visible {
        return html! {};
    }
    
    html! {
        <div class="score-result">
            <h2>{ "Your Doomer Score:" }</h2>
            <div class="score-value">{ props.score }</div>
            <div class="score-category">{ format!("You are: {}", props.category.name) }</div>
            <div class="score-person">{ format!("Your person: {}", props.category.person) }</div>
            <div class="score-comment">{ &props.category.comment }</div>
        </div>
    }
}