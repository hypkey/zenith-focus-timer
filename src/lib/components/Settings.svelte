<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { Button } from "$lib/components/ui/button";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { tagsStore } from "$lib/stores/tags.svelte";
	import type { AppSettings, Tag } from "$lib/types";

	interface Props {
		onSettingsChange?: () => void;
	}
	let { onSettingsChange }: Props = $props();

	let newTagName = $state("");
	let newTagColor = $state("#C67B5C");

	const settings = $derived(settingsStore.settings);
	const tags = $derived(tagsStore.tags);

	$effect(() => {
		settingsStore.refresh();
		tagsStore.refresh();
	});

	async function saveAndNotify(updated: AppSettings) {
		await invoke("update_settings", { newSettings: updated });
		await settingsStore.refresh();
		onSettingsChange?.();
	}

	async function toggleShowStats() {
		if (!settings) return;
		await saveAndNotify({ ...settings, show_stats: !settings.show_stats });
	}

	async function toggleDailyGoal() {
		if (!settings) return;
		await saveAndNotify({ ...settings, daily_goal_enabled: !settings.daily_goal_enabled });
	}

	async function updateDailyGoalTarget(target: number) {
		if (!settings) return;
		await saveAndNotify({ ...settings, daily_goal_target: target });
	}

	async function createTag() {
		const name = newTagName.trim();
		if (!name) return;
		try {
			await invoke("create_tag", { name, color: newTagColor });
			await tagsStore.refresh();
			newTagName = "";
		} catch {
			/* ignore */
		}
	}

	async function deleteTag(id: number) {
		try {
			await invoke("delete_tag", { id });
			await tagsStore.refresh();
		} catch {
			/* ignore */
		}
	}
</script>

<div class="flex flex-col gap-6 py-8 px-6">
	<section class="space-y-4">
		<h2 class="text-lg font-medium text-foreground">Tracking</h2>
		{#if settings}
			<label class="flex items-center gap-3 cursor-pointer">
				<input
					type="checkbox"
					checked={settings.show_stats}
					onchange={toggleShowStats}
					class="size-4 rounded border-input"
				/>
				<span class="text-sm text-foreground">Show Stats Tab</span>
			</label>
			<label class="flex items-center gap-3 cursor-pointer">
				<input
					type="checkbox"
					checked={settings.daily_goal_enabled}
					onchange={toggleDailyGoal}
					class="size-4 rounded border-input"
				/>
				<span class="text-sm text-foreground">Enable Daily Goal</span>
			</label>
			{#if settings.daily_goal_enabled}
				<div class="flex items-center gap-2">
					<label for="goal-target" class="text-sm text-foreground">Target (sessions):</label>
					<input
						id="goal-target"
						type="number"
						min="1"
						max="20"
						value={settings.daily_goal_target}
						onchange={(e) => {
							const v = parseInt((e.target as HTMLInputElement).value, 10);
							if (v >= 1 && v <= 20) updateDailyGoalTarget(v);
						}}
						class="h-9 w-20 rounded-md border border-input bg-background px-2 text-sm"
					/>
				</div>
			{/if}
		{:else}
			<p class="text-sm text-muted-foreground">Loading...</p>
		{/if}
	</section>

	<section class="space-y-4">
		<h2 class="text-lg font-medium text-foreground">Tags</h2>
		<p class="text-sm text-muted-foreground">Create tags to categorize your focus sessions.</p>
		<div class="flex gap-2">
			<input
				type="text"
				bind:value={newTagName}
				placeholder="Tag name"
				class="h-9 flex-1 rounded-md border border-input bg-background px-3 text-sm"
				onkeydown={(e) => e.key === "Enter" && createTag()}
			/>
			<input
				type="color"
				bind:value={newTagColor}
				class="h-9 w-12 cursor-pointer rounded border border-input"
				aria-label="Tag color"
			/>
			<Button onclick={createTag} disabled={!newTagName.trim()}>Add</Button>
		</div>
		{#if tags.length > 0}
			<ul class="space-y-2">
				{#each tags as tag}
					<li class="flex items-center gap-2">
						<span
							class="size-3 rounded-full"
							style="background-color: {tag.color}"
						></span>
						<span class="text-sm text-foreground">{tag.name}</span>
						<Button
							variant="ghost"
							size="sm"
							class="ml-auto text-muted-foreground hover:text-destructive"
							onclick={() => deleteTag(tag.id)}
						>
							Delete
						</Button>
					</li>
				{/each}
			</ul>
		{/if}
	</section>
</div>
