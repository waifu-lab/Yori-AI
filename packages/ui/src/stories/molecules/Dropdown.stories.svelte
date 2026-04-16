<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Dropdown from '$lib/molecules/Dropdown.svelte';
	import Button from '$lib/atoms/Button.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Dropdown',
		component: Dropdown,
		tags: ['autodocs']
	});
</script>

<Story name="Default" asChild>
	<div class="flex justify-center p-12">
		<Dropdown
			groups={[
				{
					items: [
						{ label: 'Edit Persona', icon: 'icon-[material-symbols--edit]', shortcut: '⌘E' },
						{ label: 'Duplicate', icon: 'icon-[material-symbols--content-copy]' },
						{ label: 'Share', icon: 'icon-[material-symbols--share]' }
					]
				},
				{
					label: 'Danger Zone',
					items: [{ label: 'Delete', icon: 'icon-[material-symbols--delete]', variant: 'danger' }]
				}
			]}
		>
			{#snippet trigger()}
				<Button variant="ghost">Options ▾</Button>
			{/snippet}
		</Dropdown>
	</div>
</Story>

<Story name="Icon Trigger" asChild>
	<div class="flex justify-center p-12">
		<Dropdown
			align="end"
			groups={[
				{
					label: 'Model',
					items: [
						{ label: 'Claude Sonnet 4.6', icon: 'icon-[material-symbols--smart-toy]' },
						{ label: 'GPT-4o', icon: 'icon-[material-symbols--smart-toy]' },
						{ label: 'Gemini Pro', icon: 'icon-[material-symbols--smart-toy]' }
					]
				},
				{
					items: [{ label: 'Settings', icon: 'icon-[material-symbols--settings]', shortcut: '⌘,' }]
				}
			]}
		>
			{#snippet trigger()}
				<Button variant="ghost" class="h-8 w-8 p-0">⋯</Button>
			{/snippet}
		</Dropdown>
	</div>
</Story>
