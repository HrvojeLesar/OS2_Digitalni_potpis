use gui::navigation::{NavigationButtons, NavigationState};
use iced::widget::{button, column, row, text};
use iced::{Element, Sandbox, Settings};
use tinyfiledialogs::open_file_dialog;

mod encryption;
mod file_manip;
mod gui;
mod keygen;

const PRIVATE_KEY_FILENAME: &str = "privatni_kljuc.txt";
const PUBLIC_KEY_FILENAME: &str = "javni_kljuc.txt";
const SECRET_KEY_FILENAME: &str = "tajni_kljuc.txt";

struct DigitalniPotpisApp {
    value: i32,
    navigationButtons: NavigationButtons,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NavigationMessage(NavigationState),
}

impl Sandbox for DigitalniPotpisApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            navigationButtons: NavigationButtons::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Digitalni Potpis")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NavigationMessage(msg) => self.navigationButtons.update(msg),
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let col = column![
            text(self.value.to_string()).size(50),
            self.navigationButtons
                .view()
                .map(|mes| Message::NavigationMessage(mes))
        ];
        col.into()
    }
}

fn main() -> iced::Result {
    DigitalniPotpisApp::run(Settings::default())
}
