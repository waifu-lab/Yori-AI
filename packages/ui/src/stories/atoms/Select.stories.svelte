<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Select from '$lib/atoms/Select.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Select',
		component: Select,
		tags: ['autodocs'],
		argTypes: {
			size: { control: 'select', options: ['sm', 'md', 'lg'] },
			error: { control: 'boolean' },
			disabled: { control: 'boolean' },
			placeholder: { control: 'text' }
		}
	});
</script>

<script lang="ts">
	const modelOptions = [
		{ value: 'gpt-4o', label: 'GPT-4o' },
		{ value: 'gpt-4o-mini', label: 'GPT-4o Mini' },
		{ value: 'claude-sonnet', label: 'Claude Sonnet 4.6' },
		{ value: 'claude-haiku', label: 'Claude Haiku 4.5' },
		{ value: 'gemini-pro', label: 'Gemini 1.5 Pro', disabled: true }
	];
</script>

<Story name="Default" asChild>
	<div class="max-w-xs p-6">
		<Select options={modelOptions} placeholder="Select a model..." />
	</div>
</Story>

<Story name="With Selection" asChild>
	<div class="max-w-xs p-6">
		<Select options={modelOptions} value="claude-sonnet" />
	</div>
</Story>

<Story name="Error State" asChild>
	<div class="max-w-xs p-6">
		<Select options={modelOptions} placeholder="Select a model..." error />
	</div>
</Story>

<Story name="In a Field" asChild>
	<div class="max-w-xs p-6">
		<Field label="AI Model" hint="Choose the model for this persona">
			<Select options={modelOptions} placeholder="Select a model..." />
		</Field>
	</div>
</Story>

<Story name="Sizes" asChild>
	<div class="flex max-w-xs flex-col gap-3 p-6">
		<Select options={modelOptions} size="sm" placeholder="Small" />
		<Select options={modelOptions} size="md" placeholder="Medium" />
		<Select options={modelOptions} size="lg" placeholder="Large" />
	</div>
</Story>
