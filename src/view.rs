use crate::{Diary, Message};
use iced::widget::Column;
use crate::diary;


impl Diary {
    pub fn view(&self) -> Column<Message> {
        let diary_view = || diary::view_diary(
            &self.state.diary_state,
            |m| Message::DiaryMsg(m)
        );

        diary_view()
    }
}
