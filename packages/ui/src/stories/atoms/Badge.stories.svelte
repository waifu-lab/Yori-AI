<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Badge from '$lib/atoms/Badge.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Badge',
		component: Badge,
		tags: ['autodocs'],
		argTypes: {
			variant: {
				control: 'select',
				options: ['default', 'primary', 'danger', 'muted']
			}
		}
	});
</script>

<Story name="All Variants" asChild>
	<div class="flex flex-wrap gap-3 p-6">
		<Badge variant="default">Default</Badge>
		<Badge variant="primary">Primary</Badge>
		<Badge variant="danger">Error</Badge>
		<Badge variant="muted">Deprecated</Badge>
	</div>
</Story>

<Story name="In Context" asChild>
	<div class="flex flex-col gap-3 p-6">
		<div class="text-foreground flex items-center gap-2 text-sm">
			Claude Sonnet 4.6 <Badge variant="primary">Pro</Badge>
		</div>
		<div class="text-foreground flex items-center gap-2 text-sm">
			GPT-4o Mini <Badge variant="default">Online</Badge>
		</div>
		<div class="text-foreground flex items-center gap-2 text-sm">
			Gemini 1.5 Pro <Badge variant="danger">Unavailable</Badge>
		</div>
		<div class="text-foreground flex items-center gap-2 text-sm">
			LLaMA 3 <Badge variant="muted">Local</Badge>
		</div>
	</div>
</Story>
