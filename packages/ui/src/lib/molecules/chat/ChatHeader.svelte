<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		class?: string;
	}

	let { children, class: className = '' }: Props = $props();
</script>

<div class="bg-canvas flex-shrink-0 px-6 py-4 {className}">
	{@render children()}
</div>
