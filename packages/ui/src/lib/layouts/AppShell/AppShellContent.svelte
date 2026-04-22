<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className = '' }: Props = $props();
</script>

<main class="bg-canvas flex min-w-0 flex-1 flex-col overflow-hidden {className}">
	{@render children()}
</main>
