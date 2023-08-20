use std::io;
use iced::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use Message::*;
use crate::{Message, State};
use crate::diary;

pub fn process_message(state: &mut State, msg: Message) -> Command<Message> {
    match msg {
        DiaryMsg(msg) => diary::process_message(&mut state.diary_state, msg)
            .map(|m| DiaryMsg(m)),
    }
}