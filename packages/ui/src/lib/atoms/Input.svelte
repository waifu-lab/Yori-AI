<script lang="ts">
	type Size = 'sm' | 'md' | 'lg';

	import type { HTMLInputAttributes } from 'svelte/elements';

	interface Props extends Omit<HTMLInputAttributes, 'size'> {
		size?: Size;
		error?: boolean;
		class?: string;
		value?: string | number;
	}

	let {
		size = 'md',
		error = false,
		class: className = '',
		type = 'text',
		value = $bindable<string | number | undefined>(),
		...rest
	}: Props = $props();

	const base =
		'w-full rounded-md border bg-surface text-foreground placeholder:text-faint ' +
		'shadow-mini-inset transition-all duration-fast font-[Inter] ' +
		'hover:border-border-hover ' +
		'focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 ' +
		'disabled:opacity-40 disabled:cursor-not-allowed';

	const errorClass = 'border-danger focus:border-danger focus:ring-danger/20';

	const sizes: Record<Size, string> = {
		sm: 'h-8 px-3 text-xs',
		md: 'h-9 px-3 text-sm',
		lg: 'h-11 px-4 text-base'
	};
</script>

<input class="{base} {sizes[size]} {error ? errorClass : ''} {className}" {...rest} {type} bind:value />
