<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import ContextMenu from '$lib/molecules/ContextMenu.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/ContextMenu',
		component: ContextMenu,
		tags: ['autodocs']
	});
</script>

<Story name="Default" asChild>
	<div class="flex justify-center p-12">
		<ContextMenu
			groups={[
				{
					items: [
						{ label: 'Copy Message', icon: 'icon-[material-symbols--content-copy]', shortcut: '⌘C' },
						{ label: 'Regenerate', icon: 'icon-[material-symbols--refresh]', shortcut: '⌘R' },
						{ label: 'Edit', icon: 'icon-[material-symbols--edit]' }
					]
				},
				{
					items: [{ label: 'Delete Message', variant: 'danger' }]
				}
			]}
		>
			<div class="bg-surface text-muted-foreground cursor-context-menu rounded-md px-5 py-4 text-sm select-none">
				Right-click me to open context menu
			</div>
		</ContextMenu>
	</div>
</Story>
