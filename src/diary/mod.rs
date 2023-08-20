mod logic;
mod view;
mod state;
mod message;

use iced::widget::Column;
pub use logic::process_message;
pub use state::State;
pub use message::Message;
pub use view::view_diary;
