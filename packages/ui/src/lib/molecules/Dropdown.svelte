<script lang="ts">
	import { DropdownMenu } from 'bits-ui';
	import type { Snippet } from 'svelte';

	interface DropdownItem {
		label: string;
		icon?: string;
		shortcut?: string;
		disabled?: boolean;
		variant?: 'default' | 'danger';
		onclick?: () => void;
	}

	interface DropdownGroup {
		label?: string;
		items: DropdownItem[];
	}

	interface Props {
		groups: DropdownGroup[];
		trigger: Snippet;
		align?: 'start' | 'center' | 'end';
		side?: 'top' | 'bottom' | 'left' | 'right';
		class?: string;
	}

	let { groups, trigger, align = 'start', side = 'bottom', class: className = '' }: Props = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class={className}>
		{@render trigger()}
	</DropdownMenu.Trigger>

	<DropdownMenu.Portal>
		<DropdownMenu.Content
			{align}
			{side}
			sideOffset={6}
			class="bg-surface/90 shadow-float data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0
        data-[state=open]:zoom-in-95 data-[state=closed]:zoom-out-95 z-50
        min-w-[180px] overflow-hidden
        rounded-lg p-1
        backdrop-blur-[20px] outline-none"
		>
			{#each groups as group, i (i)}
				<DropdownMenu.Group>
					{#if group.label}
						<DropdownMenu.GroupHeading
							class="text-faint px-3 py-1.5 font-[Inter] text-xs font-semibold tracking-widest uppercase"
						>
							{group.label}
						</DropdownMenu.GroupHeading>
					{/if}

					{#each group.items as item (item.label)}
						<DropdownMenu.Item
							disabled={item.disabled}
							onclick={item.onclick}
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
						</DropdownMenu.Item>
					{/each}
				</DropdownMenu.Group>

				{#if i < groups.length - 1}
					<div class="my-1.5"></div>
				{/if}
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Portal>
</DropdownMenu.Root>
