mod audio;
mod db;
mod models;
mod notifications;
mod settings;
mod timer;
mod tray;

use std::sync::Mutex;

use tauri::Manager;

use models::TimerState;
use settings::AppSettings;
use timer::Timer;

#[tauri::command]
fn start_timer(timer: tauri::State<'_, Timer>, app: tauri::AppHandle) {
    timer.start(app);
}

#[tauri::command]
fn pause_timer(timer: tauri::State<'_, Timer>, app: tauri::AppHandle) {
    timer.pause(&app);
}

#[tauri::command]
fn resume_timer(timer: tauri::State<'_, Timer>, app: tauri::AppHandle) {
    timer.resume(app);
}

#[tauri::command]
fn reset_timer(timer: tauri::State<'_, Timer>, app: tauri::AppHandle) {
    timer.reset(&app);
}

#[tauri::command]
fn skip_session(timer: tauri::State<'_, Timer>, app: tauri::AppHandle) {
    timer.skip(&app);
}

#[tauri::command]
fn get_timer_state(timer: tauri::State<'_, Timer>) -> TimerState {
    timer.get_state()
}

#[tauri::command]
fn get_settings(settings: tauri::State<'_, Mutex<AppSettings>>) -> AppSettings {
    settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(
    new_settings: AppSettings,
    settings: tauri::State<'_, Mutex<AppSettings>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    settings::save_settings(&app_data_dir, &new_settings)?;
    *settings.lock().unwrap() = new_settings;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let loaded_settings = settings::load_settings(&app_data_dir);

            db::init_db(&app_data_dir)?;

            app.manage(Timer::new());
            app.manage(Mutex::new(loaded_settings));

            tray::setup_tray(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            skip_session,
            get_timer_state,
            get_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
