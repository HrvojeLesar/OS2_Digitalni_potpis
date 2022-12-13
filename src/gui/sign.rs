use iced::{
    widget::{self, text},
    Element,
};
use tinyfiledialogs::open_file_dialog;

use crate::{encryption::EncryptRsa, file_manip::write_file};

use super::{
    path_to_filename,
    styled_components::{styled_button, styled_column, styled_error, styled_row, GREEN, RED},
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
    file_verified: Option<bool>,
    error: Option<anyhow::Error>,
}

impl SignView {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            selected_signature: None,
            file_verified: None,
            error: None,
        }
    }

    pub fn reset(&mut self) {
        self.selected_file = None;
        self.selected_signature = None;
        self.file_verified = None;
        self.error = None;
    }

    pub fn update(&mut self, message: SignMessage) {
        self.error = None;
        let rsa = match EncryptRsa::from_files(None) {
            Ok(rsa) => rsa,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };
        match message {
            SignMessage::LoadFile(f) => match f {
                LoadFileType::File => {
                    self.file_verified = None;
                    self.selected_file = open_file_dialog("Odabir datoteke", "", None);
                }
                LoadFileType::Signature => {
                    self.file_verified = None;
                    self.selected_signature =
                        open_file_dialog("Odabir datoteke s potpisom", "", None);
                }
            },
            SignMessage::Sign => {
                if let Some(path) = &self.selected_file {
                    let signature = match rsa.sign_file(path) {
                        Ok(sig) => sig,
                        Err(e) => {
                            self.error = Some(e);
                            return;
                        }
                    };
                    match write_file("potpis", &signature, false) {
                        Ok(_) => (),
                        Err(e) => {
                            self.error = Some(e);
                        }
                    }
                }
            }
            SignMessage::Verify => {
                if let (Some(file_path), Some(signature_path)) =
                    (&self.selected_file, &self.selected_signature)
                {
                    let verify = match rsa.verify_file_signature(file_path, signature_path) {
                        Ok(v) => v,
                        Err(_) => {
                            self.file_verified = Some(false);
                            return;
                        }
                    };
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
        let mut column = styled_column(None);

        if let Some(e) = &self.error {
            column = column.push(styled_error(e));
        }

        column = column
            .push(styled_row().push(load_file_button).push(load_signature))
            .push(styled_row().push(sign_button).push(verify_button));

        if let Some(hash) = self.file_verified.as_ref() {
            if *hash {
                column = column.push(text("Potpis valjan.").style(GREEN));
            } else {
                column = column.push(text("Potpis nije valjan.").style(RED));
            }
        }

        column.into()
    }
}
