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
		class?: string;
	}

	let { conversations = [], activeId = '', onSelect, class: className = '' }: Props = $props();
</script>

<div class="flex h-full flex-col {className}">
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
</div>
