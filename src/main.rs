use iced::widget::{button, column, text, Column};
use iced::{Element, Sandbox, Settings, Application};

// fn main() {
//     println!("Hello, world!");
// }


pub fn main() -> iced::Result {
    <Counter as Sandbox>::run(Settings::default())
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Counter {
        Counter { value: 0 }
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) {
        Counter::update(self, message)
    }

    fn view(&self) -> Element<Self::Message> {
        self.view().into()
    }
}


#[derive(Debug)]
struct Counter {
    // The counter value
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::IncrementPressed),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
        ]
    }
}

impl Counter {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}