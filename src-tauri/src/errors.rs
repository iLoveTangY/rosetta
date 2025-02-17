use thiserror::Error;

#[derive(Error, Debug)]
pub enum RosettaError {
    #[error("Shortcut plugin build error: {0}")]
    ShortcutPluginError(#[from] tauri::Error),
    #[error("Shtorcut register error: {0}")]
    ShortcutRegisterError(#[from] tauri_plugin_global_shortcut::Error),
    #[error("Translate error: {0}")]
    HttpRequestError(#[from] tauri_plugin_http::reqwest::Error),
}

impl serde::Serialize for RosettaError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
