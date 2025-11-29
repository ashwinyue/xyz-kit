use crate::processor::{ProcessFunc, TextProcessor};
use crate::store::PreferenceStore;
use arboard::Clipboard;
use std::sync::Mutex;
use tauri::{Emitter, State, Window};

pub struct AppState {
    pub store: Mutex<PreferenceStore>,
}

// Clipboard commands
#[tauri::command]
pub fn get_clipboard_text() -> Result<String, String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    let text = clipboard.get_text().unwrap_or_default();
    Ok(text.trim().to_string())
}

#[tauri::command]
pub fn set_clipboard_text(text: String) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())
}

// Text processing commands
#[tauri::command]
pub fn process_text(text: String, func_name: String) -> Result<String, String> {
    let lines: Vec<&str> = text.lines().collect();
    let result = match func_name.as_str() {
        "id-join" => TextProcessor::id_join(&text, &lines),
        "duckduckgo" => {
            TextProcessor::duckduckgo_search(&text)?;
            text // 返回原文本，因为已经打开浏览器
        },
        _ => return Err(format!("Unknown function: {}", func_name)),
    };

    // Auto-copy result to clipboard
    let _ = set_clipboard_text(result.clone());
    Ok(result)
}

#[tauri::command]
pub fn get_functions() -> Vec<ProcessFunc> {
    TextProcessor::get_functions()
}

// Preference commands
#[tauri::command]
pub fn get_skip_list(state: State<AppState>) -> Vec<String> {
    state.store.lock().unwrap().get_skip_list().clone()
}

#[tauri::command]
pub fn set_skip_list(list: Vec<String>, state: State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.set_skip_list(list);
    store.save()
}

#[tauri::command]
pub fn get_enabled_functions(state: State<AppState>) -> Vec<String> {
    state.store.lock().unwrap().get_enabled_functions().clone()
}

#[tauri::command]
pub fn is_function_enabled(func_name: String, state: State<AppState>) -> bool {
    state.store.lock().unwrap().is_function_enabled(&func_name)
}

#[tauri::command]
pub fn toggle_function(func_name: String, state: State<AppState>) -> Result<bool, String> {
    let mut store = state.store.lock().unwrap();
    let enabled = store.toggle_function(func_name);
    store.save()?;
    Ok(enabled)
}

// Window commands
#[tauri::command]
pub fn hide_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_window(window: Window) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_window_at_cursor(window: Window, x: i32, y: i32) -> Result<(), String> {
    let pos_x = (x - 180).max(0);
    let pos_y = (y - 100).max(0);
    window
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: pos_x,
            y: pos_y,
        }))
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    window.emit("window-shown", ()).map_err(|e| e.to_string())
}
