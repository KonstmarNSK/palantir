use iced::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::diary::{Message, State};
use crate::diary::Message::*;

const FILE_NAME: &str = "./saved-text.txt";

pub fn process_message(state: &mut State, message: Message) -> Command<Message> {
    match message {
        SaveTextPressed => {
            let text = state.current_text.clone();
            Command::perform(write_to_file(text), |r| r)
        },
        CurrentTextEdited(new_text) =>{
            state.current_text = new_text;
            Command::none()
        },
        CurrentTextSaved => Command::none(),
    }
}

async fn write_to_file(text: String) -> Message {
    let mut f = File::create(FILE_NAME).await.unwrap();
    println!("Saving content: {:?}", text.as_str());
    f.write(text.as_bytes()).await.unwrap();
    f.sync_all().await.unwrap();

    CurrentTextSaved
}



