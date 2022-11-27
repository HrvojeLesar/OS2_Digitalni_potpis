use iced::{
    widget::{self, text},
    Element,
};
use tinyfiledialogs::open_file_dialog;

use crate::{encryption::EncryptAsymmetric, file_manip::write_file};

use super::{
    path_to_filename,
    styled_components::{styled_button, styled_column, styled_row},
};

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
        let load_file_button = if let Some(path) = self.selected_file.as_ref() {
            widget::column![
                text(format!("Datoteka: {}", path_to_filename(path))),
                styled_button("Odabir datoteke")
                    .on_press(SignMessage::LoadFile(LoadFileType::File))
            ]
            .spacing(5)
        } else {
            widget::column![styled_button("Odabir datoteke")
                .on_press(SignMessage::LoadFile(LoadFileType::File))]
            .spacing(5)
        };
        let load_signature = if let Some(path) = self.selected_signature.as_ref() {
            widget::column![
                text(format!("Datoteka: {}", path_to_filename(path))),
                styled_button("Odabir potpisa")
                    .on_press(SignMessage::LoadFile(LoadFileType::Signature))
            ]
            .spacing(5)
        } else {
            widget::column![styled_button("Odabir potpisa")
                .on_press(SignMessage::LoadFile(LoadFileType::Signature))]
            .spacing(5)
        };
        let sign_button = if let Some(_sf) = self.selected_file.as_ref() {
            styled_button("Potpisi").on_press(SignMessage::Sign)
        } else {
            styled_button("Potpisi")
        };

        let verify_button = if let (Some(_sf), Some(_ss)) = (
            self.selected_file.as_ref(),
            self.selected_signature.as_ref(),
        ) {
            styled_button("Provjera potpisa").on_press(SignMessage::Verify)
        } else {
            styled_button("Provjera potpisa")
        };
        let mut column =
            styled_column(None).push(styled_row().push(load_file_button).push(load_signature));
        column = column.push(styled_row().push(sign_button).push(verify_button));

        if let Some(hash) = self.signed_files_hash.as_ref() {
            column = column.push(text(hash));
        }

        if let Some(hash) = self.file_verified.as_ref() {
            column = column.push(text(hash));
        }

        column.into()
    }
}
