<script lang="ts">
	import SvelteMarkdown from '@humanspeak/svelte-markdown';

	interface Props {
		source: string;
		class?: string;
	}

	let { source, class: className = '' }: Props = $props();
</script>

<!--
  MarkdownRenderer — wraps @humanspeak/svelte-markdown
  All typography tokens applied via .prose-etheric
-->
<div class="prose-etheric {className}">
	<SvelteMarkdown {source} />
</div>

<style>
	/* Etheric typography — Manrope scale, No-Line philosophy */
	.prose-etheric :global(h1) {
		font-size: 32px;
		font-weight: 800;
		letter-spacing: -0.05em;
		color: var(--color-text);
		line-height: 1.1;
		margin: 1.5rem 0 0.75rem;
	}

	.prose-etheric :global(h2) {
		font-size: 20px;
		font-weight: 700;
		color: var(--color-text);
		line-height: 1.3;
		margin: 1.25rem 0 0.5rem;
	}

	.prose-etheric :global(h3) {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text);
		line-height: 1.4;
		margin: 1rem 0 0.5rem;
	}

	.prose-etheric :global(p) {
		font-size: 14px;
		font-weight: 500;
		line-height: 1.6;
		color: var(--color-text);
		margin: 0 0 0.75rem;
	}

	.prose-etheric :global(strong) {
		font-weight: 700;
		color: var(--color-text);
	}

	.prose-etheric :global(em) {
		font-style: italic;
		color: var(--color-text-muted);
	}

	.prose-etheric :global(a) {
		color: var(--color-primary);
		text-decoration: none;
		font-weight: 600;
	}

	.prose-etheric :global(a:hover) {
		text-decoration: underline;
	}

	/* Code — inline */
	.prose-etheric :global(code) {
		font-size: 13px;
		font-family: ui-monospace, 'Cascadia Code', monospace;
		background: var(--color-elevated);
		color: var(--color-text);
		padding: 0.1em 0.4em;
		border-radius: var(--radius-sm);
	}

	/* Code — block, No-Line */
	.prose-etheric :global(pre) {
		background: var(--color-surface);
		border-radius: var(--radius);
		padding: 1rem;
		overflow-x: auto;
		margin: 0.75rem 0;
	}

	.prose-etheric :global(pre code) {
		background: transparent;
		padding: 0;
		font-size: 13px;
		line-height: 1.6;
	}

	/* Blockquote — left indicator bar */
	.prose-etheric :global(blockquote) {
		border-left: 2px solid var(--color-primary);
		padding-left: 1rem;
		margin: 0.75rem 0;
		color: var(--color-text-muted);
		font-style: normal;
	}

	/* Lists */
	.prose-etheric :global(ul),
	.prose-etheric :global(ol) {
		font-size: 14px;
		font-weight: 500;
		line-height: 1.6;
		color: var(--color-text);
		padding-left: 1.5rem;
		margin: 0.5rem 0 0.75rem;
	}

	.prose-etheric :global(li) {
		margin-bottom: 0.25rem;
	}

	/* Table */
	.prose-etheric :global(table) {
		width: 100%;
		font-size: 14px;
		border-collapse: collapse;
		margin: 0.75rem 0;
	}

	.prose-etheric :global(th) {
		text-align: left;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--color-text-muted);
		padding: 0.5rem 0.75rem;
		background: var(--color-elevated);
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
	}

	.prose-etheric :global(td) {
		padding: 0.5rem 0.75rem;
		color: var(--color-text);
		background: var(--color-surface);
	}

	/* Horizontal rule */
	.prose-etheric :global(hr) {
		border: none;
		height: 1px;
		background: var(--color-elevated);
		margin: 1.5rem 0;
	}
</style>
