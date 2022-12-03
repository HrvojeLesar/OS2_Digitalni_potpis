use gui::encrypt_decrypt::{EncryptDecryptMessage, EncryptDecryptView};
use gui::hash::{HashMessage, HashView};
use gui::keygen::{GenerateKeysView, KeyGenMessage};
use gui::navigation::{NavigationButtons, NavigationStateMessage};
use gui::sign::{SignMessage, SignView};
use gui::styled_components::styled_column;
use iced::widget::{container, scrollable};
use iced::{executor, Application, Command, Padding, Settings, Theme};

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
    hashing_view: HashView,
    sign_view: SignView,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NavigationMessage(NavigationStateMessage),
    KeyGenMessage(KeyGenMessage),
    EncryptDecryptMessage(EncryptDecryptMessage),
    HashMessage(HashMessage),
    SignMessage(SignMessage),
    ErrorMessage,
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
                hashing_view: HashView::new(),
                sign_view: SignView::new(),
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
                self.hashing_view.reset();
                self.sign_view.reset();
            }
            Message::KeyGenMessage(msg) => self.keygen_view.update(msg),
            Message::EncryptDecryptMessage(msg) => self.encrypt_decrypt_view.update(msg),
            Message::HashMessage(msg) => self.hashing_view.update(msg),
            Message::SignMessage(msg) => self.sign_view.update(msg),
            Message::ErrorMessage => (),
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let mut col = styled_column(Some("Digitalni potpis")).push(
            self.navigation_buttons
                .view()
                .map(Message::NavigationMessage),
        );
        col = match self.navigation_buttons.current_state {
            NavigationStateMessage::KeyGen => {
                col.push(self.keygen_view.view().map(Message::KeyGenMessage))
            }
            NavigationStateMessage::EncryptDecrypt => col.push(
                self.encrypt_decrypt_view
                    .view()
                    .map(Message::EncryptDecryptMessage),
            ),
            NavigationStateMessage::Hashing => {
                col.push(self.hashing_view.view().map(Message::HashMessage))
            }
            NavigationStateMessage::Sign => {
                col.push(self.sign_view.view().map(Message::SignMessage))
            }
        };
        let main_container = container(scrollable(
            container(col).width(iced::Length::Fill).center_x(),
        ));
        main_container
            .height(iced::Length::Fill)
            .padding(Padding::from([30, 10, 10, 10]))
            .into()
    }
}

fn main() -> iced::Result {
    DigitalniPotpisApp::run(Settings::default())
}
