use crate::{Diary, Message};
use iced::widget::{button, column, text, Column, Button, text_input};
use crate::diary;


impl Diary {
    pub fn view(&self) -> Column<Message> {
        diary::view_diary(&self.state.diary_state, |m| Message::DiaryMsg(m))
    }
}
