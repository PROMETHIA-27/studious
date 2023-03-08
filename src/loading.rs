use crate::state::{Question, Questions};

pub fn load_questions() -> Vec<Question> {
    let data = std::fs::read_to_string("assets/matchings_and_factors.toml").unwrap();
    let questions: Questions = toml::from_str(&data).unwrap();
    questions.questions
}
