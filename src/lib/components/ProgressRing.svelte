<script lang="ts">
	let { progress = 0, size = 240, strokeWidth = 8 }: { progress?: number; size?: number; strokeWidth?: number } = $props();

	const radius = $derived((size - strokeWidth) / 2);
	const circumference = $derived(2 * Math.PI * radius);
	const offset = $derived(circumference * (1 - Math.min(1, Math.max(0, progress))));
</script>

<svg
	width={size}
	height={size}
	class="transform -rotate-90"
	aria-hidden="true"
>
	<circle
		cx={size / 2}
		cy={size / 2}
		r={radius}
		fill="none"
		stroke="currentColor"
		stroke-width={strokeWidth}
		class="text-muted opacity-30"
	/>
	<circle
		cx={size / 2}
		cy={size / 2}
		r={radius}
		fill="none"
		stroke="currentColor"
		stroke-width={strokeWidth}
		stroke-linecap="round"
		stroke-dasharray={circumference}
		stroke-dashoffset={offset}
		class="text-primary transition-[stroke-dashoffset] duration-300 ease-out"
	/>
</svg>
