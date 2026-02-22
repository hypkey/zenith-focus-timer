use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter};
use tokio::time::{interval, Duration};

use crate::models::{SessionType, TimerState, TimerStatus};
use crate::notifications;
use crate::tray;

#[cfg(debug_assertions)]
const WORK_DURATION: u32 = 20;
#[cfg(debug_assertions)]
const SHORT_BREAK: u32 = 10;
#[cfg(debug_assertions)]
const LONG_BREAK: u32 = 15;

#[cfg(not(debug_assertions))]
const WORK_DURATION: u32 = 1500;
#[cfg(not(debug_assertions))]
const SHORT_BREAK: u32 = 300;
#[cfg(not(debug_assertions))]
const LONG_BREAK: u32 = 900;
const SESSIONS_BEFORE_LONG_BREAK: u32 = 4;

pub struct Timer {
    pub state: Arc<Mutex<TimerState>>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(TimerState {
                status: TimerStatus::Idle,
                session_type: SessionType::Work,
                remaining_secs: WORK_DURATION,
                total_secs: WORK_DURATION,
                sessions_completed: 0,
            })),
        }
    }

    pub fn start(&self, app: AppHandle) {
        let mut state = self.state.lock().unwrap();
        if state.status == TimerStatus::Running {
            return;
        }
        state.status = TimerStatus::Running;
        let snapshot = state.clone();
        drop(state);
        tray::update_tray_icon(&app, snapshot.status.clone(), snapshot.session_type.clone());
        self.spawn_tick_loop(app);
    }

    pub fn pause(&self, app: &AppHandle) {
        let mut state = self.state.lock().unwrap();
        if state.status == TimerStatus::Running {
            state.status = TimerStatus::Paused;
            let snapshot = state.clone();
            drop(state);
            tray::clear_tray_title(app);
            tray::update_tray_icon(app, snapshot.status.clone(), snapshot.session_type.clone());
            let _ = app.emit("timer-tick", &snapshot);
        }
    }

    pub fn resume(&self, app: AppHandle) {
        let mut state = self.state.lock().unwrap();
        if state.status != TimerStatus::Paused {
            return;
        }
        state.status = TimerStatus::Running;
        let snapshot = state.clone();
        drop(state);
        tray::update_tray_icon(&app, snapshot.status.clone(), snapshot.session_type.clone());
        self.spawn_tick_loop(app);
    }

    pub fn reset(&self, app: &AppHandle) {
        let mut state = self.state.lock().unwrap();
        state.status = TimerStatus::Idle;
        state.remaining_secs = state.total_secs;
        let snapshot = state.clone();
        drop(state);

        tray::clear_tray_title(app);
        tray::update_tray_icon(app, snapshot.status.clone(), snapshot.session_type.clone());
        let _ = app.emit("timer-tick", &snapshot);
    }

    pub fn skip(&self, app: &AppHandle) {
        let mut state = self.state.lock().unwrap();
        state.status = TimerStatus::Idle;

        match state.session_type {
            SessionType::Work => {
                state.session_type = SessionType::ShortBreak;
                state.remaining_secs = SHORT_BREAK;
                state.total_secs = SHORT_BREAK;
            }
            SessionType::ShortBreak | SessionType::LongBreak => {
                if state.session_type == SessionType::LongBreak {
                    state.sessions_completed = 0;
                }
                state.session_type = SessionType::Work;
                state.remaining_secs = WORK_DURATION;
                state.total_secs = WORK_DURATION;
            }
        }

        let snapshot = state.clone();
        drop(state);
        tray::clear_tray_title(app);
        tray::update_tray_icon(app, snapshot.status.clone(), snapshot.session_type.clone());
        let _ = app.emit("timer-tick", &snapshot);
    }

    pub fn get_state(&self) -> TimerState {
        self.state.lock().unwrap().clone()
    }

    fn spawn_tick_loop(&self, app: AppHandle) {
        let state = Arc::clone(&self.state);
        tauri::async_runtime::spawn(async move {
            tick_loop(state, app).await;
        });
    }
}

async fn tick_loop(state: Arc<Mutex<TimerState>>, app: AppHandle) {
    let mut ticker = interval(Duration::from_secs(1));
    ticker.tick().await;

    loop {
        ticker.tick().await;

        let mut s = state.lock().unwrap();
        if s.status != TimerStatus::Running {
            drop(s);
            let app_clone = app.clone();
            let _ = app.run_on_main_thread(move || {
                tray::clear_tray_title(&app_clone);
            });
            break;
        }

        s.remaining_secs = s.remaining_secs.saturating_sub(1);

        if s.remaining_secs == 0 {
            s.status = TimerStatus::Idle;
            let completed_snapshot = s.clone();

            advance_session(&mut s);
            let next_snapshot = s.clone();
            drop(s);

            let app_clone = app.clone();
            let status = next_snapshot.status.clone();
            let session_type = next_snapshot.session_type.clone();
            let _ = app.run_on_main_thread(move || {
                tray::clear_tray_title(&app_clone);
                tray::update_tray_icon(&app_clone, status, session_type);
            });

            notifications::notify_session_complete(&app, &completed_snapshot);
            let _ = app.emit("session-complete", &completed_snapshot);
            let _ = app.emit("timer-tick", &next_snapshot);
            break;
        }

        let snapshot = s.clone();
        let remaining = snapshot.remaining_secs;
        drop(s);

        let app_clone = app.clone();
        let _ = app.run_on_main_thread(move || {
            tray::update_tray_title(&app_clone, remaining);
        });
        let _ = app.emit("timer-tick", &snapshot);
    }
}

fn advance_session(state: &mut TimerState) {
    match state.session_type {
        SessionType::Work => {
            state.sessions_completed += 1;
            if state.sessions_completed >= SESSIONS_BEFORE_LONG_BREAK {
                state.session_type = SessionType::LongBreak;
                state.remaining_secs = LONG_BREAK;
                state.total_secs = LONG_BREAK;
                state.sessions_completed = 0;
            } else {
                state.session_type = SessionType::ShortBreak;
                state.remaining_secs = SHORT_BREAK;
                state.total_secs = SHORT_BREAK;
            }
        }
        SessionType::ShortBreak | SessionType::LongBreak => {
            state.session_type = SessionType::Work;
            state.remaining_secs = WORK_DURATION;
            state.total_secs = WORK_DURATION;
        }
    }
}
