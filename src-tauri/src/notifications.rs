use tauri::Manager;
use tauri_plugin_notification::NotificationExt;

use crate::audio;
use crate::models::{SessionType, TimerState};
use crate::settings::AppSettings;

pub fn notify_session_complete(app: &tauri::AppHandle, completed: &TimerState) {
    let play_sound = app
        .try_state::<std::sync::Mutex<AppSettings>>()
        .map(|s| s.lock().unwrap().notification_sound_enabled)
        .unwrap_or(true);

    if play_sound {
        match completed.session_type {
            SessionType::Work => audio::play_work_complete_sound(),
            SessionType::ShortBreak | SessionType::LongBreak => audio::play_break_complete_sound(),
        }
    }
    let (title, body) = match completed.session_type {
        SessionType::Work => {
            let n = completed.sessions_completed + 1;
            let sessions_text = if n == 1 {
                "1 session".to_string()
            } else {
                format!("{n} sessions")
            };
            (
                "Time for a break!".to_string(),
                format!("You've completed {sessions_text} this cycle."),
            )
        }
        SessionType::ShortBreak => ("Break's over.".to_string(), "Ready to focus?".to_string()),
        SessionType::LongBreak => ("Great cycle!".to_string(), "Take your time getting back.".to_string()),
    };

    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(body)
        .show();
}
