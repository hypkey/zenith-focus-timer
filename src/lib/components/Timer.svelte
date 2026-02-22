<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import ProgressRing from "$lib/components/ProgressRing.svelte";
	import { timer, initTimerListener, destroyTimerListener } from "$lib/stores/timer.svelte";

	$effect(() => {
		initTimerListener();
		return () => destroyTimerListener();
	});

	const sessionLabelClass = $derived(
		timer.sessionType === "work" ? "text-primary" : "text-accent-secondary"
	);
</script>

<div class="flex flex-col items-center gap-8 py-8 px-6">
	<h2 class={`text-lg font-medium ${sessionLabelClass}`}>{timer.sessionLabel}</h2>

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

	<div class="flex flex-wrap gap-3 justify-center">
		{#if timer.isIdle}
			<Button onclick={() => timer.start()}>Start</Button>
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
