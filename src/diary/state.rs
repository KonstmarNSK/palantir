use crate::diary::storage::Storage;

#[derive(Debug, Clone)]
pub struct State {
    pub current_text: TextInfo,
    pub storage: Storage,
}

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub title: String,
    pub body: String,
}

impl State {
    pub fn new() -> Self {
        let current_text = TextInfo { title: "".to_string(), body: "".to_string() };
        let storage = Storage{};

        State { current_text, storage }
    }
}