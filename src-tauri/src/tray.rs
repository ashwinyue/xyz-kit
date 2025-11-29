use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};
use crate::commands::AppState;

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
    
    // Check if id-join is enabled
    let state = app.state::<AppState>();
    let is_enabled = state.store.lock().unwrap().is_function_enabled("id-join");
    
    let id_join_toggle = CheckMenuItem::with_id(
        app,
        "toggle-id-join",
        "ID拼接功能",
        true,
        is_enabled,
        None::<&str>,
    )?;
    
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &separator, &id_join_toggle, &separator, &quit])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "toggle-id-join" => {
                let state = app.state::<AppState>();
                let mut store = state.store.lock().unwrap();
                let _enabled = store.toggle_function("id-join".to_string());
                let _ = store.save();
                
                // Notify frontend to refresh
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("functions-updated", ());
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
