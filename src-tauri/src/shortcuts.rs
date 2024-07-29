
use selection::get_text;
use tauri::AppHandle;
use tauri::GlobalShortcutManager;
use tauri::Manager;
use tauri::WindowBuilder;

use crate::errors::RosettaError;
use crate::APP_HANDLE;


fn handle_selection() {
    let label = "translate_window";
    let app_handle = APP_HANDLE.get().unwrap();
    let text = get_text();
    println!("selection: {}", text);
    let translate_window;
    match app_handle.get_window(&label) {
        Some(v) => {
            println!("Window existence: {}", label);
            translate_window = v;
        }
        None => {
            println!("Window not exist, creating new window");
            translate_window = WindowBuilder::new(
                app_handle,
                label,
                tauri::WindowUrl::App("index.html".into()),
            )
            .decorations(false)
            .focused(true)
            .visible(true)
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
    println!("emit new_text to frontend, text: {}", text);
    translate_window.emit("new_text", text).unwrap();
}

pub fn register_shortcuts(app_handle: &AppHandle) -> Result<(), RosettaError> {
    let mut shortcut_manager = app_handle.global_shortcut_manager();
    shortcut_manager.register("Alt+D", handle_selection)?;
    println!("In setup function!!!");
    let text = get_text();
    println!("selection text: {}", text);
    Ok(())
}