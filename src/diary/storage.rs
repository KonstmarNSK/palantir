use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::diary::state::TextInfo;

const FILE_NAME: &str = "./saved-text.txt";

#[derive(Debug, Clone)]
pub struct Storage{

}

impl Storage {
    pub async fn save_new_text(&self, text_info: &TextInfo) {
        write_to_file(&text_info.body).await
    }
}


async fn write_to_file(text: &str) {
    let mut f = File::create(FILE_NAME).await.unwrap();
    println!("Saving content: {:?}", text);
    f.write(text.as_bytes()).await.unwrap();
    f.sync_all().await.unwrap();
}