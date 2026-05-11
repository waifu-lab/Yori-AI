<script lang="ts">
	import type { Snippet } from 'svelte';

	interface BubbleAttachment {
		id?: string;
		filename: string;
		content_type: string;
		url?: string;
		size_bytes?: number;
	}

	interface Props {
		content?: string;
		timestamp?: string;
		attachments?: BubbleAttachment[];
		children?: Snippet;
		class?: string;
	}

	let { content = '', timestamp, attachments = [], children, class: className = '' }: Props = $props();

	let showActions = $state(false);

	function formatSize(bytes: number) {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	}

	function isImage(a: BubbleAttachment) {
		return a.content_type.startsWith('image/');
	}
</script>

<div
	class="group ml-auto flex max-w-[80%] items-start justify-end gap-3 {className}"
	onmouseenter={() => (showActions = true)}
	onmouseleave={() => (showActions = false)}
	role="article"
>
	<div class="flex min-w-0 flex-col items-end gap-1">
		{#if attachments.length > 0}
			<div class="flex flex-wrap justify-end gap-2">
				{#each attachments as att, i (att.id ?? i)}
					{#if isImage(att) && att.url}
						<a href={att.url} target="_blank" rel="noopener noreferrer" class="block">
							<img
								src={att.url}
								alt={att.filename}
								class="max-h-60 max-w-[240px] rounded-lg object-cover"
								loading="lazy"
							/>
						</a>
					{:else}
						<a
							href={att.url}
							target="_blank"
							rel="noopener noreferrer"
							download={att.filename}
							class="bg-elevated text-foreground hover:bg-surface duration-fast flex items-center gap-2 rounded-md px-2.5 py-1.5 text-xs transition-colors"
							title={att.filename}
						>
							<svg
								class="text-muted-foreground h-3.5 w-3.5 flex-shrink-0"
								viewBox="0 0 16 16"
								fill="none"
								stroke="currentColor"
								stroke-width="1.5"
							>
								<path d="M9 1.5H3.5v13H12.5V5L9 1.5z" stroke-linejoin="round" />
								<path d="M9 1.5V5h3.5" stroke-linejoin="round" />
							</svg>
							<span class="max-w-[160px] truncate">{att.filename}</span>
							{#if att.size_bytes != null}
								<span class="text-faint">{formatSize(att.size_bytes)}</span>
							{/if}
						</a>
					{/if}
				{/each}
			</div>
		{/if}

		{#if content || children}
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
		{/if}

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
