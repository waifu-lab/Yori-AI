<script lang="ts">
	import { Select } from 'bits-ui';

	interface Option {
		value: string;
		label: string;
		disabled?: boolean;
	}

	interface Props {
		options: Option[];
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		error?: boolean;
		size?: 'sm' | 'md' | 'lg';
		class?: string;
		onValueChange?: (value: string) => void;
	}

	let {
		options,
		value = $bindable(''),
		placeholder = 'Select...',
		disabled = false,
		error = false,
		size = 'md',
		class: className = '',
		onValueChange
	}: Props = $props();

	const selected = $derived(options.find((o) => o.value === value));

	const sizes: Record<string, string> = {
		sm: 'h-8 px-3 text-xs',
		md: 'h-9 px-3 text-sm',
		lg: 'h-11 px-4 text-base'
	};

	const triggerBase =
		'flex w-full items-center justify-between rounded-md border bg-surface text-foreground font-[Inter] ' +
		'shadow-mini-inset transition-all duration-fast cursor-pointer ' +
		'hover:border-border-hover ' +
		'focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 ' +
		'disabled:opacity-40 disabled:cursor-not-allowed';

	const errorClass = 'border-danger focus:border-danger focus:ring-danger/20';
</script>

<Select.Root
	type="single"
	{value}
	onValueChange={(v) => {
		value = v;
		onValueChange?.(v);
	}}
	items={options}
	{disabled}
>
	<Select.Trigger class="{triggerBase} {sizes[size]} {error ? errorClass : ''} {className}">
		<span class={selected ? 'text-foreground' : 'text-faint'}>
			{selected?.label ?? placeholder}
		</span>
		<svg class="text-muted-foreground h-4 w-4 shrink-0" viewBox="0 0 16 16" fill="none">
			<path
				d="M4 6L8 10L12 6"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
			/>
		</svg>
	</Select.Trigger>

	<Select.Portal>
		<Select.Content
			class="bg-surface/90 shadow-float data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0
        data-[state=open]:zoom-in-95 data-[state=closed]:zoom-out-95 z-50
        min-w-[8rem] overflow-hidden
        rounded-lg p-1
        backdrop-blur-[20px] outline-none"
			sideOffset={6}
		>
			<Select.Viewport>
				{#each options as option (option.value)}
					<Select.Item
						value={option.value}
						label={option.label}
						disabled={option.disabled}
						class="text-muted-foreground hover:bg-elevated hover:text-secondary data-[highlighted]:bg-elevated data-[highlighted]:text-secondary data-[selected]:text-primary duration-fast relative flex
              w-full cursor-pointer items-center
              rounded-sm px-3
              py-2 text-sm
              transition-colors
              outline-none select-none
              data-[disabled]:pointer-events-none data-[disabled]:opacity-40"
					>
						{#snippet children({ selected: isSelected })}
							{option.label}
							{#if isSelected}
								<svg class="text-primary ml-auto h-3 w-3" viewBox="0 0 12 10" fill="none">
									<path
										d="M1 5L4.5 8.5L11 1"
										stroke="currentColor"
										stroke-width="1.5"
										stroke-linecap="round"
										stroke-linejoin="round"
									/>
								</svg>
							{/if}
						{/snippet}
					</Select.Item>
				{/each}
			</Select.Viewport>
		</Select.Content>
	</Select.Portal>
</Select.Root>
