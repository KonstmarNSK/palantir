#[derive(Debug)]
pub struct State {
    pub current_text: String,
}

impl State{
    pub fn new() -> Self {
        State{current_text: "".to_string()}
    }
}