use selection::get_text;
use tauri::AppHandle;
use tauri::GlobalShortcutManager;
use tauri::Manager;
use tauri::WebviewWindow;
use tauri::WindowBuilder;
use tauri_plugin_global_shortcut::Code;
use tauri_plugin_global_shortcut::Modifiers;
use tauri_plugin_global_shortcut::Shortcut;

use crate::errors::RosettaError;
use crate::APP_HANDLE;

pub static TRANSLATE_WINDOW_LABEL: &str = "translate_window";

pub fn get_translate_window() -> WebviewWindow {
    let app_handle = APP_HANDLE.get().unwrap();
    let translate_window;
    match app_handle.get_webview_window(TRANSLATE_WINDOW_LABEL) {
        Some(v) => {
            println!("Window existence: {}", TRANSLATE_WINDOW_LABEL);
            translate_window = v;
        }
        None => {
            println!("Window not exist, creating new window");
            translate_window = WindowBuilder::new(
                app_handle,
                TRANSLATE_WINDOW_LABEL,
                tauri::WindowUrl::App("index.html".into()),
            )
            .decorations(true) // TODO: set to false
            // .focused(true)
            .visible(false)
            // .transparent(true)
            .build()
            .unwrap();
        }
    }

    translate_window
        .set_size(tauri::LogicalSize::new(350, 450))
        .unwrap();
    translate_window
        .set_skip_taskbar(true)
        .expect("cannot skip taskbar");
    translate_window
}

fn handle_selection() {
    let text = get_text();
    println!("selection: {}", text);
    let translate_window = get_translate_window();
    println!("emit new_text to frontend, text: {}", text);
    translate_window.emit("new_text", text).unwrap();
}

pub fn register_shortcuts(app_handle: &AppHandle) -> Result<(), RosettaError> {
    let alt_d_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyD);
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, _e| {
                println!("{:?}", shortcut);
                if shortcut == &alt_d_shortcut {
                    println!("Alt-D Detected!");
                    let text = get_text();
                    println!("selection text: {}", text);
                    get_translate_window();
                }
            })
            .build(),
    )?;
    Ok(())
}
