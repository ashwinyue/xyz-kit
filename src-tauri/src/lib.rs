use std::sync::Mutex;
use tauri::Manager;

mod commands;
mod processor;
mod shortcut;
mod store;
mod tray;

use commands::AppState;
use store::PreferenceStore;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .manage(AppState {
            store: Mutex::new(PreferenceStore::new()),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clipboard_text,
            commands::set_clipboard_text,
            commands::process_text,
            commands::get_functions,
            commands::get_skip_list,
            commands::set_skip_list,
            commands::get_enabled_functions,
            commands::is_function_enabled,
            commands::toggle_function,
            commands::hide_window,
            commands::show_window,
            commands::show_window_at_cursor,
        ])
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            tray::setup_tray(app.handle())?;
            shortcut::register_shortcuts(app.handle())?;
            
            // Hide window after initialization
            if let Some(window) = app.get_webview_window("main") {
                // Windows specific: remove window border/shadow
                #[cfg(target_os = "windows")]
                {
                    use window_vibrancy::apply_mica;
                    let _ = apply_mica(&window, None);
                }
                
                let _ = window.hide();
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Hide window instead of closing
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
