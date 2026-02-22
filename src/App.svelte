<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { Tabs, TabsContent, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
	import Timer from "$lib/components/Timer.svelte";
	import type { AppSettings } from "$lib/types";

	let activeTab = $state("timer");
	let showStats = $state(false);

	$effect(() => {
		invoke<AppSettings>("get_settings").then((s) => {
			showStats = s.show_stats;
		});
	});
</script>

<main class="flex flex-col h-screen bg-background">
	<header class="shrink-0 px-4 pt-4 pb-2">
		<h1 class="text-xl font-semibold text-foreground tracking-tight">Zenith</h1>
	</header>

	<Tabs bind:value={activeTab} class="flex flex-1 flex-col min-h-0 px-4">
		<TabsList class="w-full">
			<TabsTrigger value="timer">Timer</TabsTrigger>
			{#if showStats}
				<TabsTrigger value="stats">Stats</TabsTrigger>
			{/if}
			<TabsTrigger value="settings">Settings</TabsTrigger>
		</TabsList>

		<TabsContent value="timer" class="flex-1 min-h-0 mt-0 overflow-auto">
			<Timer />
		</TabsContent>

		{#if showStats}
			<TabsContent value="stats" class="flex-1 min-h-0 mt-0 overflow-auto">
				<div class="py-8 text-center text-muted-foreground">Stats coming in Phase 3</div>
			</TabsContent>
		{/if}

		<TabsContent value="settings" class="flex-1 min-h-0 mt-0 overflow-auto">
			<div class="py-8 text-center text-muted-foreground">Settings coming in Phase 4</div>
		</TabsContent>
	</Tabs>
</main>
