// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use errors::RosettaError;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use shortcuts::register_shortcuts;
use tauri::Manager;
use translate_service::{baidu_translator, TranslateResults};

mod errors;
mod shortcuts;
mod translate_service;

#[derive(Serialize, Deserialize)]
struct TranslateConfig {
    from: String,
    to: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn translate(
    text: String,
    config: TranslateConfig,
) -> Result<TranslateResults, RosettaError> {
    println!("Hello, {}! You've been greeted from Rust!", text);
    let ret = baidu_translator::translate(&text, &config.from, &config.to).await?;
    Ok(ret)
}

pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![translate])
        .setup(|app| {
            println!("in setup");
            APP_HANDLE.get_or_init(|| app.handle().clone());
            register_shortcuts(&app.app_handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
