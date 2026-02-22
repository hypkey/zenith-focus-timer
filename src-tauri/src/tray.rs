use std::sync::OnceLock;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
    Manager,
};

use crate::models::TimerStatus;

static TRAY_ID: OnceLock<String> = OnceLock::new();

fn format_time(secs: u32) -> String {
    let mins = secs / 60;
    let s = secs % 60;
    format!("{mins:02}:{s:02}")
}

pub fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show_window = MenuItem::with_id(app, "show_window", "Show Window", true, None::<&str>)?;
    let hide_window = MenuItem::with_id(app, "hide_window", "Hide to Tray", true, None::<&str>)?;
    let start_pause = MenuItem::with_id(app, "start_pause", "Start/Pause", true, None::<&str>)?;
    let reset = MenuItem::with_id(app, "reset", "Reset", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;

    let menu = Menu::with_items(
        app,
        &[
            &show_window,
            &hide_window,
            &sep,
            &start_pause,
            &reset,
            &sep,
            &quit,
        ],
    )?;

    let icon = app.default_window_icon().expect("default icon").clone();
    let tray = tauri::tray::TrayIconBuilder::new()
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app: &tauri::AppHandle<tauri::Wry>, event: tauri::menu::MenuEvent| {
            let id = event.id.as_ref();
            if id == "show_window" {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            } else if id == "hide_window" {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.hide();
                }
            } else if id == "start_pause" {
                let timer = app.state::<crate::timer::Timer>();
                let state = timer.get_state();
                match state.status {
                    TimerStatus::Idle => {
                        timer.start(app.clone());
                    }
                    TimerStatus::Paused => {
                        timer.resume(app.clone());
                    }
                    TimerStatus::Running => {
                        timer.pause(&app);
                    }
                }
            } else if id == "reset" {
                let timer = app.state::<crate::timer::Timer>();
                timer.reset(&app);
            }
        })
        .on_tray_icon_event(move |tray: &tauri::tray::TrayIcon<tauri::Wry>, event: tauri::tray::TrayIconEvent| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(w) = tray.app_handle().get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
        })
        .build(app)?;

    let _ = TRAY_ID.set(tray.id().as_ref().to_string());

    Ok(())
}

pub fn update_tray_title(app: &tauri::AppHandle, remaining_secs: u32) {
    if let Some(id) = TRAY_ID.get() {
        if let Some(tray) = app.tray_by_id(id.as_str()) {
            let _ = tray.set_title(Some(format_time(remaining_secs)));
        }
    }
}

pub fn clear_tray_title(app: &tauri::AppHandle) {
    if let Some(id) = TRAY_ID.get() {
        if let Some(tray) = app.tray_by_id(id.as_str()) {
            let _ = tray.set_title(None::<&str>);
        }
    }
}
