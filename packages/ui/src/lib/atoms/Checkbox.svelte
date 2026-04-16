<script lang="ts">
	import { Checkbox } from 'bits-ui';
	import type { Snippet } from 'svelte';

	interface Props {
		checked?: boolean;
		indeterminate?: boolean;
		disabled?: boolean;
		label?: string;
		id?: string;
		class?: string;
		children?: Snippet;
		onCheckedChange?: (checked: boolean) => void;
		onIndeterminateChange?: (indeterminate: boolean) => void;
	}

	let {
		checked = $bindable(false),
		indeterminate = $bindable(false),
		disabled = false,
		label,
		id,
		class: className = '',
		children,
		onCheckedChange,
		onIndeterminateChange
	}: Props = $props();
</script>

<div class="inline-flex items-center gap-2.5 {className}">
	<Checkbox.Root
		bind:checked
		bind:indeterminate
		{disabled}
		{id}
		{onCheckedChange}
		{onIndeterminateChange}
		class="bg-surface shadow-mini-inset duration-fast hover:border-border-hover data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=indeterminate]:border-primary data-[state=indeterminate]:bg-primary
      focus-visible:ring-ring/40 focus-visible:ring-offset-canvas relative box-content
      inline-flex size-[18px]
      shrink-0
      cursor-pointer
      items-center justify-center
      rounded-sm border
      transition-all focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none
      disabled:pointer-events-none disabled:opacity-40"
	>
		{#snippet children({ checked, indeterminate })}
			{#if indeterminate}
				<svg width="10" height="2" viewBox="0 0 10 2" fill="none" class="text-white">
					<rect width="10" height="2" rx="1" fill="currentColor" />
				</svg>
			{:else if checked}
				<svg width="11" height="9" viewBox="0 0 11 9" fill="none" class="text-white">
					<path
						d="M1 4.5L4 7.5L10 1"
						stroke="currentColor"
						stroke-width="1.75"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
			{/if}
		{/snippet}
	</Checkbox.Root>

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
