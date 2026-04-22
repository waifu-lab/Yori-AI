<script lang="ts">
	import type { LayoutProps } from './$types';

	import { AppShell, Sidebar, SettingsLayout } from '@waifu/ui';

	const settingsNav = [
		{ href: '/settings', label: '一般', icon: 'icon-[material-symbols--settings-outline-rounded]' },
		{ href: '/settings/ai', label: 'AI 模型', icon: 'icon-[material-symbols--smart-toy-outline-rounded]' },
		{ href: '/settings/voice', label: '語音', icon: 'icon-[material-symbols--mic-outline-rounded]' },
		{ href: '/settings/appearance', label: '外觀', icon: 'icon-[material-symbols--palette-outline]' },
		{ href: '/settings/about', label: '關於', icon: 'icon-[material-symbols--info-outline-rounded]' }
	];

	import { page } from '$app/state';
	let { data, children }: LayoutProps = $props();

	const currentNav = $derived(settingsNav.find((n) => n.href === page.url.pathname) ?? settingsNav[0]);
</script>

<AppShell.Root>
	<AppShell.Sidebar class="w-[200px]">
		<Sidebar items={settingsNav} currentPath={page.url.pathname} />
	</AppShell.Sidebar>

	<AppShell.Content class="overflow-y-auto">
		<SettingsLayout.Root title={currentNav.label} icon={currentNav.icon}>
			{@render children()}
		</SettingsLayout.Root>
	</AppShell.Content>
</AppShell.Root>
