<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'default' | 'danger';

	interface Props {
		title: string;
		description?: string;
		icon?: string;
		variant?: Variant;
		class?: string;
		children?: Snippet;
		/** Right-side control slot (toggle, button, select, etc.) */
		control?: Snippet;
	}

	let { title, description, icon, variant = 'default', class: className = '', children, control }: Props = $props();
</script>

<div
	class="bg-surface flex items-start gap-4 rounded-md px-5 py-4
    {variant === 'danger' ? 'bg-danger-surface' : ''}
    {className}"
>
	{#if icon}
		<div
			class="mt-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-md
        {variant === 'danger' ? 'bg-danger-surface text-danger' : 'bg-elevated text-muted-foreground'}"
		>
			<span class="{icon} h-5 w-5"></span>
		</div>
	{/if}

	<div class="min-w-0 flex-1">
		<p
			class="text-sm leading-snug font-semibold
        {variant === 'danger' ? 'text-danger' : 'text-foreground'}"
		>
			{title}
		</p>
		{#if description}
			<p class="text-muted-foreground mt-0.5 text-xs leading-relaxed">{description}</p>
		{/if}
		{#if children}
			<div class="mt-3">
				{@render children()}
			</div>
		{/if}
	</div>

	{#if control}
		<div class="ml-4 flex shrink-0 items-center">
			{@render control()}
		</div>
	{/if}
</div>
