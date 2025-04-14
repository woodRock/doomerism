use crate::api::ResultCategory;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ResultsTableProps {
    pub categories: Vec<ResultCategory>,
    pub current_score: Option<i32>,
}

#[function_component(ResultsTable)]
pub fn results_table(props: &ResultsTableProps) -> Html {
    let categories = &props.categories;
    let current_score = props.current_score;
    
    html! {
        <div class="results-section">
            <h2>{ "Results" }</h2>
            <table>
                <thead>
                    <tr>
                        <th>{ "Category" }</th>
                        <th>{ "Score Range" }</th> 
                        <th>{ "Person" }</th> 
                        <th>{ "Comment" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        categories.iter().map(|category| {
                            // Check if this category matches the current score
                            let is_current = if let Some(score) = current_score {
                                score >= category.min_score && score <= category.max_score
                            } else {
                                false
                            };
                            
                            let row_class = format!("category-{}", category.id);
                            let row_class = if is_current {
                                format!("{} current-category", row_class)
                            } else {
                                row_class
                            };
                            
                            let score_range = if category.min_score <= -100 {
                                format!("{} or lower", category.max_score)
                            } else if category.max_score >= 100 {
                                format!("{} or higher", category.min_score)
                            } else {
                                format!("{} to {}", category.min_score, category.max_score)
                            };
                            
                            html! {
                                <tr class={row_class}>
                                    <td>{ &category.name }</td>
                                    <td>{ score_range }</td>
                                    <td>{ &category.person }</td>
                                    <td>{ &category.comment }</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}