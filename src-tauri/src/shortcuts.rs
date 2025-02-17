use selection::get_text;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::EventTarget;
use tauri::Manager;
use tauri::WebviewWindow;
use tauri::WebviewWindowBuilder;
use tauri_plugin_global_shortcut::Code;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_global_shortcut::Modifiers;
use tauri_plugin_global_shortcut::Shortcut;
use tauri_plugin_global_shortcut::ShortcutState;

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
            translate_window = WebviewWindowBuilder::new(
                app_handle,
                TRANSLATE_WINDOW_LABEL,
                tauri::WebviewUrl::App("index.html".into()),
            )
            .decorations(true) // TODO: set to false
            .focused(false)
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
    std::thread::sleep(std::time::Duration::from_millis(50));
    let text = get_text();
    println!("selection: {}", text);
    if text.is_empty() {
        println!("Selection text is empty");
        return;
    }
    let translate_window = get_translate_window();
    println!("emit new_text to frontend, text: {}", text);
    translate_window.app_handle()
        .emit_to(TRANSLATE_WINDOW_LABEL, "new_text", text)
        .unwrap();
}

pub fn register_shortcuts(app_handle: &AppHandle) -> Result<(), RosettaError> {
    // 先创建窗口，否则快捷键会失效
    get_translate_window();
    println!("call register shortcuts");
    let alt_d_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyD);
    app_handle.plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, e| {
                if e.state == ShortcutState::Pressed && shortcut == &alt_d_shortcut {
                    handle_selection();
                }
            })
            .build(),
    )?;
    app_handle.global_shortcut().register(alt_d_shortcut)?;
    Ok(())
}
