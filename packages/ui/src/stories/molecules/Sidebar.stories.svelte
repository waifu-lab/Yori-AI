<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import Sidebar from '$lib/molecules/Sidebar.svelte';

	const { Story } = defineMeta({
		title: 'Molecules/Sidebar',
		component: Sidebar,
		tags: ['autodocs']
	});
</script>

<script lang="ts">
	const navItems = [
		{
			href: '/project',
			label: 'Projects',
			icon: 'icon-[material-symbols--folder]'
		},
		{
			href: '/persona',
			label: 'Persona & Soul',
			icon: 'icon-[material-symbols--auto-awesome]',
			children: [
				{ href: '/persona/character', label: 'Character', icon: 'icon-[material-symbols--face]' },
				{ href: '/persona/memory', label: 'Memory', icon: 'icon-[material-symbols--memory]' },
				{ href: '/persona/voice', label: 'Voice', icon: 'icon-[material-symbols--mic]' }
			]
		},
		{
			href: '/mixer',
			label: 'Channel Mixer',
			icon: 'icon-[material-symbols--settings-input-component]'
		},
		{
			href: '/plugin',
			label: 'Plugin Store',
			icon: 'icon-[material-symbols--extension]'
		}
	];
</script>

<Story name="Default" asChild>
	<div class="bg-surface h-screen w-[260px]">
		<Sidebar items={navItems} currentPath="/project" />
	</div>
</Story>

<Story name="With Active Nested" asChild>
	<div class="bg-surface h-screen w-[260px]">
		<Sidebar items={navItems} currentPath="/persona/memory" />
	</div>
</Story>

<Story name="With Header and Footer" asChild>
	<div class="bg-surface h-screen w-[260px]">
		<Sidebar items={navItems} currentPath="/mixer">
			{#snippet header()}
				<div class="flex items-center gap-3 px-1 py-2">
					<div
						class="bg-primary flex h-8 w-8 items-center justify-center rounded-md text-xs font-bold text-white shadow-lg"
					>
						Y
					</div>
					<span class="text-foreground text-lg font-extrabold tracking-tight">Yori AI</span>
				</div>
			{/snippet}
			{#snippet footer()}
				<div class="hover:bg-elevated flex cursor-pointer items-center gap-3 rounded-md px-2 py-2 transition-colors">
					<div
						class="bg-elevated text-muted-foreground flex h-8 w-8 items-center justify-center rounded-full text-xs font-bold"
					>
						RW
					</div>
					<div class="min-w-0 flex-1">
						<p class="text-foreground truncate text-sm font-semibold">name</p>
						<p class="text-muted-foreground truncate text-xs">Pro Plan</p>
					</div>
				</div>
			{/snippet}
		</Sidebar>
	</div>
</Story>
