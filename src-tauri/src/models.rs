use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub status: TimerStatus,
    pub session_type: SessionType,
    pub remaining_secs: u32,
    pub total_secs: u32,
    pub sessions_completed: u32,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub tag_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub start_time: String,
    pub end_time: Option<String>,
    pub duration_s: u32,
    pub session_type: SessionType,
    pub tag_id: Option<i64>,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSummary {
    pub tag_name: String,
    pub tag_color: String,
    pub total_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub total_focus_secs: u32,
    pub session_count: u32,
    pub sessions_by_tag: Vec<TagSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySummary {
    pub date: String,
    pub total_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyStats {
    pub total_focus_secs: u32,
    pub daily_breakdown: Vec<DaySummary>,
    pub average_per_day: u32,
}
