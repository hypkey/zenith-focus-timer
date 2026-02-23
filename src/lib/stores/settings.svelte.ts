import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "$lib/types";

let settings = $state<AppSettings | null>(null);

export const settingsStore = {
	get settings() {
		return settings;
	},
	async refresh() {
		try {
			settings = await invoke<AppSettings>("get_settings");
		} catch {
			settings = null;
		}
	},
};
