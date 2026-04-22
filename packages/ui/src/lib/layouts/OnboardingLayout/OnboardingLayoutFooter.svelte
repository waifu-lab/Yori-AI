<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className = '' }: Props = $props();
</script>

<div class="bg-canvas flex flex-shrink-0 items-center justify-between px-8 py-6 {className}">
	{@render children()}
</div>
