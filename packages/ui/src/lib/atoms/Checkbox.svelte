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
				<span class="icon-[material-symbols--check-indeterminate-small-rounded] text-white"></span>
			{:else if checked}
				<span class="icon-[material-symbols--check-rounded] text-white"></span>
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
