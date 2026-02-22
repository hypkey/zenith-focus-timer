export type TimerStatus = "idle" | "running" | "paused";

export type SessionType = "work" | "short_break" | "long_break";

export interface TimerState {
  status: TimerStatus;
  session_type: SessionType;
  remaining_secs: number;
  total_secs: number;
  sessions_completed: number;
  start_time?: string | null;
  tag_id?: number | null;
}

export interface AppSettings {
  work_duration_s: number;
  short_break_s: number;
  long_break_s: number;
  auto_advance: boolean;
  show_stats: boolean;
  daily_goal_enabled: boolean;
  daily_goal_target: number;
  theme: string;
  global_shortcut: string;
  launch_at_login: boolean;
  notification_sound_enabled: boolean;
  tick_sound_enabled: boolean;
}

export interface Session {
  id: number;
  start_time: string;
  end_time: string | null;
  duration_s: number;
  session_type: SessionType;
  tag_id: number | null;
  completed: boolean;
  created_at: string;
}

export interface Tag {
  id: number;
  name: string;
  color: string;
  created_at: string;
}

export interface TagSummary {
  tag_name: string;
  tag_color: string;
  total_secs: number;
}

export interface DailyStats {
  total_focus_secs: number;
  session_count: number;
  sessions_by_tag: TagSummary[];
}

export interface DaySummary {
  date: string;
  total_secs: number;
}

export interface WeeklyStats {
  total_focus_secs: number;
  daily_breakdown: DaySummary[];
  average_per_day: number;
}
