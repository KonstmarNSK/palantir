use iced::Command;
use crate::diary::{Message, State};
use crate::diary::Message::*;

pub fn process_message(state: &mut State, message: Message) -> Command<Message> {
    match message {
        SaveTextPressed => {
            // fixme: cloning whole diary state is too expensive
            Command::perform(save_curr_text(state.clone()), |r| r)
        },
        CurrentTextEdited(new_text) =>{
            state.current_text.body = new_text;
            Command::none()
        },
        CurrentTextSaved => Command::none(),
    }
}

async fn save_curr_text(state: State) -> Message {
    state.storage.save_new_text(&state.current_text).await;
    CurrentTextSaved
}


