<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Checkbox from '$lib/atoms/Checkbox.svelte';
	import CheckboxGroup from '$lib/atoms/CheckboxGroup.svelte';

	const { Story } = defineMeta({
		title: 'Atoms/Checkbox',
		component: Checkbox,
		tags: ['autodocs'],
		argTypes: {
			checked: { control: 'boolean' },
			disabled: { control: 'boolean' },
			label: { control: 'text' }
		}
	});
</script>

<Story name="Unchecked" args={{ label: 'Enable notifications', checked: false }} />

<Story name="Checked" args={{ label: 'Enable notifications', checked: true }} />

<Story name="Indeterminate" args={{ label: 'Select all', indeterminate: true }} />

<Story name="Disabled" args={{ label: 'Disabled option', disabled: true }} />

<Story name="CheckboxGroup — Vertical" asChild>
	<div class="p-6">
		<CheckboxGroup
			options={[
				{ value: 'chat', label: 'Chat Interface' },
				{ value: 'voice', label: 'Voice Mode' },
				{ value: 'vision', label: 'Vision (image input)' },
				{ value: 'tools', label: 'Tool Use', disabled: true }
			]}
			value={['chat', 'vision']}
		/>
	</div>
</Story>

<Story name="CheckboxGroup — Horizontal" asChild>
	<div class="p-6">
		<CheckboxGroup
			orientation="horizontal"
			options={[
				{ value: 'en', label: 'English' },
				{ value: 'zh', label: 'Chinese' },
				{ value: 'ja', label: 'Japanese' }
			]}
			value={['en']}
		/>
	</div>
</Story>
