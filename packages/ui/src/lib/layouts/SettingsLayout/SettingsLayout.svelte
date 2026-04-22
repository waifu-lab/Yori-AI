<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		icon?: string;
		children: Snippet;
		class?: string;
	}

	let { title, icon, children, class: className = '' }: Props = $props();
</script>

<div class="flex w-full flex-col {className}">
	<div class="border-border mb-6 flex flex-col items-start gap-1 border-b px-4 pt-4 pb-4">
		<div class="flex items-center gap-2">
			{#if icon}
				<span class="{icon} text-muted-foreground text-xl"></span>
			{/if}
			<h1 class="text-foreground text-lg font-bold tracking-wide">
				{title}
			</h1>
		</div>
	</div>

	<div class="flex flex-col gap-3 px-4">
		{@render children()}
	</div>
</div>
