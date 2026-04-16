<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Field from '$lib/atoms/Field.svelte';
	import Input from '$lib/atoms/Input.svelte';
	import Textarea from '$lib/atoms/Textarea.svelte';
	import Select from '$lib/atoms/Select.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Field',
		component: Field,
		tags: ['autodocs'],
		argTypes: {
			label: { control: 'text' },
			hint: { control: 'text' },
			error: { control: 'text' },
			required: { control: 'boolean' }
		}
	});

	const langOptions = [
		{ value: 'en', label: 'English' },
		{ value: 'zh', label: '繁體中文' },
		{ value: 'ja', label: '日本語' }
	];
</script>

<Story name="With Hint" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="Display Name" hint="This is shown in conversations">
			{#snippet children()}
				<Input placeholder="e.g. Rex Wu" />
			{/snippet}
		</Field>
	</div>
</Story>

<Story name="With Error" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="API Key" error="Invalid key format — must start with sk-ant-">
			{#snippet children()}
				<Input placeholder="sk-ant-..." value="bad-key" />
			{/snippet}
		</Field>
	</div>
</Story>

<Story name="Required" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="Persona Name" hint="Give your AI a memorable name" required>
			{#snippet children()}
				<Input placeholder="e.g. Yori" />
			{/snippet}
		</Field>
	</div>
</Story>

<Story name="With Select" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="Language" hint="Interface language">
			{#snippet children()}
				<Select options={langOptions} value="en" />
			{/snippet}
		</Field>
	</div>
</Story>

<Story name="With Textarea" asChild>
	<div class="flex max-w-sm flex-col gap-4 p-6">
		<Field label="System Prompt" hint="Instructions that define your persona's behaviour">
			{#snippet children()}
				<Textarea placeholder="You are a helpful assistant..." rows={4} />
			{/snippet}
		</Field>
	</div>
</Story>

<Story name="Form Group" asChild>
	<div class="flex max-w-sm flex-col gap-5 p-6">
		<Field label="Persona Name" required>
			{#snippet children()}
				<Input placeholder="e.g. Yori" />
			{/snippet}
		</Field>
		<Field label="Model Provider" hint="Choose your AI backend">
			{#snippet children()}
				<Select options={langOptions} placeholder="Select provider..." />
			{/snippet}
		</Field>
		<Field label="API Key" error="This field is required">
			{#snippet children()}
				<Input placeholder="sk-ant-..." type="password" />
			{/snippet}
		</Field>
	</div>
</Story>
