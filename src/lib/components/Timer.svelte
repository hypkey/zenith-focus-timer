<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/core";
	import { Button } from "$lib/components/ui/button";
	import ProgressRing from "$lib/components/ProgressRing.svelte";
	import TagPicker from "$lib/components/TagPicker.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";
	import { timer, initTimerListener, destroyTimerListener } from "$lib/stores/timer.svelte";

	let selectedTagId = $state<number | null>(null);
	let todayCount = $state(0);

	const settings = $derived(settingsStore.settings);
	const showDailyGoal = $derived(settings?.daily_goal_enabled ?? false);
	const goalTarget = $derived(settings?.daily_goal_target ?? 8);
	const goalProgress = $derived(Math.min(1, todayCount / Math.max(1, goalTarget)));
	const goalReached = $derived(todayCount >= goalTarget && goalTarget > 0);
	let celebrating = $state(false);
	let lastCelebratedDate = $state<string | null>(null);

	$effect(() => {
		initTimerListener();
		return () => destroyTimerListener();
	});

	$effect(() => {
		invoke<number>("get_today_session_count").then((c) => (todayCount = c));
		const unlisten = listen("session-complete", () => {
			invoke<number>("get_today_session_count").then((c) => (todayCount = c));
		});
		return () => {
			unlisten.then((fn) => fn());
		};
	});

	$effect(() => {
		if (!goalReached || celebrating) return;
		const today = new Date().toISOString().slice(0, 10);
		if (lastCelebratedDate === today) return;
		lastCelebratedDate = today;
		celebrating = true;
		const t = setTimeout(() => (celebrating = false), 1500);
		return () => clearTimeout(t);
	});

	const sessionLabelClass = $derived(
		timer.sessionType === "work" ? "text-primary" : "text-accent-secondary"
	);
</script>

<div class="flex flex-col items-center gap-8 py-8 px-6">
	<h2 class={`text-lg font-medium ${sessionLabelClass}`}>{timer.sessionLabel}</h2>

	{#if timer.isIdle}
		<div class="flex items-center gap-2">
			<span class="text-sm text-muted-foreground">Tag:</span>
			<TagPicker
				value={selectedTagId}
				onChange={(id) => (selectedTagId = id)}
				disabled={false}
			/>
		</div>
	{/if}

	<div class="relative flex items-center justify-center">
		<ProgressRing progress={timer.progress} size={240} strokeWidth={8} />
		<span
			class="absolute text-5xl font-semibold tabular-nums text-foreground"
			aria-live="polite"
		>
			{timer.formattedTime}
		</span>
	</div>

	<div class="flex gap-1.5" aria-label="Work sessions completed">
		{#each Array(4) as _, i}
			<span
				class="size-2 rounded-full transition-colors {i < timer.sessionsCompleted
					? 'bg-primary'
					: 'bg-muted'}"
			></span>
		{/each}
	</div>

	{#if showDailyGoal}
		<div
			class="relative flex items-center justify-center transition-shadow duration-500 {celebrating
				? 'animate-pulse'
				: ''}"
		>
			<div
				class="relative rounded-full transition-shadow {celebrating
					? 'shadow-[0_0_20px_var(--primary)]'
					: ''}"
			>
				<ProgressRing progress={goalProgress} size={64} strokeWidth={6} />
				<span
					class="absolute inset-0 flex items-center justify-center text-sm font-medium tabular-nums text-foreground"
					aria-label="Daily goal: {todayCount} of {goalTarget} sessions"
				>
					{todayCount}/{goalTarget}
				</span>
			</div>
			<span class="ml-2 text-sm text-muted-foreground">sessions today</span>
		</div>
	{/if}

	<div class="flex flex-wrap gap-3 justify-center">
		{#if timer.isIdle}
			<Button onclick={() => timer.start(selectedTagId)}>Start</Button>
			<Button variant="outline" onclick={() => timer.skip()}>Skip</Button>
		{:else if timer.isRunning}
			<Button variant="secondary" onclick={() => timer.pause()}>Pause</Button>
			<Button variant="outline" onclick={() => timer.skip()}>Skip</Button>
		{:else if timer.isPaused}
			<Button onclick={() => timer.resume()}>Resume</Button>
			<Button variant="outline" onclick={() => timer.reset()}>Reset</Button>
		{/if}
	</div>
</div>
