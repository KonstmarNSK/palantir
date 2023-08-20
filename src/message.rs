use crate::diary;

#[derive(Debug, Clone)]
pub enum Message {
    DiaryMsg(diary::Message)
}