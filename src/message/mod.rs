#[derive(Debug, Clone, Copy)]
pub enum Message {
    AnswerTrueFalse(bool),
    AnswerMultipleChoice(usize),
}
