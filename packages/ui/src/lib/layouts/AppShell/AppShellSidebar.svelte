<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className = '' }: Props = $props();
</script>

<aside class="bg-surface flex h-full flex-shrink-0 flex-col overflow-x-hidden overflow-y-auto {className}">
	{@render children()}
</aside>
