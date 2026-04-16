<script lang="ts">
	import type { Snippet } from 'svelte';

	type Variant = 'default' | 'primary' | 'danger' | 'muted';

	interface Props {
		variant?: Variant;
		class?: string;
		children: Snippet;
	}

	let { variant = 'default', class: className = '', children }: Props = $props();

	const variants: Record<Variant, string> = {
		default: 'bg-elevated text-foreground',
		primary: 'bg-primary-bloom text-primary',
		danger: 'bg-danger-surface text-danger',
		muted: 'bg-elevated text-muted-foreground'
	};
</script>

<span
	class="inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 font-[Inter] text-xs font-medium {variants[
		variant
	]} {className}"
>
	{@render children()}
</span>
