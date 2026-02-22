use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub work_duration_s: u32,
    pub short_break_s: u32,
    pub long_break_s: u32,
    pub auto_advance: bool,
    pub show_stats: bool,
    pub daily_goal_enabled: bool,
    pub daily_goal_target: u32,
    pub theme: String,
    pub global_shortcut: String,
    pub launch_at_login: bool,
    pub notification_sound_enabled: bool,
    pub tick_sound_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            work_duration_s: 1500,
            short_break_s: 300,
            long_break_s: 900,
            auto_advance: false,
            show_stats: false,
            daily_goal_enabled: false,
            daily_goal_target: 8,
            theme: "warm-dawn".to_string(),
            global_shortcut: "Cmd+Shift+F".to_string(),
            launch_at_login: false,
            notification_sound_enabled: true,
            tick_sound_enabled: false,
        }
    }
}

fn settings_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join(SETTINGS_FILE)
}

pub fn load_settings(app_data_dir: &PathBuf) -> AppSettings {
    let path = settings_path(app_data_dir);

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            log::warn!("Corrupt settings file, using defaults: {e}");
            let defaults = AppSettings::default();
            let _ = save_settings(app_data_dir, &defaults);
            defaults
        }),
        Err(_) => {
            log::info!("No settings file found, creating defaults");
            let defaults = AppSettings::default();
            let _ = save_settings(app_data_dir, &defaults);
            defaults
        }
    }
}

pub fn save_settings(app_data_dir: &PathBuf, settings: &AppSettings) -> Result<(), String> {
    fs::create_dir_all(app_data_dir).map_err(|e| format!("Failed to create app data dir: {e}"))?;

    let json =
        serde_json::to_string_pretty(settings).map_err(|e| format!("Failed to serialize: {e}"))?;

    fs::write(settings_path(app_data_dir), json)
        .map_err(|e| format!("Failed to write settings: {e}"))
}
