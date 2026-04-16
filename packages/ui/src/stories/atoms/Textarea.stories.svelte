<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Textarea from '$lib/atoms/Textarea.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Textarea',
		component: Textarea,
		tags: ['autodocs'],
		argTypes: {
			error: { control: 'boolean' },
			disabled: { control: 'boolean' },
			autoResize: { control: 'boolean' },
			placeholder: { control: 'text' },
			rows: { control: 'number' }
		}
	});
</script>

<Story name="Default" args={{ placeholder: 'Enter system prompt...', rows: 4 }} />

<Story name="Auto Resize" args={{ placeholder: 'Type to grow...', autoResize: true, rows: 2 }} />

<Story name="Error State" args={{ placeholder: 'Required field', error: true, rows: 3 }} />

<Story name="In a Field" asChild>
	<div class="max-w-md p-6">
		<Field label="System Prompt" hint="Defines the AI persona's behaviour and tone">
			<Textarea placeholder="You are Yori, a helpful AI assistant..." rows={5} />
		</Field>
	</div>
</Story>
