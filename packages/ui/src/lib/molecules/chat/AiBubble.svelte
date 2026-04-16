<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		content?: string;
		isStreaming?: boolean;
		timestamp?: string;
		avatar?: string;
		children?: Snippet;
		class?: string;
	}

	let { content = '', isStreaming = false, timestamp, avatar, children, class: className = '' }: Props = $props();

	let showActions = $state(false);

	function copyContent() {
		navigator.clipboard.writeText(content);
	}
</script>

<div
	class="group flex max-w-[80%] items-start gap-3 {className}"
	onmouseenter={() => (showActions = true)}
	onmouseleave={() => (showActions = false)}
	role="article"
>
	<div class="bg-primary-bloom mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-md">
		{#if avatar}
			<img src={avatar} alt="AI" class="h-full w-full rounded-md object-cover" />
		{:else}
			<svg class="text-primary h-4 w-4" viewBox="0 0 16 16" fill="currentColor">
				<path
					d="M8 2a6 6 0 1 0 0 12A6 6 0 0 0 8 2zm0 2a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm0 8.5c-2 0-3.75-1-4.75-2.5.75-1.25 2.5-2 4.75-2s4 .75 4.75 2c-1 1.5-2.75 2.5-4.75 2.5z"
				/>
			</svg>
		{/if}
	</div>

	<div class="flex min-w-0 flex-col gap-1">
		<div
			class="bg-surface text-foreground relative px-4 py-3 text-sm leading-relaxed"
			style="border-radius: 12px 12px 12px 4px;"
		>
			{#if children}
				{@render children()}
			{:else}
				<span>{content}</span>
			{/if}

			{#if isStreaming}
				<span class="bg-primary ml-0.5 inline-block h-4 w-0.5 animate-pulse align-middle"></span>
			{/if}
		</div>

		<div
			class="duration-fast flex items-center gap-1 transition-all"
			class:opacity-100={showActions}
			class:opacity-0={!showActions}
			class:pointer-events-none={!showActions}
		>
			<button
				onclick={copyContent}
				class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex items-center gap-1.5 rounded-sm px-2 py-1 text-xs transition-colors"
				title="Copy"
			>
				<svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
					<rect x="5" y="5" width="8" height="8" rx="1.5" />
					<path d="M11 5V3.5A1.5 1.5 0 0 0 9.5 2h-6A1.5 1.5 0 0 0 2 3.5v6A1.5 1.5 0 0 0 3.5 11H5" />
				</svg>
				Copy
			</button>
			<button
				class="text-muted-foreground hover:bg-elevated hover:text-foreground duration-fast flex items-center gap-1.5 rounded-sm px-2 py-1 text-xs transition-colors"
				title="Regenerate"
			>
				<svg class="h-3.5 w-3.5" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M13.5 8A5.5 5.5 0 1 1 8 2.5" stroke-linecap="round" />
					<path d="M8 1v3.5L10.5 3" stroke-linecap="round" stroke-linejoin="round" />
				</svg>
				Regenerate
			</button>
			{#if timestamp}
				<span class="text-faint ml-1 text-xs">{timestamp}</span>
			{/if}
		</div>
	</div>
</div>
