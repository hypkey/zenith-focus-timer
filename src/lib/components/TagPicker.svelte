<script lang="ts">
	import { tagsStore } from "$lib/stores/tags.svelte";
	import type { Tag } from "$lib/types";

	interface Props {
		value: number | null;
		onChange: (tagId: number | null) => void;
		disabled?: boolean;
	}
	let { value, onChange, disabled = false }: Props = $props();

	const tags = $derived(tagsStore.tags);
	const selectedTag = $derived(tags.find((t) => t.id === value));

	$effect(() => {
		tagsStore.refresh();
	});

	function handleSelect(e: Event) {
		const target = e.target as HTMLSelectElement;
		const v = target.value;
		onChange(v === "" ? null : parseInt(v, 10));
	}
</script>

<div class="flex items-center gap-2">
	{#if selectedTag}
		<span
			class="size-2.5 shrink-0 rounded-full"
			style="background-color: {selectedTag.color}"
			aria-hidden="true"
		></span>
	{/if}
	<select
		value={value ?? ""}
		onchange={handleSelect}
		disabled={disabled}
		class="h-9 rounded-md border border-input bg-background px-3 py-1 text-sm text-foreground disabled:opacity-50"
		aria-label="Select tag for this session"
	>
		<option value="">No tag</option>
		{#each tags as tag}
			<option value={tag.id}>{tag.name}</option>
		{/each}
	</select>
</div>
