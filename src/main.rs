use gui::encrypt_decrypt::{EncryptDecryptMessage, EncryptDecryptView};
use gui::keygen::{GenerateKeysView, KeyGenMessage};
use gui::navigation::{NavigationButtons, NavigationStateMessage};
use iced::widget;
use iced::{executor, Application, Command, Settings, Theme};

mod encryption;
mod file_manip;
mod gui;
mod keygen;

const PRIVATE_KEY_FILENAME: &str = "privatni_kljuc.txt";
const PUBLIC_KEY_FILENAME: &str = "javni_kljuc.txt";
const SECRET_KEY_FILENAME: &str = "tajni_kljuc.txt";

struct DigitalniPotpisApp {
    navigation_buttons: NavigationButtons,
    keygen_view: GenerateKeysView,
    encrypt_decrypt_view: EncryptDecryptView,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NavigationMessage(NavigationStateMessage),
    KeyGenMessage(KeyGenMessage),
    EncryptDecryptMessage(EncryptDecryptMessage),
}

impl Application for DigitalniPotpisApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                navigation_buttons: NavigationButtons::new(),
                keygen_view: GenerateKeysView::new(),
                encrypt_decrypt_view: EncryptDecryptView::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Digitalni Potpis")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::NavigationMessage(msg) => {
                self.navigation_buttons.update(msg);
                self.encrypt_decrypt_view.reset();
            }
            Message::KeyGenMessage(msg) => {
                self.keygen_view.update(msg);
                self.encrypt_decrypt_view.reset();
            }
            Message::EncryptDecryptMessage(msg) => self.encrypt_decrypt_view.update(msg),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let mut col = widget::column![self
            .navigation_buttons
            .view()
            .map(|mes| Message::NavigationMessage(mes))];
        col = match self.navigation_buttons.current_state {
            NavigationStateMessage::KeyGen => col.push(
                self.keygen_view
                    .view()
                    .map(|msg| Message::KeyGenMessage(msg)),
            ),
            NavigationStateMessage::EncryptDecrypt => col.push(
                self.encrypt_decrypt_view
                    .view()
                    .map(|msg| Message::EncryptDecryptMessage(msg)),
            ),
            _ => todo!(),
        };
        col.into()
    }
}

fn main() -> iced::Result {
    DigitalniPotpisApp::run(Settings::default())
}
