<script lang="ts">
	import { Tabs, TabsContent, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
	import Timer from "$lib/components/Timer.svelte";
	import Stats from "$lib/components/Stats.svelte";
	import Settings from "$lib/components/Settings.svelte";
	import { settingsStore } from "$lib/stores/settings.svelte";

	let activeTab = $state("timer");
	const showStats = $derived(settingsStore.settings?.show_stats ?? false);

	$effect(() => {
		settingsStore.refresh();
	});

	function refreshSettings() {
		settingsStore.refresh();
	}
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
				<Stats />
			</TabsContent>
		{/if}

		<TabsContent value="settings" class="flex-1 min-h-0 mt-0 overflow-auto">
			<Settings onSettingsChange={refreshSettings} />
		</TabsContent>
	</Tabs>
</main>
