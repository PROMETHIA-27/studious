use iced::widget::{button, column, container, radio, text, Column, Container};
use iced::Length;

use crate::message::Message;
use crate::state::Question;

impl Question {
    pub fn view(&self) -> Container<Message> {
        match self {
            Self::TrueFalse {
                text: qtext,
                correct,
            } => {
                let qtext = container(text(qtext)).width(Length::Fill).center_x();
                let answers = container(column![
                    radio("True", true, None, |_| Message::AnswerTrueFalse(true)),
                    radio("False", false, None, |_| Message::AnswerTrueFalse(false)),
                ])
                .width(Length::Fill)
                .center_x();
                container(column![qtext, answers])
            }
            Self::MultipleChoice {
                text: qtext,
                choices,
                correct,
            } => {
                let qtext = container(text(qtext)).width(Length::Fill).center_x();
                let answers = container(Column::with_children(
                    choices
                        .iter()
                        .enumerate()
                        .map(|(idx, ans)| {
                            radio(ans, idx, None, |_| Message::AnswerMultipleChoice(idx)).into()
                        })
                        .collect(),
                ))
                .width(Length::Fill)
                .center_x();
                container(column![qtext, answers])
            }
        }
    }
}
