<script lang="ts">
	import { Dialog } from 'bits-ui';

	interface CommandItem {
		id: string;
		label: string;
		description?: string;
		icon?: string;
		shortcut?: string;
		group?: string;
		onclick?: () => void;
	}

	interface Props {
		open?: boolean;
		items?: CommandItem[];
		placeholder?: string;
		onOpenChange?: (open: boolean) => void;
	}

	let { open = $bindable(false), items = [], placeholder = 'Search commands...', onOpenChange }: Props = $props();

	let query = $state('');
	let inputEl: HTMLInputElement | undefined = $state();

	$effect(() => {
		if (open && inputEl) Promise.resolve().then(() => inputEl?.focus());
	});

	const filtered = $derived(
		query.trim()
			? items.filter(
					(item) =>
						item.label.toLowerCase().includes(query.toLowerCase()) ||
						item.description?.toLowerCase().includes(query.toLowerCase())
				)
			: items
	);

	const groups = $derived(
		filtered.reduce<Record<string, CommandItem[]>>((acc, item) => {
			const key = item.group ?? 'Actions';
			if (!acc[key]) acc[key] = [];
			acc[key].push(item);
			return acc;
		}, {})
	);

	let activeIndex = $state(0);

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			activeIndex = Math.min(activeIndex + 1, filtered.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			activeIndex = Math.max(activeIndex - 1, 0);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			filtered[activeIndex]?.onclick?.();
			open = false;
		}
	}

	$effect(() => {
		if (!open) {
			query = '';
			activeIndex = 0;
		}
	});
</script>

<svelte:window
	onkeydown={(e) => {
		if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
			e.preventDefault();
			open = true;
		}
	}}
/>

<Dialog.Root bind:open {onOpenChange}>
	<Dialog.Portal>
		<Dialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0 fixed
        inset-0 z-50
        bg-black/60 backdrop-blur-[24px]"
		/>
		<Dialog.Content
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=open]:fade-in-0 data-[state=closed]:fade-out-0 data-[state=open]:zoom-in-95 data-[state=closed]:zoom-out-95 fixed top-[20%] left-1/2
        z-50 w-full
        max-w-lg -translate-x-1/2
        p-4 focus:outline-none"
		>
			<div class="bg-surface/90 shadow-float overflow-hidden rounded-lg backdrop-blur-[20px]">
				<!-- Search row -->
				<div class="flex items-center gap-3 px-4 py-3">
					<svg
						class="text-muted-foreground h-4 w-4 shrink-0"
						viewBox="0 0 16 16"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<circle cx="7" cy="7" r="4.5" />
						<path d="M10.5 10.5L14 14" stroke-linecap="round" />
					</svg>
					<input
						bind:this={inputEl}
						type="text"
						bind:value={query}
						{placeholder}
						onkeydown={handleKeydown}
						class="text-foreground placeholder:text-faint flex-1 bg-transparent text-sm focus:outline-none"
					/>
					<kbd class="bg-elevated text-faint hidden items-center rounded-sm px-1.5 py-0.5 text-xs sm:inline-flex"
						>Esc</kbd
					>
				</div>

				<div class="max-h-[360px] overflow-y-auto p-2">
					{#if filtered.length === 0}
						<p class="text-muted-foreground py-8 text-center text-sm">No results found.</p>
					{:else}
						{#each Object.entries(groups) as [groupName, groupItems] (groupName)}
							<div class="mb-2">
								<p class="text-faint px-3 py-1 font-[Inter] text-xs font-semibold tracking-widest uppercase">
									{groupName}
								</p>
								{#each groupItems as item (item.id)}
									{@const globalIndex = filtered.indexOf(item)}
									<button
										onclick={() => {
											item.onclick?.();
											open = false;
										}}
										onmouseenter={() => (activeIndex = globalIndex)}
										class="duration-fast flex w-full items-center gap-3 rounded-sm px-3 py-2.5 text-sm transition-colors
                      {activeIndex === globalIndex
											? 'bg-elevated text-foreground'
											: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
									>
										{#if item.icon}<span class="{item.icon} h-4 w-4 shrink-0"></span>{/if}
										<span class="flex-1 text-left">{item.label}</span>
										{#if item.description}
											<span class="text-faint hidden text-xs sm:block">{item.description}</span>
										{/if}
										{#if item.shortcut}
											<kbd class="bg-elevated text-faint rounded-sm px-1.5 py-0.5 text-xs">{item.shortcut}</kbd>
										{/if}
									</button>
								{/each}
							</div>
						{/each}
					{/if}
				</div>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
