use std::path::Path;

pub mod encrypt_decrypt;
pub mod hash;
pub mod keygen;
pub mod navigation;
pub mod sign;
pub mod styled_components;

pub(super) fn path_to_filename(path: &str) -> String {
    if let Some(file_name) = Path::new(path).file_name() {
        file_name.to_string_lossy().to_string()
    } else {
        "".to_owned()
    }
}
