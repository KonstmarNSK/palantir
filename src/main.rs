mod view;
mod logic;
mod executor;
mod message;
mod diary;
mod state;

use iced::{Element, Settings, Application, Command};
use crate::executor::TokioRuntime;
use crate::logic::process_message;
use crate::message::Message;
use crate::state::State;


pub fn main() -> iced::Result {
    <Diary as Application>::run(Settings::default())
}


#[derive(Debug)]
struct Diary {
    state: State,
}

impl Application for Diary {
    type Executor = TokioRuntime;
    type Message = Message;
    type Theme = iced::theme::Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Diary, Command<Message>) {
        let diary_state = diary::State::new();
        let state = State { diary_state };

        (Diary { state }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        process_message(&mut self.state, message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.view().into()
    }
}
