/// Home - home.rs
/// ==============
/// This is the home page. It is the first page that the user sees when they visit the website.
use yew::prelude::*;
use yew::{html, Callback};
use log::info;

#[function_component(Home)]
pub fn home() -> Html {
    // State for tracking scores
    let transhumanism_score = use_state(|| 0);
    let extropianism_score = use_state(|| 0);
    let singularitarianism_score = use_state(|| 0);
    let cosmism_score = use_state(|| 0);
    let rationalism_score = use_state(|| 0);
    let ea_score = use_state(|| 0);
    let longtermism_score = use_state(|| 0);
    
    // State for showing/hiding results
    let show_results = use_state(|| false);
    
    // Create a callback for revealing results
    let on_show_results = {
        let show_results = show_results.clone();
        Callback::from(move |_| {
            show_results.set(true);
        })
    };
    
    // Calculate total score
    let total_score = *transhumanism_score + *extropianism_score + *singularitarianism_score + 
                      *cosmism_score + *rationalism_score + *ea_score + *longtermism_score;
    
    // Determine the category based on total score
    let (category, person, comment) = match total_score {
        s if s <= -14 => ("Bloomer", "Sam Altman", "Prepare to be ALIGNED!"),
        s if s >= -13 && s <= -8 => ("Bloom-curious", "Yann LeCun", "'AI doomism is quickly becoming indistinguishable from an apocalyptic religion.' ~ Yann LeCun"),
        s if s >= -7 && s <= 7 => ("NPC", "Elon Musk", "Stick to cars, and rockets, and internet, future king of Mars! 👑"),
        s if s >= 8 && s <= 13 => ("Doom-curious", "Nick Bostrom", "Terminator was good... but it was just a movie! 😎"),
        _ => ("Doomer", "Eliezer Yudkowsky", "Please unblock me on Twitter! ❤️"),
    };

    html! {
        <div class="quiz-container">
            <div class="quiz-header">
                <h1>{ "How doomer are you?" }</h1>
                <p>{ "For each term, respond to how much your beliefs align with the term, and the definition provided here."}</p>
                <p>{ "Select your answer for each question, then click 'Reveal My Score' at the end!"}</p>
            </div>
            
            <div class="quiz-content">
                <h3>{ "Transhumanism" }</h3>
                <p>{ "The belief that humans can and should use technology to improve themselves (CoPilot)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *transhumanism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = transhumanism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Transhumanism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *transhumanism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = transhumanism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Transhumanism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *transhumanism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = transhumanism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Transhumanism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *transhumanism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = transhumanism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Transhumanism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *transhumanism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = transhumanism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Transhumanism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>

                <h3>{ "Extropianism" }</h3>
                <p>{ "Evolving framework of values and standards for continuously improving the human condition (Wikipedia)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *extropianism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = extropianism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Extropianism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *extropianism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = extropianism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Extropianism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *extropianism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = extropianism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Extropianism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *extropianism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = extropianism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Extropianism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *extropianism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = extropianism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Extropianism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>

                <h3>{ "Singularitarianism" }</h3>
                <p>{ "The belief that the development of artificial intelligence will lead to a technological singularity (CoPilot)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *singularitarianism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = singularitarianism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Singularitarianism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *singularitarianism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = singularitarianism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Singularitarianism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *singularitarianism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = singularitarianism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Singularitarianism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *singularitarianism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = singularitarianism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Singularitarianism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *singularitarianism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = singularitarianism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Singularitarianism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>
            
                <h3>{ "Cosmism" }</h3>
                <p>{ "The doctrine that the material universe works automatically; affirmative atheism (Century Dictionary)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *cosmism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = cosmism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Cosmism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *cosmism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = cosmism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Cosmism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *cosmism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = cosmism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Cosmism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *cosmism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = cosmism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Cosmism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *cosmism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = cosmism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Cosmism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>

                <h3>{ "Rationalism" }</h3>
                <p>{ "The belief that reason is the chief source and test of knowledge (CoPilot)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *rationalism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = rationalism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Rationalism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *rationalism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = rationalism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Rationalism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *rationalism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = rationalism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Rationalism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *rationalism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = rationalism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Rationalism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *rationalism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = rationalism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Rationalism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>

                <h3>{ "Effective Altruism (EA)" }</h3>
                <p>{ "Sam Bankman-Fried appears to have engaged in extreme misconduct precisely because he believed in utilitarianism and effective altruism, and that his mostly EA-affiliated colleagues at FTX and Alameda Research went along with the plan for the same reasons (Vox)."}</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *ea_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = ea_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("EA - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *ea_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = ea_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("EA - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *ea_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = ea_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("EA - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *ea_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = ea_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("EA - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *ea_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = ea_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("EA - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>

                <h3>{ "Longtermism" }</h3>
                <p>{ "The belief that the long-term future of humanity is more important than the short-term (CoPilot)." }</p>
                <div id="position-buttons-for-user">
                    <button 
                        class={if *longtermism_score == -2 { "selected" } else { "" }}
                        onclick={
                            let score = longtermism_score.clone();
                            Callback::from(move |_| {
                                score.set(-2);
                                info!("Longtermism - Strongly disagree (-2)");
                            })
                        }
                    >
                        { "Strongly disagree" }
                    </button>
                    <button 
                        class={if *longtermism_score == -1 { "selected" } else { "" }}
                        onclick={
                            let score = longtermism_score.clone();
                            Callback::from(move |_| {
                                score.set(-1);
                                info!("Longtermism - Disagree (-1)");
                            })
                        }
                    >
                        { "Disagree" }
                    </button>
                    <button 
                        class={if *longtermism_score == 0 { "selected" } else { "" }}
                        onclick={
                            let score = longtermism_score.clone();
                            Callback::from(move |_| {
                                score.set(0);
                                info!("Longtermism - Neutral (0)");
                            })
                        }
                    >
                        { "Neutral" }
                    </button>
                    <button 
                        class={if *longtermism_score == 1 { "selected" } else { "" }}
                        onclick={
                            let score = longtermism_score.clone();
                            Callback::from(move |_| {
                                score.set(1);
                                info!("Longtermism - Agree (1)");
                            })
                        }
                    >
                        { "Agree" }
                    </button>
                    <button 
                        class={if *longtermism_score == 2 { "selected" } else { "" }}
                        onclick={
                            let score = longtermism_score.clone();
                            Callback::from(move |_| {
                                score.set(2);
                                info!("Longtermism - Strongly Agree (2)");
                            })
                        }
                    >
                        { "Strongly Agree" }
                    </button>
                </div>
                
                <div class="finished-section">
                    <h1>{ "Finished?" }</h1>
                    <p>{ "Click the button below to see your score!" }</p>
                    <button class="reveal-button" onclick={on_show_results}>
                        { "Reveal My Score" }
                    </button>
                </div>
                
                // Show results if the button has been clicked
                {
                    if *show_results {
                        html! {
                            <div class="score-result">
                                <h2>{ "Your Doomer Score:" }</h2>
                                <div class="score-value">{ total_score }</div>
                                <div class="score-category">{ format!("You are: {}", category) }</div>
                                <div class="score-person">{ format!("Your person: {}", person) }</div>
                                <div class="score-comment">{ comment }</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                
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
                            <tr>
                                <td>{ "Bloomer" }</td>
                                <td>{ "-14 or lower" }</td>
                                <td>{ "Sam Altman" }</td>
                                <td>{ "Prepare to be ALIGNED!" }</td>
                            </tr>
                            <tr>
                                <td>{ "Bloom-curious" }</td>
                                <td>{ "-13 to -8" }</td>
                                <td>{ "Yann LeCun" }</td>
                                <td>{ "'AI doomism is quickly becoming indistinguishable from an apocalyptic religion.' ~ Yann LeCun" }</td>
                            </tr>
                            <tr>
                                <td>{ "NPC" }</td>
                                <td>{ "-7 to 7" }</td>
                                <td>{ "Elon Musk" }</td>
                                <td>{ "Stick to cars, and rockets, and internet, future king of Mars! 👑" }</td>
                            </tr>
                            <tr>
                                <td>{ "Doom-curious" }</td>
                                <td>{ "8 to 13" }</td>
                                <td>{ "Nick Bostrom" }</td>
                                <td>{ "Terminator was good... but it was just a movie! 😎" }</td>
                            </tr>
                            <tr>
                                <td>{ "Doomer" }</td>
                                <td>{ "14+" }</td>
                                <td>{ "Eliezer Yudkowsky" }</td>
                                <td>{ "Please unblock me on Twitter! ❤️" }</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}