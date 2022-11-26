use iced::{
    widget::{button, row},
    Element,
};

#[derive(Debug, Clone, Copy)]
pub enum NavigationStateMessage {
    KeyGen,
    EncryptDecrypt,
    Hashing,
    Sign,
}

pub struct NavigationButtons {
    pub current_state: NavigationStateMessage,
}

impl NavigationButtons {
    pub fn new() -> Self {
        Self {
            current_state: NavigationStateMessage::KeyGen,
        }
    }

    pub fn update(&mut self, state: NavigationStateMessage) {
        self.current_state = state;
    }

    pub fn view(&self) -> Element<NavigationStateMessage> {
        row![
            button("KeyGen").on_press(NavigationStateMessage::KeyGen),
            button("EncryptDecrypt").on_press(NavigationStateMessage::EncryptDecrypt),
            button("Hashing").on_press(NavigationStateMessage::Hashing),
            button("Sign").on_press(NavigationStateMessage::Sign),
        ]
        .spacing(20)
        .into()
    }
}
