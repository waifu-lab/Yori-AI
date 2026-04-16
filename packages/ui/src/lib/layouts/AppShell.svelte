<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		sidebar: Snippet;
		children: Snippet;
		class?: string;
	}

	let { sidebar, children, class: className = '' }: Props = $props();
</script>

<div class="bg-canvas flex h-screen w-full overflow-hidden {className}">
	<aside class="bg-surface flex h-full flex-shrink-0 flex-col overflow-x-hidden overflow-y-auto">
		{@render sidebar()}
	</aside>

	<main class="bg-canvas flex min-w-0 flex-1 flex-col overflow-hidden">
		{@render children()}
	</main>
</div>
