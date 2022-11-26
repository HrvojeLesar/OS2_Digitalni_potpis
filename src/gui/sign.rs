use iced::{
    widget::{self, button, row, text},
    Element,
};
use tinyfiledialogs::open_file_dialog;

use crate::{encryption::EncryptAsymmetric, file_manip::write_file};

#[derive(Debug, Clone, Copy)]
pub enum LoadFileType {
    File,
    Signature,
}

#[derive(Debug, Clone, Copy)]
pub enum SignMessage {
    LoadFile(LoadFileType),
    Sign,
    Verify,
}

pub struct SignView {
    selected_file: Option<String>,
    selected_signature: Option<String>,
    signed_files_hash: Option<String>,
    file_verified: Option<bool>,
}

impl SignView {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            selected_signature: None,
            signed_files_hash: None,
            file_verified: None,
        }
    }

    pub fn reset(&mut self) {
        self.selected_file = None;
        self.selected_signature = None;
        self.signed_files_hash = None;
        self.file_verified = None;
    }

    pub fn update(&mut self, message: SignMessage) {
        match message {
            SignMessage::LoadFile(f) => match f {
                LoadFileType::File => {
                    self.selected_file = open_file_dialog("Odabir datoteke", "", None)
                }
                LoadFileType::Signature => {
                    self.selected_signature =
                        open_file_dialog("Odabir datoteke s potpisom", "", None)
                }
            },
            SignMessage::Sign => {
                if let Some(path) = &self.selected_file {
                    let rsa = EncryptAsymmetric::from_files(None);
                    let signature = rsa.sign_file(path);
                    write_file("potpis", &signature, false);
                }
            }
            SignMessage::Verify => {
                if let (Some(file_path), Some(signature_path)) =
                    (&self.selected_file, &self.selected_signature)
                {
                    let rsa = EncryptAsymmetric::from_files(None);
                    let verify = rsa.verify_file_signature(file_path, signature_path);
                    self.file_verified = Some(verify);
                }
            }
        }
    }

    pub fn view(&self) -> Element<SignMessage> {
        let load_file_button =
            button("Odabir datoteke").on_press(SignMessage::LoadFile(LoadFileType::File));
        let load_signature =
            button("Odabir potpisa").on_press(SignMessage::LoadFile(LoadFileType::Signature));
        let sign_button = button("Potpisi").on_press(SignMessage::Sign);
        let verify_button = button("Provjera potpisa").on_press(SignMessage::Verify);
        let mut column = widget::column![row![
            load_file_button,
            load_signature,
            sign_button,
            verify_button
        ]];

        if let Some(hash) = self.signed_files_hash.as_ref() {
            column = column.push(text(hash));
        }

        if let Some(hash) = self.file_verified.as_ref() {
            column = column.push(text(hash));
        }

        column.into()
    }
}
