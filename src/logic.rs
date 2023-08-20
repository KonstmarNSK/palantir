use iced::Command;
use Message::*;
use crate::{Message, State};
use crate::diary;

pub fn process_message(state: &mut State, msg: Message) -> Command<Message> {
    match msg {
        DiaryMsg(msg) => diary::process_message(&mut state.diary_state, msg)
            .map(|m| DiaryMsg(m)),
    }
}