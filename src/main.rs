use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings};

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
