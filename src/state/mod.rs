use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Question {
    TrueFalse {
        text: String,
        correct: bool,
    },
    MultipleChoice {
        text: String,
        choices: Vec<String>,
        correct: usize,
    },
}

impl Question {
    pub fn unwrap_true_false(&self) -> (&str, bool) {
        match self {
            Question::TrueFalse { text, correct } => (&text, *correct),
            _ => panic!("tried to unwrap a true/false as something other than true/false"),
        }
    }

    pub fn unwrap_multiple_choice(&self) -> (&str, &[String], usize) {
        match self {
            Question::MultipleChoice {
                text,
                choices,
                correct,
            } => (&text, &choices, *correct),
            _ => {
                panic!("tried to unwrap a multiple choice as something other than multiple choice")
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Questions {
    pub questions: Vec<Question>,
}
