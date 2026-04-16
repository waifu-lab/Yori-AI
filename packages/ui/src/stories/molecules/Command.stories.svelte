<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Command from '$lib/molecules/Command.svelte';
	import Button from '$lib/atoms/Button.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Command',
		component: Command,
		tags: ['autodocs']
	});
</script>

<script lang="ts">
	const commandItems = [
		{
			id: 'new-chat',
			label: 'New Chat',
			description: 'Start a conversation',
			icon: 'icon-[material-symbols--add]',
			group: 'Actions',
			shortcut: '⌘N'
		},
		{
			id: 'new-persona',
			label: 'New Persona',
			description: 'Create AI persona',
			icon: 'icon-[material-symbols--auto-awesome]',
			group: 'Actions'
		},
		{
			id: 'settings',
			label: 'Open Settings',
			icon: 'icon-[material-symbols--settings]',
			group: 'Navigation',
			shortcut: '⌘,'
		},
		{ id: 'plugins', label: 'Plugin Store', icon: 'icon-[material-symbols--extension]', group: 'Navigation' },
		{ id: 'docs', label: 'Documentation', icon: 'icon-[material-symbols--menu-book]', group: 'Help' }
	];
</script>

<Story name="Default" asChild>
	<div class="flex flex-col items-start gap-4 p-6">
		<p class="text-muted-foreground text-sm">
			Press <kbd class="bg-elevated rounded px-1.5 py-0.5 text-xs">Ctrl+K</kbd> to open
		</p>
		<Command items={commandItems} open />
	</div>
</Story>
