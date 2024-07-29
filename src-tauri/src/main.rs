// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::OnceCell;
use shortcuts::register_shortcuts;
use tauri::Manager;

mod errors;
mod shortcuts;
mod translate_service;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            APP_HANDLE.get_or_init(|| app.handle());
            register_shortcuts(&app.app_handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
