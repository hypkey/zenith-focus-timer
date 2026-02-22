import { invoke } from "@tauri-apps/api/core";
import type { Tag } from "$lib/types";

let tags = $state<Tag[]>([]);

export const tagsStore = {
	get tags() {
		return tags;
	},
	async refresh() {
		try {
			tags = await invoke<Tag[]>("get_tags");
		} catch {
			tags = [];
		}
	},
};
