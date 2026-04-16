<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title: string;
		description?: string;
		children: Snippet;
		class?: string;
	}

	let { title, description, children, class: className = '' }: Props = $props();
</script>

<div class="mx-auto flex max-w-3xl flex-col gap-8 px-6 py-8 {className}">
	<div class="flex flex-col gap-1">
		<h1 class="text-foreground text-[32px] leading-tight font-extrabold tracking-[-0.05em]">
			{title}
		</h1>
		{#if description}
			<p class="text-muted-foreground max-w-lg text-sm leading-relaxed">{description}</p>
		{/if}
	</div>

	<div class="flex flex-col gap-3">
		{@render children()}
	</div>
</div>
