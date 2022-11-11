use iced::{button, Button, Column, Sandbox, Text, Settings};

mod encryption;
mod keygen;

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            increment_button: button::State::default(),
            decrement_button: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
