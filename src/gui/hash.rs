use iced::{
    widget::{self, button, row, text},
    Element,
};
use tinyfiledialogs::open_file_dialog;

use crate::encryption::ShaHash;

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
        let load_file_button = button("Odabir datoteke").on_press(HashMessage::LoadFile);
        let hash_button = button("Hash").on_press(HashMessage::Hash);

        let mut column = widget::column![row![load_file_button, hash_button]];

        if let Some(hash) = self.file_hash.as_ref() {
            column = column.push(text(hash));
        }

        column.into()
    }
}
