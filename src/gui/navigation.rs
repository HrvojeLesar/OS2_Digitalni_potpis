use iced::{widget::{button, row}, Element};

#[derive(Debug, Clone, Copy)]
pub enum NavigationState {
    KeyGen,
    EncryptDecrypt,
    Hashing,
    Sign,
}

pub struct NavigationButtons {
    current_state: NavigationState,
}

impl NavigationButtons {
    pub fn new() -> Self {
        Self {
            current_state: NavigationState::KeyGen,
        }
    }

    pub fn update(&mut self, state: NavigationState) {
        self.current_state = state;
    }

    pub fn view(&self) -> Element<NavigationState> {
        row![
            button("KeyGen").on_press(NavigationState::KeyGen),
            button("EncryptDecrypt").on_press(NavigationState::EncryptDecrypt),
            button("Hashing").on_press(NavigationState::Hashing),
            button("Sign").on_press(NavigationState::Sign),
        ]
        .spacing(20)
        .into()
    }
}
