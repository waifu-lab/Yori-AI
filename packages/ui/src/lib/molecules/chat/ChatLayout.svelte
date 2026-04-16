<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		messages?: Snippet;
		input?: Snippet;
		header?: Snippet;
		empty?: Snippet;
		hasMessages?: boolean;
		class?: string;
	}

	let { messages, input, header, empty, hasMessages = true, class: className = '' }: Props = $props();

	let scrollEl: HTMLDivElement | undefined = $state();

	export function scrollToBottom(smooth = true) {
		if (!scrollEl) return;
		scrollEl.scrollTo({
			top: scrollEl.scrollHeight,
			behavior: smooth ? 'smooth' : 'instant'
		});
	}
</script>

<div class="bg-canvas flex h-full flex-col {className}">
	{#if header}
		<div class="bg-canvas flex-shrink-0 px-6 py-4">
			{@render header()}
		</div>
	{/if}

	<div bind:this={scrollEl} class="flex-1 space-y-6 overflow-y-auto px-6 py-4">
		{#if !hasMessages && empty}
			<div class="flex h-full items-center justify-center">
				{@render empty()}
			</div>
		{:else if messages}
			{@render messages()}
		{/if}
	</div>

	<div class="flex-shrink-0">
		{@render input?.()}
	</div>
</div>
