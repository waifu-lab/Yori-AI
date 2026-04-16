<script lang="ts">
	import type { Snippet } from 'svelte';
	import StepIndicator from '../molecules/StepIndicator.svelte';

	interface Step {
		label: string;
		description?: string;
	}

	interface Props {
		steps: Step[];
		current?: number;
		title?: string;
		children: Snippet;
		footer?: Snippet;
		class?: string;
	}

	let { steps = [], current = 0, title, children, footer, class: className = '' }: Props = $props();
</script>

<div class="bg-canvas flex h-full flex-col {className}">
	<div class="flex-shrink-0 px-8 pt-8 pb-6">
		{#if title}
			<h1 class="text-foreground mb-6 text-[32px] font-extrabold tracking-[-0.05em]">{title}</h1>
		{/if}
		{#if steps.length}
			<StepIndicator {steps} {current} orientation="horizon" />
		{/if}
	</div>

	<div class="flex-1 overflow-y-auto px-8 py-4">
		{@render children()}
	</div>

	{#if footer}
		<div class="bg-canvas flex flex-shrink-0 items-center justify-between px-8 py-6">
			{@render footer()}
		</div>
	{/if}
</div>
