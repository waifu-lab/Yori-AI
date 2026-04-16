<script lang="ts">
	import { Button as BitsButton } from 'bits-ui';
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	type Variant = 'primary' | 'secondary' | 'tertiary' | 'ghost' | 'danger';
	type Size = 'sm' | 'md' | 'lg';

	interface Props extends HTMLButtonAttributes {
		variant?: Variant;
		size?: Size;
		loading?: boolean;
		children: Snippet;
	}

	let {
		variant = 'primary',
		size = 'md',
		loading = false,
		disabled = false,
		class: className = '',
		children,
		...rest
	}: Props = $props();

	const base =
		'inline-flex items-center justify-center gap-2 font-[Inter] font-semibold rounded-md ' +
		'transition-all duration-fast select-none cursor-pointer ' +
		'active:scale-[0.97] ' +
		'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-canvas ' +
		'disabled:opacity-40 disabled:pointer-events-none disabled:active:scale-100';

	const variants: Record<Variant, string> = {
		primary:
			'[background:linear-gradient(145deg,var(--color-primary),color-mix(in_srgb,var(--color-primary)_80%,#000))] ' +
			'text-white shadow-mini ' +
			'hover:brightness-110',

		secondary: 'bg-transparent border text-foreground ' + 'hover:bg-elevated',

		tertiary: 'bg-transparent text-foreground ' + 'hover:text-secondary',

		ghost: 'bg-transparent text-muted-foreground ' + 'hover:bg-elevated hover:text-foreground',

		danger: 'bg-danger-surface text-danger ' + 'hover:bg-danger hover:text-white'
	};

	const sizes: Record<Size, string> = {
		sm: 'h-8 px-3 text-xs',
		md: 'h-9 px-4 text-sm',
		lg: 'h-11 px-6 text-base'
	};
</script>

<BitsButton.Root
	class="{base} {variants[variant]} {sizes[size]} {className}"
	disabled={disabled || loading}
	{...rest as any}
>
	{#if loading}
		<span class="h-4 w-4 shrink-0 animate-spin rounded-full border-2 border-current border-t-transparent"></span>
	{/if}
	{@render children()}
</BitsButton.Root>
