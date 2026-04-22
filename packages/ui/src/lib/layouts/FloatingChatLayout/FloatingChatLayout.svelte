<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		header?: Snippet;
		messages?: Snippet;
		input?: Snippet;
		class?: string;
	}

	let { header, messages, input, class: className = '' }: Props = $props();
</script>

<div class="bg-canvas flex h-screen w-full flex-col {className}">
	{#if header}
		<div class="border-border flex-shrink-0 border-b">
			{@render header()}
		</div>
	{/if}

	<div class="flex-1 overflow-y-auto">
		{#if messages}
			{@render messages()}
		{/if}
	</div>

	<div class="border-border flex-shrink-0 border-t">
		{#if input}
			{@render input()}
		{/if}
	</div>
</div>
