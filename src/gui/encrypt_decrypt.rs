use iced::{
    widget::{self, text},
    Element,
};
use openssl::symm::Cipher;
use tinyfiledialogs::open_file_dialog;

use crate::{
    encryption::{EncryptAsymmetric, EncryptSymmetric},
    file_manip::{read_file_to_buffer, write_file},
    SECRET_KEY_FILENAME,
};

use super::{
    path_to_filename,
    styled_components::{styled_button, styled_column, styled_row},
};

#[derive(Debug, Clone, Copy)]
pub enum EncryptDecryptMessage {
    LoadKeys,
    LoadFile,
    EncryptSymmetric,
    EncryptAsymmetric,
    DecryptSymmetric,
    DecryptAsymmetric,
}

pub struct EncryptDecryptView {
    selected_file: Option<String>,
    symmetric: Option<EncryptSymmetric>,
    asymmetric: Option<EncryptAsymmetric>,
}

impl EncryptDecryptView {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            symmetric: None,
            asymmetric: None,
        }
    }

    pub fn reset(&mut self) {
        self.selected_file = None;
        self.symmetric = None;
        self.asymmetric = None;
    }

    pub fn update(&mut self, message: EncryptDecryptMessage) {
        match message {
            EncryptDecryptMessage::LoadKeys => {
                self.asymmetric = Some(EncryptAsymmetric::from_files(None));
                let secret_key = read_file_to_buffer(SECRET_KEY_FILENAME);
                let cipher = Cipher::aes_128_cbc();
                self.symmetric = Some(EncryptSymmetric::new(cipher, secret_key, None));
            }
            EncryptDecryptMessage::LoadFile => {
                self.selected_file = open_file_dialog("Odabir datoteke", "", None);
            }
            EncryptDecryptMessage::EncryptAsymmetric => {
                if let (Some(encrypt), Some(path)) =
                    (self.asymmetric.as_ref(), self.selected_file.as_ref())
                {
                    let res = encrypt.public_encrypt_file(path);
                    write_file("rsa_enkriptirana_datoteka", &res, false);
                }
            }
            EncryptDecryptMessage::DecryptAsymmetric => {
                if let (Some(decrypt), Some(path)) =
                    (self.asymmetric.as_ref(), self.selected_file.as_ref())
                {
                    let res = decrypt.private_decrypt_file(path);
                    write_file("rsa_dekriptirana_datoteka", &res, false);
                }
            }
            EncryptDecryptMessage::EncryptSymmetric => {
                if let (Some(encrypt), Some(path)) =
                    (self.symmetric.as_ref(), self.selected_file.as_ref())
                {
                    let res = encrypt.encrypt_file(path);
                    write_file("aes_enkriptirana_datoteka", &res, false);
                }
            }
            EncryptDecryptMessage::DecryptSymmetric => {
                if let (Some(encrypt), Some(path)) =
                    (self.symmetric.as_ref(), self.selected_file.as_ref())
                {
                    let res = encrypt.decrypt_file(path);
                    write_file("aes_dekriptirana_datoteka", &res, false);
                }
            }
        }
    }

    pub fn view(&self) -> Element<EncryptDecryptMessage> {
        let load_keys_button = styled_button("Ucitaj kljuceve");
        let load_keys_button = if self.symmetric.is_none() && self.asymmetric.is_none() {
            widget::column![load_keys_button.on_press(EncryptDecryptMessage::LoadKeys)]
        } else {
            widget::column![text("Kljucevi su ucitani"), load_keys_button].spacing(5)
        };

        let load_file_button =
            styled_button("Odabir datoteke").on_press(EncryptDecryptMessage::LoadFile);

        let mut row = styled_row().push(load_keys_button);

        if let Some(path) = &self.selected_file {
            row = row.push(
                widget::column![
                    text(format!("Datoteka: {}", path_to_filename(path))),
                    load_file_button
                ]
                .spacing(5),
            );
        } else {
            row = row.push(load_file_button)
        }

        let mut col = styled_column(None);
        col = col.push(row);

        if let (Some(_asymmetric), Some(_symmetric), Some(_selected_file)) = (
            self.asymmetric.as_ref(),
            self.symmetric.as_ref(),
            self.selected_file.as_ref(),
        ) {
            col = col.push(
                styled_row()
                    .push(
                        styled_button("Enkriptiraj datuteku simetricnim algoritmom")
                            .on_press(EncryptDecryptMessage::EncryptSymmetric),
                    )
                    .push(
                        styled_button("Enkriptiraj datuteku asimetricnim algoritmom")
                            .on_press(EncryptDecryptMessage::EncryptAsymmetric),
                    )
                    .push(
                        styled_button("Dekriptiraj datuteku simetricnim algoritmom")
                            .on_press(EncryptDecryptMessage::DecryptSymmetric),
                    )
                    .push(
                        styled_button("Dekriptiraj datuteku asimetricnim algoritmom")
                            .on_press(EncryptDecryptMessage::DecryptAsymmetric),
                    ),
            );
        }

        col.into()
    }
}
