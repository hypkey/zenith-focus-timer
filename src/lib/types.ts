export type TimerStatus = "idle" | "running" | "paused";

export type SessionType = "work" | "short_break" | "long_break";

export interface TimerState {
  status: TimerStatus;
  session_type: SessionType;
  remaining_secs: number;
  total_secs: number;
  sessions_completed: number;
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
