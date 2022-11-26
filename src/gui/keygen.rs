use std::fs::File;

use iced::{
    widget::{button, column, row, text},
    Element,
};

use crate::{
    encryption::EncryptAsymmetric, file_manip::write_file, keygen::Keygen, PRIVATE_KEY_FILENAME,
    PUBLIC_KEY_FILENAME, SECRET_KEY_FILENAME,
};

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
            button("Generiraj tajni kljuc").on_press(KeyGenMessage::GenerateSecretKey);
        let keypair_button =
            button("Generiraj par kljuceva").on_press(KeyGenMessage::GenerateKeyPair);
        row = if File::open(SECRET_KEY_FILENAME).is_ok() {
            row.push(column![text("Tajni kljuc vec postoji"), secret_key_button])
        } else {
            row.push(secret_key_button)
        };

        row = if File::open(PRIVATE_KEY_FILENAME).is_ok() && File::open(PUBLIC_KEY_FILENAME).is_ok()
        {
            row.push(column![text("Par kljuceva vec postoji"), keypair_button])
        } else {
            row.push(keypair_button)
        };

        row.spacing(20).into()
    }
}
