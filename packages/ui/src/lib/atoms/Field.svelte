<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		label?: string;
		hint?: string;
		error?: string;
		required?: boolean;
		id?: string;
		class?: string;
		children: Snippet;
	}

	let { label, hint, error, required = false, id, class: className = '', children }: Props = $props();
</script>

<div class="flex flex-col gap-1.5 {className}">
	{#if label}
		<label for={id} class="text-foreground font-[Inter] text-xs font-medium select-none">
			{label}
			{#if required}<span class="text-danger ml-0.5">*</span>{/if}
		</label>
	{/if}

	{@render children()}

	{#if error}
		<p class="text-danger text-xs leading-snug">{error}</p>
	{:else if hint}
		<p class="text-muted-foreground text-xs leading-snug">{hint}</p>
	{/if}
</div>
