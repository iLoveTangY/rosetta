// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use dirs::config_dir;
use errors::RosettaError;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use shortcuts::register_shortcuts;
use tauri::{Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};
use translate_service::{baidu_translator, TranslateResults};

mod errors;
mod shortcuts;
mod translate_service;

#[derive(Serialize, Deserialize)]
struct TranslateConfig {
    from: String,
    to: String,
}
pub struct StoreState(Arc<Store<Wry>>);

#[tauri::command]
async fn translate(
    app_handle: tauri::AppHandle,
    text: String,
    config: TranslateConfig,
) -> Result<TranslateResults, RosettaError> {
    println!("Hello, {}! You've been greeted from Rust!", text);
    let ret = baidu_translator::translate(app_handle, &text, &config.from, &config.to).await?;
    Ok(ret)
}

#[tauri::command]
async fn set_config(app_handle: tauri::AppHandle, key: String, value: serde_json::Value) {
    println!("set_config called, key = {:?}, value = {:?}", key, value);
    let state = app_handle.state::<StoreState>();
    let store = state.0.clone();
    store.set(key, value);
}

#[tauri::command]
async fn get_config(
    app_handle: tauri::AppHandle,
    key: String,
) -> Result<serde_json::Value, RosettaError> {
    println!("get_config called, key = {:?}", key);
    let state = app_handle.state::<StoreState>();
    let store = state.0.clone();
    let value = store.get(&key).ok_or(RosettaError::ConfigError(format!("Empty config for key {}", key)))?;
    Ok(value)
}

pub static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();

fn init_config(app: &mut tauri::App) {
    let config_path = config_dir().unwrap();
    let config_path = config_path.join(app.config().identifier.clone());
    let config_path = config_path.join("config.json");
    println!("Load config from: {:?}", config_path);
    let store = StoreBuilder::new(app.handle(), config_path)
        .auto_save(std::time::Duration::from_millis(100))
        .build()
        .expect("Terminated due to config create failed!!");
    app.manage(StoreState(store));
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![translate, set_config, get_config])
        .setup(|app| {
            println!("in setup");
            init_config(app);
            APP_HANDLE.get_or_init(|| app.handle().clone());
            register_shortcuts(&app.app_handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
