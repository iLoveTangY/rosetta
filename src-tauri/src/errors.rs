use thiserror::Error;

#[derive(Error, Debug)]
pub enum RosettaError {
    #[error("Shortcut register error: {0}")]
    ShortcutError(#[from] tauri_runtime::Error)
}
