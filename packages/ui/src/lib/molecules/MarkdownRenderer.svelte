<script lang="ts">
	import SvelteMarkdown from '@humanspeak/svelte-markdown';

	interface Props {
		source: string;
		class?: string;
	}

	let { source, class: className = '' }: Props = $props();
</script>

<div class="prose prose-edit {className}">
	<SvelteMarkdown {source} />
</div>
