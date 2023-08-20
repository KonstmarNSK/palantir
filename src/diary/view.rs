use iced::widget::{button, Column, column, text_input};
use crate::diary::{Message, State};
use crate::diary::Message::{CurrentTextEdited, SaveTextPressed};

pub fn view_diary<TMsgOut, TMsgMapper>(diary_state: &State, mapper: TMsgMapper) -> Column<TMsgOut>
    where
        TMsgOut: Clone + 'static,
        TMsgMapper: Fn(Message) -> TMsgOut + 'static
{
    let save_button = button("Save")
        .on_press(mapper(SaveTextPressed));

    let text_field = text_input(
        "Write your thoughts here",
        &diary_state.current_text,
        move |s| mapper(CurrentTextEdited(s))
    ).size(150);

    column![
            save_button,
            text_field,
        ]
}