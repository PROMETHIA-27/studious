use iced::widget::{column, container, scrollable, text};
use iced::{Length, Sandbox, Settings};
use message::Message;
use state::Question;
use wasm_bindgen::prelude::*;

mod loading;
mod message;
mod state;
mod update;
mod view;

#[wasm_bindgen]
pub fn start_app() {
    App::run(Settings::default()).unwrap()
}

struct App {
    questions: Vec<Question>,
    selected: usize,
    num_correct: usize,
    num_answered: usize,
}

impl App {
    fn current(&self) -> &Question {
        &self.questions[self.selected]
    }

    fn next_q(&mut self) {
        self.selected = (self.selected + 1) % self.questions.len();
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            questions: loading::load_questions(),
            selected: 0,
            num_correct: 0,
            num_answered: 0,
        }
    }

    fn title(&self) -> String {
        "Studious".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AnswerTrueFalse(answer) => {
                if answer == self.current().unwrap_true_false().1 {
                    self.num_correct += 1;
                }
            }
            Message::AnswerMultipleChoice(answer) => {
                if answer == self.current().unwrap_multiple_choice().2 {
                    self.num_correct += 1;
                }
            }
        }
        self.num_answered += 1;
        self.next_q();
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let acc = (self.num_correct as f64 / self.num_answered as f64) * 100.0;
        let header = format!(
            "{}/{} Correct (Accuracy: {:.0}%)",
            self.num_correct,
            self.num_answered,
            if !acc.is_nan() { acc } else { 0.0 }
        );
        let header = container(text(header)).width(Length::Fill).center_x();
        let question = self
            .current()
            .view()
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();
        column![header, question].into()
    }
}
