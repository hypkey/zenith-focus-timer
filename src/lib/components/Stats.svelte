<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import type { DailyStats, WeeklyStats } from "$lib/types";

	let dailyStats = $state<DailyStats | null>(null);
	let weeklyStats = $state<WeeklyStats | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	function formatDuration(secs: number): string {
		const h = Math.floor(secs / 3600);
		const m = Math.floor((secs % 3600) / 60);
		if (h > 0) return `${h}h ${m}m`;
		return `${m}m`;
	}

	function getTodayDate(): string {
		const d = new Date();
		const y = d.getFullYear();
		const m = String(d.getMonth() + 1).padStart(2, "0");
		const day = String(d.getDate()).padStart(2, "0");
		return `${y}-${m}-${day}`;
	}

	function getWeekStart(): string {
		const d = new Date();
		const day = d.getDay();
		const diff = d.getDate() - day + (day === 0 ? -6 : 1);
		const monday = new Date(d.getFullYear(), d.getMonth(), diff);
		const y = monday.getFullYear();
		const m = String(monday.getMonth() + 1).padStart(2, "0");
		const dayNum = String(monday.getDate()).padStart(2, "0");
		return `${y}-${m}-${dayNum}`;
	}

	async function loadStats() {
		loading = true;
		error = null;
		try {
			const [daily, weekly] = await Promise.all([
				invoke<DailyStats>("get_daily_stats", { date: getTodayDate() }),
				invoke<WeeklyStats>("get_weekly_stats", { weekStart: getWeekStart() }),
			]);
			dailyStats = daily;
			weeklyStats = weekly;
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	$effect(() => {
		loadStats();
	});
</script>

<div class="flex flex-col gap-8 py-8 px-6">
	{#if loading}
		<p class="text-center text-muted-foreground">Loading stats...</p>
	{:else if error}
		<p class="text-center text-destructive">{error}</p>
	{:else}
		<section class="space-y-2">
			<h2 class="text-lg font-medium text-foreground">Today</h2>
			<p class="text-2xl font-semibold text-primary">
				{formatDuration(dailyStats?.total_focus_secs ?? 0)} focused
			</p>
			<p class="text-sm text-muted-foreground">
				{dailyStats?.session_count ?? 0} sessions completed
			</p>
		</section>

		<section class="space-y-2">
			<h2 class="text-lg font-medium text-foreground">This Week</h2>
			<p class="text-2xl font-semibold text-primary">
				{formatDuration(weeklyStats?.total_focus_secs ?? 0)}
			</p>
			<p class="text-sm text-muted-foreground">
				Avg: {formatDuration(weeklyStats?.average_per_day ?? 0)} / day
			</p>
		</section>

		{#if (dailyStats?.sessions_by_tag?.length ?? 0) > 0}
			<section class="space-y-2">
				<h2 class="text-lg font-medium text-foreground">By Tag (Today)</h2>
				<ul class="space-y-1.5">
					{#each dailyStats?.sessions_by_tag ?? [] as tag}
						<li class="flex items-center gap-2">
							<span
								class="size-2.5 shrink-0 rounded-full"
								style="background-color: {tag.tag_color}"
							></span>
							<span class="text-foreground">{tag.tag_name}</span>
							<span class="ml-auto text-muted-foreground"
								>{formatDuration(tag.total_secs)}</span
							>
						</li>
					{/each}
				</ul>
			</section>
		{/if}
	{/if}
</div>
