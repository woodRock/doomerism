use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::prelude::*;

/// Question data structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub term: String,
    pub definition: String,
}

/// Category data structure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultCategory {
    pub id: String,
    pub name: String,
    pub min_score: i32,
    pub max_score: i32,
    pub person: String,
    pub comment: String,
}

/// Response options for the quiz
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseOption {
    StronglyDisagree = -2,
    Disagree = -1,
    Neutral = 0,
    Agree = 1,
    StronglyAgree = 2,
}

// Helper functions to convert ResponseOption to string and back
impl ResponseOption {
    pub fn to_string(&self) -> String {
        match self {
            ResponseOption::StronglyDisagree => "Strongly disagree".to_string(),
            ResponseOption::Disagree => "Disagree".to_string(),
            ResponseOption::Neutral => "Neutral".to_string(),
            ResponseOption::Agree => "Agree".to_string(),
            ResponseOption::StronglyAgree => "Strongly Agree".to_string(),
        }
    }
    
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            -2 => Some(ResponseOption::StronglyDisagree),
            -1 => Some(ResponseOption::Disagree),
            0 => Some(ResponseOption::Neutral),
            1 => Some(ResponseOption::Agree),
            2 => Some(ResponseOption::StronglyAgree),
            _ => None,
        }
    }
    
    pub fn all_options() -> Vec<ResponseOption> {
        vec![
            ResponseOption::StronglyDisagree,
            ResponseOption::Disagree,
            ResponseOption::Neutral,
            ResponseOption::Agree,
            ResponseOption::StronglyAgree,
        ]
    }
}

// Mock API function that would eventually be replaced with a Supabase call
pub async fn fetch_quiz_questions() -> Vec<QuizQuestion> {
    // For now, this returns static data, but would eventually fetch from Supabase
    vec![
        QuizQuestion {
            id: "transhumanism".to_string(),
            term: "Transhumanism".to_string(),
            definition: "The belief that humans can and should use technology to improve themselves (CoPilot).".to_string(),
        },
        QuizQuestion {
            id: "extropianism".to_string(),
            term: "Extropianism".to_string(),
            definition: "Evolving framework of values and standards for continuously improving the human condition (Wikipedia).".to_string(),
        },
        QuizQuestion {
            id: "singularitarianism".to_string(),
            term: "Singularitarianism".to_string(),
            definition: "The belief that the development of artificial intelligence will lead to a technological singularity (CoPilot).".to_string(),
        },
        QuizQuestion {
            id: "cosmism".to_string(),
            term: "Cosmism".to_string(),
            definition: "The doctrine that the material universe works automatically; affirmative atheism (Century Dictionary).".to_string(),
        },
        QuizQuestion {
            id: "rationalism".to_string(),
            term: "Rationalism".to_string(),
            definition: "The belief that reason is the chief source and test of knowledge (CoPilot).".to_string(),
        },
        QuizQuestion {
            id: "effective_altruism".to_string(),
            term: "Effective Altruism (EA)".to_string(),
            definition: "Sam Bankman-Fried appears to have engaged in extreme misconduct precisely because he believed in utilitarianism and effective altruism, and that his mostly EA-affiliated colleagues at FTX and Alameda Research went along with the plan for the same reasons (Vox).".to_string(),
        },
        QuizQuestion {
            id: "longtermism".to_string(),
            term: "Longtermism".to_string(),
            definition: "The belief that the long-term future of humanity is more important than the short-term (CoPilot).".to_string(),
        },
    ]
}

// Mock API function that would eventually be replaced with a Supabase call
pub async fn fetch_result_categories() -> Vec<ResultCategory> {
    // For now, this returns static data, but would eventually fetch from Supabase
    vec![
        ResultCategory {
            id: "bloomer".to_string(),
            name: "Bloomer".to_string(),
            min_score: -999, // Using a large negative number as lower bound
            max_score: -14,
            person: "Sam Altman".to_string(),
            comment: "Prepare to be ALIGNED!".to_string(),
        },
        ResultCategory {
            id: "bloom_curious".to_string(),
            name: "Bloom-curious".to_string(),
            min_score: -13,
            max_score: -8,
            person: "Yann LeCun".to_string(),
            comment: "'AI doomism is quickly becoming indistinguishable from an apocalyptic religion.' ~ Yann LeCun".to_string(),
        },
        ResultCategory {
            id: "npc".to_string(),
            name: "NPC".to_string(),
            min_score: -7,
            max_score: 7,
            person: "Elon Musk".to_string(),
            comment: "Stick to cars, and rockets, and internet, future king of Mars! 👑".to_string(),
        },
        ResultCategory {
            id: "doom_curious".to_string(),
            name: "Doom-curious".to_string(),
            min_score: 8,
            max_score: 13,
            person: "Nick Bostrom".to_string(),
            comment: "Terminator was good... but it was just a movie! 😎".to_string(),
        },
        ResultCategory {
            id: "doomer".to_string(),
            name: "Doomer".to_string(),
            min_score: 14,
            max_score: 999, // Using a large number as upper bound
            person: "Eliezer Yudkowsky".to_string(),
            comment: "Please unblock me on Twitter! ❤️".to_string(),
        },
    ]
}

// Function to determine the user's category based on score
pub fn get_category_for_score(score: i32, categories: &[ResultCategory]) -> Option<ResultCategory> {
    categories
        .iter()
        .find(|category| score >= category.min_score && score <= category.max_score)
        .cloned()
}