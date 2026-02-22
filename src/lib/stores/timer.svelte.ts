import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { TimerState, SessionType } from "$lib/types";

const DEFAULT_STATE: TimerState = {
  status: "idle",
  session_type: "work",
  remaining_secs: 1500,
  total_secs: 1500,
  sessions_completed: 0,
};

let timerState = $state<TimerState>({ ...DEFAULT_STATE });

export const timer = {
  get status() {
    return timerState.status;
  },
  get sessionType(): SessionType {
    return timerState.session_type;
  },
  get remainingSecs() {
    return timerState.remaining_secs;
  },
  get totalSecs() {
    return timerState.total_secs;
  },
  get sessionsCompleted() {
    return timerState.sessions_completed;
  },
  get isRunning() {
    return timerState.status === "running";
  },
  get isPaused() {
    return timerState.status === "paused";
  },
  get isIdle() {
    return timerState.status === "idle";
  },
  get progress() {
    if (timerState.total_secs === 0) return 0;
    return 1 - timerState.remaining_secs / timerState.total_secs;
  },
  get formattedTime() {
    const mins = Math.floor(timerState.remaining_secs / 60);
    const secs = timerState.remaining_secs % 60;
    return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  },
  get sessionLabel() {
    switch (timerState.session_type) {
      case "work":
        return "Focus";
      case "short_break":
        return "Short Break";
      case "long_break":
        return "Long Break";
    }
  },

  async start() {
    await invoke("start_timer");
  },
  async pause() {
    await invoke("pause_timer");
  },
  async resume() {
    await invoke("resume_timer");
  },
  async reset() {
    await invoke("reset_timer");
  },
  async skip() {
    await invoke("skip_session");
  },
  async fetchState() {
    timerState = await invoke<TimerState>("get_timer_state");
  },
};

let unlisten: UnlistenFn | null = null;

export async function initTimerListener() {
  if (unlisten) return;

  await timer.fetchState();

  unlisten = await listen<TimerState>("timer-tick", (event) => {
    timerState = event.payload;
  });
}

export function destroyTimerListener() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
}
