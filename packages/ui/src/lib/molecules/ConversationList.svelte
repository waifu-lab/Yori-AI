<script lang="ts">
	interface Conversation {
		id: string;
		title: string;
		preview?: string;
	}

	interface Props {
		conversations?: Conversation[];
		activeId?: string;
		onSelect?: (id: string) => void;
		onNew?: () => void;
		onSettings?: () => void;
		class?: string;
	}

	let { conversations = [], activeId = '', onSelect, onNew, onSettings, class: className = '' }: Props = $props();
</script>

<div class="flex h-full flex-col {className}">
	<div class="flex items-center justify-between px-3 py-2">
		<span class="text-muted-foreground text-xs font-medium">對話</span>
		<button
			type="button"
			onclick={onNew}
			class="text-muted-foreground hover:bg-elevated hover:text-foreground rounded-md p-1 transition-colors"
			title="新增對話"
		>
			<svg class="h-4 w-4" viewBox="0 0 16 16" fill="none">
				<path d="M8 3v10M3 8h10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
			</svg>
		</button>
	</div>

	<div class="flex-1 space-y-0.5 overflow-y-auto p-2">
		{#each conversations as conv (conv.id)}
			<button
				type="button"
				onclick={() => onSelect?.(conv.id)}
				class="group duration-fast flex w-full flex-col gap-0.5 rounded-md px-3 py-2.5 text-left
					transition-colors
					{activeId === conv.id
					? 'bg-elevated text-foreground'
					: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
			>
				<span class="truncate text-sm font-medium">{conv.title}</span>
				{#if conv.preview}
					<span class="text-faint truncate text-xs">{conv.preview}</span>
				{/if}
			</button>
		{/each}

		{#if conversations.length === 0}
			<div class="text-faint flex h-24 items-center justify-center text-sm">還沒有對話</div>
		{/if}
	</div>

	<div class="border-border border-t p-2">
		<button
			type="button"
			onclick={onSettings}
			class="text-muted-foreground hover:bg-elevated hover:text-foreground flex w-full items-center gap-2 rounded-md px-3 py-2 text-sm transition-colors"
			title="設定"
		>
			<svg class="h-4 w-4 shrink-0" viewBox="0 0 16 16" fill="none">
				<path
					d="M8 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"
					stroke="currentColor"
					stroke-width="1.2"
				/>
				<path
					d="M13.3 6.6a1 1 0 0 0 .2-1.1l-.9-1.5a1 1 0 0 0-1-.5l-1 .2a5 5 0 0 0-.8-.5l-.2-1A1 1 0 0 0 8.6 2h-1.2a1 1 0 0 0-1 .7l-.2 1a5 5 0 0 0-.8.5l-1-.2a1 1 0 0 0-1 .5L2.5 6a1 1 0 0 0 .2 1.1l.7.7a4.6 4.6 0 0 0 0 .4v.4l-.7.7a1 1 0 0 0-.2 1.1l.9 1.5a1 1 0 0 0 1 .5l1-.2c.2.2.5.3.8.5l.2 1a1 1 0 0 0 1 .7h1.2a1 1 0 0 0 1-.7l.2-1a5 5 0 0 0 .8-.5l1 .2a1 1 0 0 0 1-.5l.9-1.5a1 1 0 0 0-.2-1.1l-.7-.7v-.8l.7-.7Z"
					stroke="currentColor"
					stroke-width="1.2"
					stroke-linejoin="round"
				/>
			</svg>
			設定
		</button>
	</div>
</div>
