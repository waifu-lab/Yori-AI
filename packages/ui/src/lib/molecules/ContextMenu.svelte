<script lang="ts">
	import { ContextMenu } from 'bits-ui';
	import type { Snippet } from 'svelte';

	interface MenuItem {
		label: string;
		icon?: string;
		shortcut?: string;
		disabled?: boolean;
		variant?: 'default' | 'danger';
		onSelect?: (event: Event) => void;
		onclick?: () => void;
	}

	interface MenuGroup {
		label?: string;
		items: MenuItem[];
	}

	interface Props {
		groups: MenuGroup[];
		children: Snippet;
		class?: string;
	}

	let { groups, children, class: className = '' }: Props = $props();

	function handleItemSelect(item: MenuItem, event: Event) {
		item.onSelect?.(event);
		if (!event.defaultPrevented) item.onclick?.();
	}
</script>

<ContextMenu.Root>
	<ContextMenu.Trigger class={className}>
		{@render children()}
	</ContextMenu.Trigger>

	<ContextMenu.Portal>
		<ContextMenu.Content
			class="bg-surface/90 shadow-float data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0
        data-[state=open]:zoom-in-95 data-[state=closed]:zoom-out-95 z-50
        min-w-[180px] overflow-hidden
        rounded-lg p-1
        backdrop-blur-[20px] outline-none"
		>
			{#each groups as group, i (i)}
				<ContextMenu.Group>
					{#if group.label}
						<ContextMenu.GroupHeading
							class="text-faint px-3 py-1.5 font-[Inter] text-xs font-semibold tracking-widest uppercase"
						>
							{group.label}
						</ContextMenu.GroupHeading>
					{/if}

					{#each group.items as item, j (j)}
						<ContextMenu.Item
							disabled={item.disabled}
							onSelect={(event) => handleItemSelect(item, event)}
							class="duration-fast flex cursor-pointer items-center gap-2.5 rounded-sm px-3 py-2
                text-sm transition-colors outline-none
                {item.variant === 'danger'
								? 'text-danger hover:bg-danger-surface data-[highlighted]:bg-danger-surface'
								: 'text-muted-foreground hover:bg-elevated hover:text-secondary data-[highlighted]:bg-elevated data-[highlighted]:text-secondary'}
                data-[disabled]:pointer-events-none data-[disabled]:opacity-40"
						>
							{#if item.icon}<span class="{item.icon} h-4 w-4 shrink-0"></span>{/if}
							<span class="flex-1">{item.label}</span>
							{#if item.shortcut}<span class="text-faint text-xs">{item.shortcut}</span>{/if}
						</ContextMenu.Item>
					{/each}
				</ContextMenu.Group>

				{#if i < groups.length - 1}
					<div class="my-1.5"></div>
				{/if}
			{/each}
		</ContextMenu.Content>
	</ContextMenu.Portal>
</ContextMenu.Root>
