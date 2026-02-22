<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { Button } from "$lib/components/ui/button";
	import { tagsStore } from "$lib/stores/tags.svelte";
	import type { AppSettings, Tag } from "$lib/types";

	interface Props {
		onSettingsChange?: () => void;
	}
	let { onSettingsChange }: Props = $props();

	let settings = $state<AppSettings | null>(null);
	let newTagName = $state("");
	let newTagColor = $state("#C67B5C");

	const tags = $derived(tagsStore.tags);

	$effect(() => {
		invoke<AppSettings>("get_settings").then((s) => (settings = s));
		tagsStore.refresh();
	});

	async function toggleShowStats() {
		if (!settings) return;
		const updated = { ...settings, show_stats: !settings.show_stats };
		await invoke("update_settings", { newSettings: updated });
		settings = updated;
		onSettingsChange?.();
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
