<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { tagsStore } from "$lib/stores/tags.svelte";
	import type { Session, SessionType } from "$lib/types";

	const PAGE_SIZE = 20;

	let sessions = $state<Session[]>([]);
	let offset = $state(0);
	let loading = $state(false);
	let hasMore = $state(true);

	const tags = $derived(tagsStore.tags);

	function formatDate(iso: string): string {
		const d = new Date(iso);
		const now = new Date();
		const today = now.toDateString();
		const sessionDate = d.toDateString();
		if (sessionDate === today) {
			return `Today ${d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`;
		}
		const yesterday = new Date(now);
		yesterday.setDate(yesterday.getDate() - 1);
		if (sessionDate === yesterday.toDateString()) {
			return `Yesterday ${d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}`;
		}
		return d.toLocaleDateString([], {
			month: "short",
			day: "numeric",
			hour: "2-digit",
			minute: "2-digit",
		});
	}

	function formatDuration(secs: number): string {
		const m = Math.floor(secs / 60);
		const s = secs % 60;
		if (m > 0) return `${m}m ${s}s`;
		return `${s}s`;
	}

	function sessionTypeLabel(t: SessionType): string {
		switch (t) {
			case "work":
				return "Focus";
			case "short_break":
				return "Short break";
			case "long_break":
				return "Long break";
		}
	}

	function getTagForSession(session: Session) {
		if (session.tag_id == null) return null;
		return tags.find((t) => t.id === session.tag_id);
	}

	async function loadMore() {
		if (loading || !hasMore) return;
		loading = true;
		try {
			const batch = await invoke<Session[]>("get_session_history", {
				limit: PAGE_SIZE,
				offset,
				tagId: null,
			});
			sessions = [...sessions, ...batch];
			offset += batch.length;
			hasMore = batch.length === PAGE_SIZE;
		} catch {
			hasMore = false;
		} finally {
			loading = false;
		}
	}

	let initialized = $state(false);
	$effect(() => {
		if (initialized) return;
		initialized = true;
		sessions = [];
		offset = 0;
		hasMore = true;
		loadMore();
	});
</script>

<div class="flex flex-col gap-4">
	<h2 class="text-lg font-medium text-foreground">Session History</h2>
	<div class="max-h-64 overflow-y-auto rounded-md border border-input">
		{#if sessions.length === 0 && !loading}
			<p class="py-8 text-center text-sm text-muted-foreground">No sessions yet</p>
		{:else}
			<ul class="divide-y divide-border">
				{#each sessions as session}
					<li class="flex items-center gap-3 px-4 py-3 text-sm">
						<span class=" shrink-0 text-muted-foreground">{formatDate(session.start_time)}</span>
						<span class="w-20 shrink-0">{formatDuration(session.duration_s)}</span>
						<span
							class="shrink-0 {session.session_type === 'work'
								? 'text-primary'
								: 'text-muted-foreground'}"
						>
							{sessionTypeLabel(session.session_type)}
						</span>
						{#if getTagForSession(session)}
							<span
								class="inline-flex items-center gap-1.5 shrink-0"
								style="color: {getTagForSession(session)!.color}"
							>
								<span
									class="size-2 rounded-full"
									style="background-color: {getTagForSession(session)!.color}"
								></span>
								{getTagForSession(session)!.name}
							</span>
						{/if}
						<span class="ml-auto shrink-0">
							{session.completed ? "Completed" : "Skipped"}
						</span>
					</li>
				{/each}
			</ul>
			{#if hasMore}
				<div class="flex justify-center py-3">
					<button
						type="button"
						class="text-sm text-muted-foreground hover:text-foreground"
						onclick={loadMore}
						disabled={loading}
					>
						{loading ? "Loading..." : "Load more"}
					</button>
				</div>
			{/if}
		{/if}
	</div>
</div>
