<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		left?: Snippet;
		center?: Snippet;
		right?: Snippet;
		sticky?: boolean;
		class?: string;
	}

	let { left, center, right, sticky = false, class: className = '' }: Props = $props();
</script>

<div
	class="flex items-center justify-between gap-4 px-4 py-2
    {sticky ? 'bg-surface/90 sticky top-0 z-10 backdrop-blur-[20px]' : 'bg-surface'}
    {className}"
>
	<div class="flex min-w-0 items-center gap-2">
		{#if left}{@render left()}{/if}
	</div>

	{#if center}
		<div class="absolute left-1/2 flex -translate-x-1/2 items-center gap-2">
			{@render center()}
		</div>
	{/if}

	<div class="ml-auto flex items-center gap-2">
		{#if right}{@render right()}{/if}
	</div>
</div>
