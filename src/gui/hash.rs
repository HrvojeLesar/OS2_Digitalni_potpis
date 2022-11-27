use iced::{
    widget::{self, text},
    Element,
};
use tinyfiledialogs::open_file_dialog;

use crate::encryption::ShaHash;

use super::{
    path_to_filename,
    styled_components::{styled_button, styled_column, styled_row},
};

#[derive(Debug, Clone, Copy)]
pub enum HashMessage {
    LoadFile,
    Hash,
}

pub struct HashView {
    selected_file: Option<String>,
    file_hash: Option<String>,
}

impl HashView {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            file_hash: None,
        }
    }

    pub fn reset(&mut self) {
        self.selected_file = None;
        self.file_hash = None;
    }

    pub fn update(&mut self, message: HashMessage) {
        match message {
            HashMessage::LoadFile => {
                self.selected_file = open_file_dialog("Odabir datoteke", "", None);
            }
            HashMessage::Hash => {
                if let Some(path) = &self.selected_file {
                    let hash = ShaHash::hash_file(path);
                    self.file_hash = Some(hex::encode(&hash));
                }
            }
        }
    }

    pub fn view(&self) -> Element<HashMessage> {
        let load_file_button = styled_button("Odabir datoteke").on_press(HashMessage::LoadFile);
        let hash_button = styled_button("Hash").on_press(HashMessage::Hash);

        let mut row = styled_row();

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

        row = row.push(hash_button);
        let mut column = styled_column(None).push(row);

        // let mut column = widget::column![row![load_file_button, hash_button]];

        if let Some(hash) = self.file_hash.as_ref() {
            column = column.push(text(hash));
        }

        column.into()
    }
}
