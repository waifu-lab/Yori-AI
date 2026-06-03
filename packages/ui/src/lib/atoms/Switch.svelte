<script lang="ts">
	import { Switch } from 'bits-ui';
	import type { Snippet } from 'svelte';

	interface Props {
		checked?: boolean;
		disabled?: boolean;
		label?: string;
		id?: string;
		class?: string;
		children?: Snippet;
		onCheckedChange?: (checked: boolean) => void;
	}

	let {
		checked = $bindable(false),
		disabled = false,
		label,
		id,
		class: className = '',
		children,
		onCheckedChange
	}: Props = $props();
</script>

<div class="inline-flex items-center gap-2.5 {className}">
	<Switch.Root
		bind:checked
		{disabled}
		{id}
		{onCheckedChange}
		class="bg-surface shadow-mini-inset duration-fast hover:border-border-hover data-[state=checked]:border-primary data-[state=checked]:bg-primary
			focus-visible:ring-ring/40 focus-visible:ring-offset-canvas relative inline-flex h-5 w-9 shrink-0 cursor-pointer
			items-center rounded-full border p-0.5 transition-all focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none
			disabled:pointer-events-none disabled:opacity-40"
	>
		<Switch.Thumb
			class="bg-foreground/70 data-[state=checked]:bg-white block size-4 rounded-full shadow-sm transition-transform duration-fast data-[state=checked]:translate-x-4"
		/>
	</Switch.Root>

	{#if label || children}
		<label
			for={id}
			class="text-foreground font-[Inter] text-sm leading-none select-none"
			class:cursor-pointer={!disabled}
			class:opacity-40={disabled}
			class:cursor-not-allowed={disabled}
		>
			{#if children}
				{@render children()}
			{:else}
				{label}
			{/if}
		</label>
	{/if}
</div>
