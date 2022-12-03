use iced::Element;

use super::styled_components::{styled_button, styled_row};

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
        styled_row()
            .push(styled_button("Generiranje kljuceva").on_press(NavigationStateMessage::KeyGen))
            .push(
                styled_button("Enkripcija / dekripcija")
                    .on_press(NavigationStateMessage::EncryptDecrypt),
            )
            .push(styled_button("Sazetak").on_press(NavigationStateMessage::Hashing))
            .push(styled_button("Potpis").on_press(NavigationStateMessage::Sign))
            .into()
    }
}
