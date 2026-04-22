<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className = '' }: Props = $props();

	let scrollEl: HTMLDivElement | undefined = $state();

	export function scrollToBottom(smooth = true) {
		if (!scrollEl) return;
		scrollEl.scrollTo({
			top: scrollEl.scrollHeight,
			behavior: smooth ? 'smooth' : 'instant'
		});
	}
</script>

<div bind:this={scrollEl} class="flex-1 space-y-6 overflow-y-auto px-6 py-4 {className}">
	{@render children()}
</div>
