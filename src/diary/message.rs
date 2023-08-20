#[derive(Debug, Clone)]
pub enum Message {
    SaveTextPressed,
    CurrentTextSaved,
    CurrentTextEdited(String),
}