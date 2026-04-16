<script lang="ts">
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	interface Props extends HTMLTextareaAttributes {
		error?: boolean;
		autoResize?: boolean;
		class?: string;
	}

	let { error = false, autoResize = false, class: className = '', value = $bindable(''), ...rest }: Props = $props();

	let el: HTMLTextAreaElement | undefined = $state();

	$effect(() => {
		if (autoResize && el) {
			el.style.height = 'auto';
			el.style.height = `${el.scrollHeight}px`;
		}
	});

	const base =
		'w-full rounded-md border bg-surface text-foreground placeholder:text-faint ' +
		'px-3 py-2.5 text-sm leading-relaxed font-[Inter] ' +
		'shadow-mini-inset transition-all duration-fast ' +
		'hover:border-border-hover ' +
		'focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 ' +
		'disabled:opacity-40 disabled:cursor-not-allowed resize-none';

	const errorClass = 'border-danger focus:border-danger focus:ring-danger/20';
</script>

<textarea bind:this={el} bind:value class="{base} {error ? errorClass : ''} {className}" {...rest}></textarea>
