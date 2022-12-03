use std::path::Path;

use anyhow::{anyhow, Result};

pub mod encrypt_decrypt;
pub mod hash;
pub mod keygen;
pub mod navigation;
pub mod sign;
pub mod styled_components;

pub(super) fn path_to_filename(path: &str) -> Result<String> {
    if let Some(file_name) = Path::new(path).file_name() {
        Ok(file_name.to_string_lossy().to_string())
    } else {
        Err(anyhow!("Failed to convert path to filename!"))
    }
}
