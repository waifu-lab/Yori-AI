<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		content?: string;
		timestamp?: string;
		children?: Snippet;
		class?: string;
	}

	let { content = '', timestamp, children, class: className = '' }: Props = $props();

	let showActions = $state(false);
</script>

<div
	class="group ml-auto flex max-w-[80%] items-start justify-end gap-3 {className}"
	onmouseenter={() => (showActions = true)}
	onmouseleave={() => (showActions = false)}
	role="article"
>
	<div class="flex min-w-0 flex-col items-end gap-1">
		<div
			class="bg-primary-bloom text-foreground px-4 py-3 text-sm leading-relaxed"
			style="border-radius: 12px 12px 4px 12px;"
		>
			{#if children}
				{@render children()}
			{:else}
				<span>{content}</span>
			{/if}
		</div>

		<div
			class="duration-fast flex items-center gap-1 transition-all"
			class:opacity-100={showActions}
			class:opacity-0={!showActions}
			class:pointer-events-none={!showActions}
		>
			{#if timestamp}
				<span class="text-faint text-xs">{timestamp}</span>
			{/if}
			<button
				class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex items-center gap-1.5 rounded-sm px-2 py-1 text-xs transition-colors"
				title="Edit"
			>
				<svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M11 2l3 3-8 8H3v-3l8-8z" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
				Edit
			</button>
		</div>
	</div>
</div>
