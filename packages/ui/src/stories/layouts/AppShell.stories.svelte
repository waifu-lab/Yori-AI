<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import AppShell from '$lib/layouts/AppShell.svelte';
	import Sidebar from '$lib/molecules/Sidebar.svelte';

	const { Story } = defineMeta({
		title: 'Layouts/AppShell',
		component: AppShell,
		tags: ['autodocs']
	});

	const navItems = [
		{ href: '/project', label: 'Projects', icon: 'icon-[material-symbols--folder]' },
		{ href: '/persona', label: 'Persona & Soul', icon: 'icon-[material-symbols--auto-awesome]' },
		{ href: '/mixer', label: 'Channel Mixer', icon: 'icon-[material-symbols--settings-input-component]' },
		{ href: '/plugin', label: 'Plugin Store', icon: 'icon-[material-symbols--extension]' }
	];
</script>

<Story name="Default" asChild>
	<div class="h-[600px] w-full">
		<AppShell>
			{#snippet sidebar()}
				<Sidebar items={navItems} currentPath="/project">
					{#snippet header()}
						<div class="flex items-center gap-3 px-1 py-2">
							<div
								class="bg-primary flex h-8 w-8 items-center justify-center rounded-md font-extrabold text-white shadow-lg"
							>
								Y
							</div>
							<span class="text-foreground text-lg font-extrabold tracking-tight">Yori AI</span>
						</div>
					{/snippet}
					{#snippet footer()}
						<div
							class="hover:bg-elevated mb-2 flex cursor-pointer items-center gap-3 rounded-md px-2 py-2 transition-colors"
						>
							<div
								class="bg-elevated text-muted-foreground flex h-8 w-8 items-center justify-center rounded-full text-xs font-bold"
							>
								RW
							</div>
							<div class="min-w-0 flex-1">
								<p class="text-foreground truncate text-sm font-semibold">Rex Wu</p>
								<p class="text-muted-foreground truncate text-xs">Pro Plan</p>
							</div>
						</div>
					{/snippet}
				</Sidebar>
			{/snippet}
			{#snippet children()}
				<div class="flex-1 overflow-y-auto p-8">
					<h1 class="text-foreground mb-2 text-[32px] font-extrabold tracking-[-0.05em]">Projects</h1>
					<p class="text-muted-foreground text-sm">Manage your AI personas and projects here.</p>
					<div class="mt-8 grid grid-cols-2 gap-4">
						{#each ['Yori Assistant', 'Code Helper', 'Creative Writer'] as name}
							<div class="bg-surface rounded-md p-5">
								<p class="text-foreground font-semibold">{name}</p>
								<p class="text-muted-foreground mt-1 text-xs">Last active 2h ago</p>
							</div>
						{/each}
					</div>
				</div>
			{/snippet}
		</AppShell>
	</div>
</Story>
