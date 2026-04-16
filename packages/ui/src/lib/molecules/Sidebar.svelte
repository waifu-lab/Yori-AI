<script lang="ts">
	import type { Snippet } from 'svelte';

	interface NavItem {
		href: string;
		label: string;
		icon: string;
		children?: NavItem[];
	}

	interface Props {
		items: NavItem[];
		currentPath?: string;
		header?: Snippet;
		footer?: Snippet;
		width?: string;
		class?: string;
	}

	let { items, currentPath = '', header, footer, width = 'w-[260px]', class: className = '' }: Props = $props();

	let expanded = $state<Record<string, boolean>>({});

	function toggleExpanded(href: string) {
		expanded[href] = !expanded[href];
	}

	function isActive(href: string) {
		return currentPath === href || currentPath.startsWith(href + '/');
	}
</script>

<nav class="flex flex-col {width} h-full py-4 {className}">
	{#if header}
		<div class="mb-4 px-4">
			{@render header()}
		</div>
	{/if}

	<ul class="flex flex-1 flex-col gap-0.5 overflow-y-auto px-3">
		{#each items as item (item.href)}
			<li>
				{#if item.children?.length}
					<button
						type="button"
						onclick={() => toggleExpanded(item.href)}
						class="group duration-fast flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-sm
              font-semibold transition-colors
              {isActive(item.href)
							? 'bg-elevated text-foreground'
							: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
					>
						<span class="{item.icon} h-5 w-5 shrink-0"></span>
						<span class="flex-1 text-left">{item.label}</span>

						<svg
							class="text-faint duration-fast h-4 w-4 transition-transform"
							class:rotate-180={expanded[item.href]}
							viewBox="0 0 16 16"
							fill="none"
						>
							<path
								d="M4 6L8 10L12 6"
								stroke="currentColor"
								stroke-width="1.5"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
						</svg>
					</button>

					{#if expanded[item.href]}
						<ul class="border-elevated mt-0.5 ml-4 flex flex-col gap-0.5 border-l-2 pl-3">
							{#each item.children as child (child.href)}
								<li>
									<a
										href={child.href}
										class="duration-fast relative flex items-center gap-2.5 rounded-sm px-3 py-2
                      text-sm transition-colors
                      {isActive(child.href)
											? 'bg-elevated text-foreground font-medium'
											: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
									>
										<span class="{child.icon} h-4 w-4 shrink-0"></span>
										{child.label}
									</a>
								</li>
							{/each}
						</ul>
					{/if}
				{:else}
					<a
						href={item.href}
						class="duration-fast flex items-center gap-3 rounded-md px-3 py-2.5 text-sm
              font-medium transition-colors
              {isActive(item.href)
							? 'bg-elevated text-foreground'
							: 'text-muted-foreground hover:bg-elevated hover:text-foreground'}"
					>
						<span class="{item.icon} h-5 w-5 shrink-0"></span>
						{item.label}
					</a>
				{/if}
			</li>
		{/each}
	</ul>

	{#if footer}
		<div class="mt-auto px-3 pt-4">
			{@render footer()}
		</div>
	{/if}
</nav>
