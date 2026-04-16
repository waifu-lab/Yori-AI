<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Input from '$lib/atoms/Input.svelte';
	import Field from '$lib/atoms/Field.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Input',
		component: Input,
		tags: ['autodocs'],
		argTypes: {
			size: { control: 'select', options: ['sm', 'md', 'lg'] },
			error: { control: 'boolean' },
			disabled: { control: 'boolean' },
			placeholder: { control: 'text' }
		}
	});
</script>

<Story name="Default" args={{ placeholder: 'Type something...' }} />

<Story name="With Value" args={{ value: 'Yori AI Assistant', placeholder: 'Name' }} />

<Story name="Error State" args={{ placeholder: 'Email address', error: true }} />

<Story name="Disabled" args={{ placeholder: 'Disabled input', disabled: true }} />

<Story name="Sizes" asChild>
	<div class="flex max-w-sm flex-col gap-3 p-6">
		<Input size="sm" placeholder="Small input" />
		<Input size="md" placeholder="Medium input" />
		<Input size="lg" placeholder="Large input" />
	</div>
</Story>

<Story name="In a Field" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="Model Name" hint="Unique identifier for this model">
			<Input placeholder="e.g. gpt-4o-mini" />
		</Field>
		<Field label="API Key" error="This field is required" required>
			<Input placeholder="sk-..." error />
		</Field>
	</div>
</Story>
