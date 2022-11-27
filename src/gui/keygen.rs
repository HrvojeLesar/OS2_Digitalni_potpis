use std::fs::File;

use iced::{
    widget::{self, row, text},
    Element,
};

use crate::{
    encryption::EncryptAsymmetric, file_manip::write_file, keygen::Keygen, PRIVATE_KEY_FILENAME,
    PUBLIC_KEY_FILENAME, SECRET_KEY_FILENAME,
};

use super::styled_components::styled_button;

#[derive(Debug, Clone, Copy)]
pub enum KeyGenMessage {
    GenerateSecretKey,
    GenerateKeyPair,
}

pub struct GenerateKeysView {}

impl GenerateKeysView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: KeyGenMessage) {
        match message {
            KeyGenMessage::GenerateSecretKey => {
                let key = Keygen::default().generate_128bit_key();
                write_file(SECRET_KEY_FILENAME, &key, false);
            }
            KeyGenMessage::GenerateKeyPair => {
                EncryptAsymmetric::new_save_keys();
            }
        }
    }

    pub fn view(&self) -> Element<KeyGenMessage> {
        let mut row = row![];
        let secret_key_button =
            styled_button("Generiraj tajni kljuc").on_press(KeyGenMessage::GenerateSecretKey);
        let keypair_button =
            styled_button("Generiraj par kljuceva").on_press(KeyGenMessage::GenerateKeyPair);
        row = if File::open(SECRET_KEY_FILENAME).is_ok() {
            row.push(widget::column![text("Tajni kljuc vec postoji"), secret_key_button].spacing(5))
        } else {
            row.push(secret_key_button)
        };

        row = if File::open(PRIVATE_KEY_FILENAME).is_ok() && File::open(PUBLIC_KEY_FILENAME).is_ok()
        {
            row.push(widget::column![text("Par kljuceva vec postoji"), keypair_button].spacing(5))
        } else {
            row.push(keypair_button)
        };

        row.spacing(20).into()
    }
}
