<script lang="ts">
	import type { Snippet } from 'svelte';
	import MarkdownRenderer from '../MarkdownRenderer.svelte';

	interface Props {
		content?: string;
		isStreaming?: boolean;
		timestamp?: string;
		children?: Snippet;
		class?: string;
	}

	let { content = '', isStreaming = false, timestamp, children, class: className = '' }: Props = $props();

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
	<div class="flex min-w-0 flex-col gap-1">
		<div class="text-foreground relative text-sm leading-relaxed">
			{#if children}
				{@render children()}
			{:else}
				<MarkdownRenderer source={content} />
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
