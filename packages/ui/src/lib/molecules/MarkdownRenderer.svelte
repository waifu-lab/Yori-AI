<script lang="ts">
	import SvelteMarkdown from '@humanspeak/svelte-markdown';
	import type { LinkSnippetProps } from '@humanspeak/svelte-markdown';

	interface Props {
		source: string;
		class?: string;
	}

	let { source, class: className = '' }: Props = $props();
</script>

<div class="prose prose-edit dark:prose-invert {className}">
	<SvelteMarkdown {source}>
		{#snippet link({ href, title, children }: LinkSnippetProps)}
			<a {href} {title} target="_blank" rel="noopener noreferrer">{@render children?.()}</a>
		{/snippet}
	</SvelteMarkdown>
</div>
